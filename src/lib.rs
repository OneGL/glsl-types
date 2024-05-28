#![deny(clippy::all)]

use wasm_bindgen::prelude::*;

mod cli;
mod debounce;
mod error_check;
mod generator;
mod import_resolver;
mod log;
mod utils;

#[wasm_bindgen]
extern "C" {
  fn read_file(file: String) -> String;
  fn log(message: String);
  fn canonicalize(path: &str) -> String;
}

// #[wasm_bindgen]
// pub fn start_cli() -> String {
//   cli::start(args);
// }

#[wasm_bindgen]
pub fn resolve_imports(file: String, input_folder: String) -> String {
  let file = std::path::PathBuf::from(file);
  let input_folder = std::path::PathBuf::from(input_folder);

  match import_resolver::import_resolver::try_resolve_imports(&file, &input_folder) {
    Some(output) => output,
    None => String::from(""),
  }
}
