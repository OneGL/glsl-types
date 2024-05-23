use super::file::{get_file_data, ImportedFile};
use super::import_resolver::ImportError;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct FileManager {
  pub files: HashMap<PathBuf, ImportedFile>,
}

impl FileManager {
  pub fn new() -> Self {
    Self {
      files: HashMap::new(),
    }
  }

  pub fn get_file(&mut self, file_path: &PathBuf) -> Result<ImportedFile, ImportError> {
    if let Some(file) = self.files.get(file_path) {
      return Ok(file.clone());
    }

    let file = get_file_data(file_path)?;
    self.files.insert(file_path.clone(), file.clone());

    return Ok(file);
  }

  pub fn get_file_imports(&mut self, file_path: &PathBuf) -> Result<Vec<PathBuf>, ImportError> {
    return match self.get_file(file_path) {
      Ok(file) => Ok(file.imports),
      Err(error) => Err(error),
    };
  }
}
