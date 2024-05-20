use std::{collections::HashMap, path::PathBuf};

use super::imports::get_file_imports;

#[derive(Debug, Clone)]
pub struct File {
  pub contents: String,
  pub imports: HashMap<String, PathBuf>,
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

  pub fn get_file(&mut self, file_path: &PathBuf) -> File {
    if let Some(file) = self.files.get(file_path) {
      return file.clone();
    }

    let file_contents = std::fs::read_to_string(file_path).unwrap();
    let file_imports = get_file_imports(&file_contents, file_path);

    let file = File {
      contents: file_contents,
      imports: file_imports,
    };

    self.files.insert(file_path.clone(), file.clone());
    return file;
  }

  pub fn get_file_imports(&mut self, file_path: &PathBuf) -> HashMap<String, PathBuf> {
    self.get_file(file_path).imports.clone()
  }
}
