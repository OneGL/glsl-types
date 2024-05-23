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
    for struct_definition in visitor.struct_definitions {
      transpiler::glsl::show_struct(&mut output, &struct_definition);
    }

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

static DEFAULT_TOKEN: Token = Token {
  word: String::new(),
  word_start: 0,
};

fn update_imported_fn_calls(
  source: &str,
  import_identifier: &str,
  import_path: &PathBuf,
  name_manager: &mut FunctionNameManager,
) -> Result<String, ImportError> {
  // We need to find the following pattern: <import_identifier>.<prev_fn_name>
  let mut output = String::new();
  output.push_str(source);

  let tokens = get_tokens_from_source(source);

  for i in 0..tokens.len() {
    let mut import_identifier_found = false;
    let mut dot_found = false;

    let token1 = &tokens[i];
    let token2 = &tokens.get(i + 1).unwrap_or(&DEFAULT_TOKEN);
    let token3 = &tokens.get(i + 2).unwrap_or(&DEFAULT_TOKEN);

    if token1.word == import_identifier {
      import_identifier_found = true;
    }

    if token2.word == "." {
      dot_found = true;
    }

    if import_identifier_found && dot_found {
      let fn_name = &token3.word;

      let file_function_names = name_manager
        .file_fn_names
        .entry(import_path.clone())
        .or_insert_with(Vec::new);

      if !file_function_names.contains(&fn_name) {
        return Err(ImportError::FileDoesNotExportFunction {
          fn_name: fn_name.to_string(),
          import_identifier: import_identifier.to_string(),
          import_path: import_path.clone(),
        });
      }

      let new_fn_name = name_manager.get_fn_name(&fn_name, import_path);
      let start = token1.word_start;
      let end = token3.word_start + token3.word.len();

      // Add space after the new function name to match the original
      // width of the function name
      let spaces_to_add = (end - start) - new_fn_name.len();
      let new_fn_name = format!("{}{}", new_fn_name, " ".repeat(spaces_to_add));
      output.replace_range(start..end, &new_fn_name);
    }
  }

  return Ok(output);
}

fn get_tokens_from_source(source: &str) -> Vec<Token> {
  let mut iter = WhitespaceSkippingIterator::new(source);
  let mut words = Vec::new();

  while let Some(word) = get_next_token(&mut iter) {
    words.push(word);
  }

  return words;
}

#[derive(Debug)]
struct Token {
  pub word: String,
  pub word_start: usize,
}

fn get_next_token(iter: &mut WhitespaceSkippingIterator) -> Option<Token> {
  let mut word = String::new();
  let mut word_start = None;

  while let Some((c, _, is_whitespace)) = iter.next() {
    if is_whitespace {
      continue;
    }

    if !c.is_alphanumeric() && c != '_' {
      return Some(Token {
        word: c.to_string(),
        word_start: iter.i - 1,
      });
    }

    word.push(c);
    word_start = Some(iter.i - 1);
    break;
  }

  while let Some((c, _, is_whitespace)) = iter.peek() {
    if is_whitespace || (!c.is_alphanumeric() && c != '_') {
      break;
    }

    word.push(c);
    iter.next();
  }

  match word_start {
    Some(word_start) => Some(Token { word, word_start }),
    None => None,
  }
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

  fn peek(&self) -> Option<(char, usize, bool)> {
    if self.i < self.chars.len() {
      let c = self.chars[self.i];
      return Some((
        c,
        self.i,
        c.is_whitespace()
          || self.inside_a_single_line_comment
          || self.inside_a_multi_line_comment
          || self.inside_string,
      ));
    }

    None
  }
}
