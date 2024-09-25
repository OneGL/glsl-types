use crate::canonicalize;
use crate::file_exists;
use crate::read_file;

use super::import_resolver::ImportError;
use glsl::parser::Parse as _;
use glsl::syntax::{FunctionDefinition, PreprocessorInclude, ShaderStage, StructSpecifier};
use glsl::visitor::{Host, Visit, Visitor};
use std::path::PathBuf;

pub fn get_file_data(file_path: &PathBuf) -> Result<ImportedFile, ImportError> {
  let file_path_string = file_path.to_str().unwrap().to_string();

  if !file_exists(&file_path_string) {
    return Err(ImportError::FileNotFound(file_path.to_path_buf()));
  }

  let contents = read_file(file_path_string);

  let ast = match ShaderStage::parse(&contents) {
    Ok(ast) => ast,
    Err(_) => return Err(ImportError::CouldNotParseFile(file_path.to_path_buf())),
  };

  let mut visitor = ImportedFileVisitor::new(file_path);
  ast.visit(&mut visitor);

  if let Some(error) = visitor.error {
    return Err(error);
  }

  return Ok(ImportedFile {
    structs: visitor.structs,
    functions: visitor.functions,
    imports: visitor.imports,
    contents,
  });
}

#[derive(Clone, Debug)]
pub struct ImportedFile {
  pub structs: Vec<String>,
  pub functions: Vec<String>,
  pub imports: Vec<PathBuf>,
  pub contents: String,
}

struct ImportedFileVisitor {
  path: PathBuf,
  parent_path: PathBuf,
  structs: Vec<String>,
  functions: Vec<String>,
  imports: Vec<PathBuf>,
  error: Option<ImportError>,
}

impl ImportedFileVisitor {
  fn new(path: &PathBuf) -> Self {
    Self {
      path: path.clone(),
      parent_path: path.parent().unwrap().to_path_buf(),
      structs: Vec::new(),
      functions: Vec::new(),
      imports: Vec::new(),
      error: None,
    }
  }
}

impl Visitor for ImportedFileVisitor {
  fn visit_function_definition(&mut self, function: &FunctionDefinition) -> Visit {
    self.functions.push(function.prototype.name.to_string());
    Visit::Parent
  }

  fn visit_struct_specifier(&mut self, struct_specifier: &StructSpecifier) -> Visit {
    return match &struct_specifier.name {
      Some(name) => {
        self.structs.push(name.to_string());
        Visit::Parent
      }
      None => Visit::Parent,
    };
  }

  fn visit_preprocessor_include(&mut self, import: &PreprocessorInclude) -> Visit {
    let path = match &import.path {
      glsl::syntax::Path::Absolute(path) => PathBuf::from(path),
      glsl::syntax::Path::Relative(path) => {
        PathBuf::from(canonicalize(self.parent_path.join(path).to_str().unwrap()))
      }
    };

    if self.imports.contains(&path) {
      self.error = Some(ImportError::DuplicateImport(self.path.clone(), path));
      return Visit::Parent;
    }

    self.imports.push(path);

    Visit::Parent
  }
}
