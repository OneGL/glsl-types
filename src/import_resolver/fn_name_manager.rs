use std::{collections::HashMap, path::PathBuf};

#[derive(Clone)]
pub struct FunctionNameManager {
  // fn_name -> owner (file)
  fn_names: HashMap<String, PathBuf>,
}

impl FunctionNameManager {
  pub fn new() -> Self {
    Self {
      fn_names: HashMap::new(),
    }
  }

  pub fn get_fn_name(&mut self, fn_name: &str, file_path: &PathBuf) -> String {
    let mut id = 0;
    let mut new_fn_name = fn_name.to_string();

    loop {
      if let Some(owner) = self.fn_names.get(&new_fn_name) {
        if file_path == owner {
          // This file is the owner of the function name
          break;
        }

        new_fn_name = format!("{}_{}", fn_name, id);
        id += 1;
      } else {
        // The function name is unique
        break;
      }
    }

    self.fn_names.insert(new_fn_name.clone(), file_path.clone());

    return new_fn_name;
  }
}
