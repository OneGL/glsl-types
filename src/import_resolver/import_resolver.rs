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
    let mut output = String::new();
    let file = self.file_manager.get_file(file_path);

    if is_root {
      output += "\n\n// Main file\n\n";
      output += &file.contents;

      for (import_identifier, import_path) in file.imports {
        output = rename_fn_call_inplace(
          &output,
          &import_identifier,
          &import_path,
          &mut self.fn_names,
        );
      }

      return output;
    }

    let mut ast = ShaderStage::parse(&file.contents).unwrap();

    let fn_names = self.fn_names.clone();

    let (fn_names, fn_definitions) =
      rename_functions_to_avoid_collisions(&mut ast, fn_names, file_path);

    let fn_names =
      rename_imported_function_calls(&mut ast, fn_names, file.imports, file_path.clone());

    self.fn_names = fn_names;

    output += &format!(
      "\n\n// File: {}\n\n",
      file_path
        .file_name()
        .unwrap_or(OsStr::new("unknown"))
        .to_str()
        .unwrap_or("unknown")
    );

    // Transpile the shader AST back to GLSL
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

fn rename_fn_call_inplace(
  source: &str,
  import_identifier: &str,
  import_path: &PathBuf,
  name_manager: &mut FunctionNameManager,
) -> String {
  // We need to find the following pattern: <import_identifier>.<prev_fn_name>
  let mut output = String::new();
  let import_identifier_with_dot = format!("{}.", import_identifier);
  let mut iter = WhitespaceSkippingIterator::new(source);

  while let Some((c, i, w)) = iter.next() {
    output.push(c);

    // We want to keep the whitespace
    if w {
      continue;
    }

    if i + import_identifier_with_dot.len() > source.len() {
      break;
    }

    let slice = &source[i..i + import_identifier_with_dot.len()];

    // We need to check if the slice is the import identifier
    // move forward to check if the next character is a dot
    // and then get the function name between the dot and the opening parenthesis
    if slice == import_identifier_with_dot {
      iter.i += import_identifier_with_dot.len() - 1;
      let mut fn_name = String::new();

      while let Some((c, _, w)) = iter.next() {
        if w {
          continue;
        }

        if c == '(' {
          break;
        }

        fn_name.push(c);
      }

      let new_fn_name = name_manager.get_fn_name(&fn_name, import_path);

      output.pop();
      output += &new_fn_name;
      output += "(";
    }
  }

  return output;
}

struct WhitespaceSkippingIterator {
  pub i: usize,
  inside_string: bool,
  inside_a_single_line_comment: bool,
  inside_a_multi_line_comment: bool,
  chars: Vec<char>,
}

impl WhitespaceSkippingIterator {
  fn new(source: &str) -> Self {
    Self {
      i: 0,
      inside_string: false,
      inside_a_single_line_comment: false,
      inside_a_multi_line_comment: false,
      chars: source.chars().collect(),
    }
  }

  fn next(&mut self) -> Option<(char, usize, bool)> {
    while self.i < self.chars.len() {
      let c = self.chars[self.i];
      // Toggle inside_string flag only if we are not inside any comment
      if c == '\"' && !self.inside_a_single_line_comment && !self.inside_a_multi_line_comment {
        self.inside_string = !self.inside_string;
      }

      // Handle start of comments
      if !self.inside_string {
        // Handle comment start
        if c == '/' && self.i + 1 < self.chars.len() {
          let next_c = self.chars[self.i + 1];

          if next_c == '/' {
            self.inside_a_single_line_comment = true;
          } else if next_c == '*' {
            self.inside_a_multi_line_comment = true;
          }
        }

        // Handle end of single-line comments
        if c == '\n' {
          self.inside_a_single_line_comment = false;
        }

        // Handle end of multi-line comments
        if c == '*' && self.i + 1 < self.chars.len() {
          let next_c = self.chars[self.i + 1];

          if next_c == '/' {
            self.inside_a_multi_line_comment = false;
          }
        }
      }

      self.i += 1;

      return Some((
        c,
        self.i - 1,
        c.is_whitespace()
          || self.inside_a_single_line_comment
          || self.inside_a_multi_line_comment
          || self.inside_string,
      ));
    }

    None
  }
}
