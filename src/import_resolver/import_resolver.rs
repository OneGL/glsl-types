use glsl::transpiler;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::PathBuf;

use super::file_manager::FileManager;
use super::fn_calls::rename_imported_function_calls;
use super::fn_definitions::rename_functions_to_avoid_collisions;
use super::fn_name_manager::FunctionNameManager;
use super::graph::Graph;

#[derive(Debug, Clone)]
pub enum ImportError {
  CycleDetected,
  CouldNotParseFile(PathBuf),
  FileNotFound(PathBuf),
  DuplicateImportIdentifier(String),
  FileDoesNotExportFunction {
    fn_name: String,
    import_identifier: String,
    import_path: PathBuf,
  },
}

pub fn resolve_imports(file: &PathBuf) -> Result<String, ImportError> {
  let mut resolver = ImportResolver::new();

  resolver.build_import_graph(file)?;

  let output = resolver.combine_files(file, &mut HashSet::new(), true)?;

  return Ok(move_glsl_version_to_top(output));
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

  fn build_import_graph(&mut self, file_path: &PathBuf) -> Result<&Graph, ImportError> {
    let file_imports = self.file_manager.get_file_imports(&file_path)?;

    for (_, path) in file_imports {
      self.graph.add_edge(file_path.clone(), path.clone());

      if self.graph.has_cycle() {
        return Err(ImportError::CycleDetected);
      }

      self.build_import_graph(&path)?;
    }

    return Ok(&self.graph);
  }

  fn combine_files(
    &mut self,
    node: &PathBuf,
    visited: &mut HashSet<PathBuf>,
    is_root: bool,
  ) -> Result<String, ImportError> {
    // Here we are reserving the function names of the root file
    // if we do it now, the root file will preserve its function names
    if is_root {
      let mut file = self.file_manager.get_file(node)?;

      let visitor =
        rename_functions_to_avoid_collisions(&mut file.ast, self.fn_names.clone(), node);

      self.fn_names = visitor.fn_name_manager;
    }

    let mut output = String::new();

    if visited.contains(node) {
      return Ok(output);
    }

    visited.insert(node.clone());

    let neighbors = self
      .graph
      .get_neighbors(node)
      .cloned()
      .unwrap_or_else(Vec::new);

    for neighbor in neighbors {
      output += &self.combine_files(&neighbor, visited, false)?;
    }

    output += &self.get_file_content(node, is_root)?;

    return Ok(output);
  }

  fn get_file_content(
    &mut self,
    file_path: &PathBuf,
    is_root: bool,
  ) -> Result<String, ImportError> {
    let mut output = String::new();
    let mut file = self.file_manager.get_file(file_path)?;

    if is_root {
      output += "\n\n// Main file\n\n";
      output += &file.contents;

      for (import_identifier, import_path) in &file.imports {
        output = update_imported_fn_calls(
          &output,
          &import_identifier,
          &import_path,
          &mut self.fn_names,
        )?;
      }

      return Ok(output);
    }

    let fn_names = self.fn_names.clone();

    let visitor = rename_functions_to_avoid_collisions(&mut file.ast, fn_names, file_path);

    let fn_names = visitor.fn_name_manager;

    let fn_names =
      rename_imported_function_calls(&mut file.ast, fn_names, file.imports, file_path.clone());

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
    for fn_definition in visitor.fn_definitions {
      transpiler::glsl::show_function_definition(&mut output, &fn_definition);
    }

    return Ok(output);
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

fn update_imported_fn_calls(
  source: &str,
  import_identifier: &str,
  import_path: &PathBuf,
  name_manager: &mut FunctionNameManager,
) -> Result<String, ImportError> {
  // We need to find the following pattern: <import_identifier>.<prev_fn_name>
  let mut output = String::new();
  let mut iter = WhitespaceSkippingIterator::new(source);
  let import_identifier_with_dot = format!("{}.", import_identifier);

  while let Some((c, i, is_whitespace)) = iter.next() {
    output.push(c);

    // Skip processing if current character is whitespace
    if is_whitespace {
      continue;
    }

    // Skip processing if the remaining source is shorter than the import identifier
    if i + import_identifier_with_dot.len() > source.len() {
      continue;
    }

    let slice = &source[i..i + import_identifier_with_dot.len()];

    // We need to check if the slice is the import identifier
    // move forward to check if the next character is a dot
    // and then get the function name between the dot and the opening parenthesis
    if slice == import_identifier_with_dot {
      iter.i += import_identifier_with_dot.len() - 1;
      let mut fn_name = String::new();

      while let Some((c, _, is_whitespace)) = iter.next() {
        if is_whitespace {
          continue;
        }

        if c == '(' {
          break;
        }

        fn_name.push(c);
      }

      let file_function_names = name_manager
        .file_fn_names
        .entry(import_path.clone())
        .or_insert_with(Vec::new);

      if !file_function_names.contains(&fn_name) {
        return Err(ImportError::FileDoesNotExportFunction {
          fn_name,
          import_identifier: import_identifier.to_string(),
          import_path: import_path.clone(),
        });
      }

      let new_fn_name = name_manager.get_fn_name(&fn_name, import_path);

      output.pop();
      output += &new_fn_name;
      output += "(";
    }
  }

  return Ok(output);
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
