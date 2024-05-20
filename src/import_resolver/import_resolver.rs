use std::collections::HashSet;
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

  println!("{:?}", resolver.graph);

  return resolver.combine_files(file, &mut HashSet::new());
}

struct ImportResolver {
  graph: Graph,
  file_manager: FileManager,
  function_names: FunctionNameManager,
}

impl ImportResolver {
  pub fn new() -> Self {
    Self {
      graph: Graph::new(),
      file_manager: FileManager::new(),
      function_names: FunctionNameManager::new(),
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

  fn combine_files(&mut self, node: &PathBuf, visited: &mut HashSet<PathBuf>) -> String {
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
      output += &self.combine_files(&neighbor, visited);
    }

    output += &self.get_file_content(node);

    return output;
  }

  fn get_file_content(&mut self, file_path: &PathBuf) -> String {
    let file = self.file_manager.get_file(file_path);
    let mut ast = ShaderStage::parse(&file.contents).unwrap();

    let fn_names = self.function_names.clone();
    let fn_names = rename_functions_to_avoid_collisions(&mut ast, fn_names, file_path);
    let fn_names =
      rename_imported_function_calls(&mut ast, fn_names, file.imports, file_path.clone());
    self.function_names = fn_names;

    // Transpile the shader AST back to GLSL
    let mut output = String::new();
    transpiler::glsl::show_translation_unit(&mut output, &ast);
    return output;
  }
}
