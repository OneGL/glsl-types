use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::path::PathBuf;

use crate::log::{print_level, Level};

use super::file_manager::FileManager;
use super::graph::Graph;

#[derive(Debug, Clone)]
pub enum DefinitionErrorType {
  Function,
  Struct,
}

#[derive(Debug, Clone)]
pub enum ImportError {
  CycleDetected(PathBuf, PathBuf),
  CouldNotParseFile(PathBuf),
  FileNotFound(PathBuf),
  DuplicateImport(PathBuf),
  InvalidFilePath(String),
  DuplicateDefinition {
    name: String,
    first_file: PathBuf,
    second_file: PathBuf,
    definition_type: DefinitionErrorType,
  },
}

pub fn try_resolve_imports(file: &PathBuf) -> Option<String> {
  match resolve_imports(file) {
    Ok(output) => Some(output),
    Err(err) => {
      match err {
        ImportError::CouldNotParseFile(file_path) => {
          print_level(Level::ERROR);
          println!("Could not parse file: {}", file_path.to_str().unwrap());
        }
        ImportError::CycleDetected(file_path, import_path) => {
          print_level(Level::ERROR);
          println!(
            "Cycle detected between files: {} and {}",
            file_path.to_str().unwrap(),
            import_path.to_str().unwrap()
          );
        }
        ImportError::FileNotFound(file_path) => {
          print_level(Level::ERROR);
          println!("File not found: {}", file_path.to_str().unwrap());
        }
        ImportError::DuplicateImport(path) => {
          print_level(Level::ERROR);
          println!("Duplicate import identifier: {}", path.to_str().unwrap());
        }
        ImportError::DuplicateDefinition {
          name,
          first_file,
          second_file,
          definition_type,
        } => {
          print_level(Level::ERROR);
          println!(
            "Duplicate definition of {} in files: {} and {}, type {:?}",
            name,
            first_file.to_str().unwrap(),
            second_file.to_str().unwrap(),
            definition_type
          );
        }
        ImportError::InvalidFilePath(path) => {
          print_level(Level::ERROR);
          println!("Invalid file path: {}", path);
        }
      }

      return None;
    }
  }
}

pub fn resolve_imports(file: &PathBuf) -> Result<String, ImportError> {
  let mut resolver = ImportResolver::new();

  resolver.build_import_graph(file)?;
  let output = resolver.combine_files(file, &mut HashSet::new());
  resolver.check_for_duplicate_definitions()?;
  let output = move_glsl_version_to_top(output);

  return Ok(output);
}

#[derive(Debug)]
struct ImportResolver {
  graph: Graph,
  file_manager: FileManager,
}

impl ImportResolver {
  pub fn new() -> Self {
    Self {
      graph: Graph::new(),
      file_manager: FileManager::new(),
    }
  }

  fn check_for_duplicate_definitions(&self) -> Result<(), ImportError> {
    // name -> file_owner
    let mut fn_definitions: HashMap<String, PathBuf> = HashMap::new();
    let mut struct_definitions: HashMap<String, PathBuf> = HashMap::new();

    for (file_path, file) in &self.file_manager.files {
      for function in &file.functions {
        if let Some(file_owner) = fn_definitions.get(function) {
          // GLSL supports function overloading, but we will only allow it
          // if the functions are defined in the same file.
          if file_owner != file_path {
            return Err(ImportError::DuplicateDefinition {
              name: function.clone(),
              first_file: file_owner.clone(),
              second_file: file_path.clone(),
              definition_type: DefinitionErrorType::Function,
            });
          }
        }

        fn_definitions.insert(function.clone(), file_path.clone());
      }

      for struct_name in &file.structs {
        if let Some(file_owner) = struct_definitions.get(struct_name) {
          // Here we do not check if the struct is defined in the same file
          // because GLSL does not support struct overloading.
          return Err(ImportError::DuplicateDefinition {
            name: struct_name.clone(),
            first_file: file_owner.clone(),
            second_file: file_path.clone(),
            definition_type: DefinitionErrorType::Struct,
          });
        }

        struct_definitions.insert(struct_name.clone(), file_path.clone());
      }
    }

    return Ok(());
  }

  fn build_import_graph(&mut self, file_path: &PathBuf) -> Result<&Graph, ImportError> {
    let file_imports = self.file_manager.get_file_imports(&file_path)?;

    for path in file_imports {
      self.graph.add_edge(file_path.clone(), path.clone());

      if self.graph.has_cycle() {
        return Err(ImportError::CycleDetected(file_path.clone(), path.clone()));
      }

      self.build_import_graph(&path)?;
    }

    return Ok(&self.graph);
  }

  fn combine_files(&mut self, node: &PathBuf, visited: &mut HashSet<PathBuf>) -> String {
    let mut output = String::new();

    if visited.contains(node) {
      return output;
    }

    visited.insert(node.clone());

    let file = self.file_manager.get_file(node).unwrap();

    for neighbor in file.imports {
      output += &self.combine_files(&neighbor, visited);
    }

    output += &format!(
      "\n\n// File: {}\n\n",
      node
        .file_name()
        .unwrap_or(OsStr::new("unknown"))
        .to_str()
        .unwrap_or("unknown")
    );

    output += &file.contents;

    return output;
  }
}

fn move_glsl_version_to_top(content: String) -> String {
  let mut lines = content.lines().collect::<Vec<&str>>();
  let mut version_line = None;

  for (i, line) in lines.iter().enumerate() {
    let parts = line.trim().split_whitespace().collect::<Vec<&str>>();

    if parts.len() >= 2 {
      if parts[0] == "#" && parts[1] == "version" {
        version_line = Some(i);
        break;
      } else if parts[0] == "#version" {
        version_line = Some(i);
        break;
      }
    }
  }

  if let Some(version_line) = version_line {
    let version_line = lines.remove(version_line);
    lines.insert(0, version_line);
  }

  return lines.join("\n");
}
