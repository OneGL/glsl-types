use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::PathBuf;

use glsl::parser::Parse as _;
use glsl::syntax::ShaderStage;
use glsl::transpiler;

use super::file_manager::FileManager;
use super::fn_calls::rename_imported_function_calls;
use super::fn_definitions::rename_functions_to_avoid_collisions;
use super::fn_name_manager::FunctionNameManager;
use super::graph::Graph;

pub fn resolve_imports(file: &PathBuf) -> String {
  let mut resolver = ImportResolver::new();

  resolver.build_import_graph(file);

  return move_glsl_version_to_top(resolver.combine_files(file, &mut HashSet::new(), true));
}

struct ImportResolver {
  graph: Graph,
  file_manager: FileManager,
  fn_names: FunctionNameManager,
}

impl ImportResolver {
  pub fn new() -> Self {
    Self {
      graph: Graph::new(),
      file_manager: FileManager::new(),
      fn_names: FunctionNameManager::new(),
    }
  }

  fn build_import_graph(&mut self, file_path: &PathBuf) -> &Graph {
    let file_imports = self.file_manager.get_file_imports(&file_path);

    for (_, path) in file_imports {
      self.graph.add_edge(file_path.clone(), path.clone());

      if self.graph.has_cycle() {
        panic!("Cycle detected in the import graph");
      }

      self.build_import_graph(&path);
    }

    return &self.graph;
  }

  fn combine_files(
    &mut self,
    node: &PathBuf,
    visited: &mut HashSet<PathBuf>,
    is_root: bool,
  ) -> String {
    // Here we are reserving the function names of the root file
    // if we do it now, the root file will preserve its function names
    if is_root {
      let file = self.file_manager.get_file(node);
      let mut ast = ShaderStage::parse(&file.contents).unwrap();

      let (fn_names, _) =
        rename_functions_to_avoid_collisions(&mut ast, self.fn_names.clone(), node);

      self.fn_names = fn_names;
    }

    let mut output = String::new();

    if visited.contains(node) {
      return output;
    }

    visited.insert(node.clone());

    let neighbors = self
      .graph
      .get_neighbors(node)
      .cloned()
      .unwrap_or_else(Vec::new);

    for neighbor in neighbors {
      output += &self.combine_files(&neighbor, visited, false);
    }

    output += &self.get_file_content(node, is_root);

    return output;
  }

  fn get_file_content(&mut self, file_path: &PathBuf, is_root: bool) -> String {
    let file = self.file_manager.get_file(file_path);
    let mut ast = ShaderStage::parse(&file.contents).unwrap();

    let fn_names = self.fn_names.clone();

    let (fn_names, fn_definitions) =
      rename_functions_to_avoid_collisions(&mut ast, fn_names, file_path);

    let fn_names =
      rename_imported_function_calls(&mut ast, fn_names, file.imports, file_path.clone());

    self.fn_names = fn_names;

    // Transpile the shader AST back to GLSL
    let mut output = String::new();

    output += &format!(
      "\n\n// File: {}\n\n",
      file_path
        .file_name()
        .unwrap_or(OsStr::new("unknown"))
        .to_str()
        .unwrap_or("unknown")
    );

    if is_root {
      transpiler::glsl::show_translation_unit(&mut output, &ast);
      return output;
    }

    for fn_definition in fn_definitions {
      transpiler::glsl::show_function_definition(&mut output, &fn_definition);
    }

    return output;
  }
}

fn move_glsl_version_to_top(content: String) -> String {
  let mut lines = content.lines().collect::<Vec<&str>>();
  let mut version_line = None;

  for (i, line) in lines.iter().enumerate() {
    if line.starts_with("#version") {
      version_line = Some(i);
      break;
    }
  }

  if let Some(version_line) = version_line {
    let version_line = lines.remove(version_line);
    lines.insert(0, version_line);
  }

  return lines.join("\n");
}
