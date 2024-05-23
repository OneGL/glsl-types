use glsl::syntax::TranslationUnit;
use glsl::visitor::{Host, Visit, Visitor};

use std::collections::HashMap;
use std::path::PathBuf;

use super::import_resolver::ImportError;

pub fn get_file_imports(
  ast: &mut TranslationUnit,
  file_path: &PathBuf,
) -> Result<HashMap<String, PathBuf>, ImportError> {
  let mut visitor = FileImports::new(file_path);
  ast.visit(&mut visitor);

  if let Some(error) = visitor.error {
    return Err(error);
  }

  return Ok(visitor.imports);
}

#[derive(Clone, Debug)]
pub struct FileImports {
  base_path: PathBuf,
  imports: HashMap<String, PathBuf>,
  error: Option<ImportError>,
}

impl FileImports {
  fn new(file_path: &PathBuf) -> Self {
    Self {
      base_path: file_path.parent().unwrap().to_path_buf(),
      imports: HashMap::new(),
      error: None,
    }
  }
}

impl Visitor for FileImports {
  fn visit_import(&mut self, import: &glsl::syntax::Import) -> Visit {
    let path = match &import.path {
      glsl::syntax::Path::Absolute(path) => PathBuf::from(path),
      glsl::syntax::Path::Relative(path) => match self.base_path.join(path).canonicalize() {
        Ok(path) => path,
        Err(_) => {
          self.error = Some(ImportError::InvalidFilePath(path.clone()));
          return Visit::Parent;
        }
      },
    };

    let identifier = import.identifier.to_string();

    if self.imports.contains_key(&identifier) {
      self.error = Some(ImportError::DuplicateImportIdentifier(identifier));
      return Visit::Parent;
    }

    self.imports.insert(identifier, path);
    return Visit::Parent;
  }
}
