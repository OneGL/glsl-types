use glsl::parser::Parse as _;
use glsl::syntax::{ShaderStage, TranslationUnit};
use std::{collections::HashMap, path::PathBuf};

use super::{import_resolver::ImportError, imports::get_file_imports};

#[derive(Debug, Clone)]
pub struct File {
  pub contents: String,
  pub imports: HashMap<String, PathBuf>,
  pub ast: TranslationUnit,
}

pub struct FileManager {
  files: HashMap<PathBuf, File>,
}

impl FileManager {
  pub fn new() -> Self {
    Self {
      files: HashMap::new(),
    }
  }

  pub fn get_file(&mut self, file_path: &PathBuf) -> Result<File, ImportError> {
    if let Some(file) = self.files.get(file_path) {
      return Ok(file.clone());
    }

    let contents = match std::fs::read_to_string(file_path) {
      Ok(contents) => contents,
      Err(_) => return Err(ImportError::FileNotFound(file_path.to_path_buf())),
    };

    let mut ast = match ShaderStage::parse(&contents) {
      Ok(ast) => ast,
      Err(_) => return Err(ImportError::CouldNotParseFile(file_path.to_path_buf())),
    };

    let imports = get_file_imports(&mut ast, file_path)?;

    let file = File {
      contents,
      imports,
      ast,
    };

    self.files.insert(file_path.clone(), file.clone());
    return Ok(file);
  }

  pub fn get_file_imports(
    &mut self,
    file_path: &PathBuf,
  ) -> Result<HashMap<String, PathBuf>, ImportError> {
    let file = self.get_file(file_path)?;
    return Ok(file.imports);
  }
}
