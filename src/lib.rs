#![deny(clippy::all)]

use wasm_bindgen::prelude::*;

mod cli;
mod generator;
mod import_resolver;
mod utils;

#[wasm_bindgen]
extern "C" {
  fn read_file(file: String) -> String;
  fn logln(message: &str);
  fn log(message: &str);
  fn log_with_color(message: &str, color: &str);
  fn canonicalize(path: &str) -> String;
  fn file_exists(path: &str) -> bool;
  fn create_dir_all(path: &str);
  fn write_file(path: &str, content: &str);
}

#[wasm_bindgen]
pub fn start_cli(file_path: String, input_folder: String, output_folder: String) {
  cli::generate(file_path, input_folder, output_folder);
}

#[wasm_bindgen]
pub fn resolve_imports(file: String, input_folder: String) -> String {
  let file = std::path::PathBuf::from(file);
  let input_folder = std::path::PathBuf::from(input_folder);

  match import_resolver::import_resolver::try_resolve_imports(&file, &input_folder) {
    Some(output) => output,
    None => String::from(""),
  }
}
