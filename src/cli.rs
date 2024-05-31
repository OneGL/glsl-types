use crate::generator::type_script;
use crate::{canonicalize, create_dir_all, file_exists};
use crate::import_resolver;
use std::path::PathBuf;

pub fn generate(file_path: String, input_folder: String, output_folder: String) {
  if !file_exists(&input_folder) {
    create_dir_all(&input_folder);
  }

  if !file_exists(&output_folder) {
    create_dir_all(&output_folder);
  }

  let file_path = PathBuf::from(file_path);
  let input_folder = PathBuf::from(input_folder);
  let output_folder = PathBuf::from(output_folder);

  // Update the file path to be relative to the input folder
  let input_folder_canon = PathBuf::from(canonicalize(&input_folder.to_str().unwrap()));
  let input_folder_parent = &input_folder_canon.parent().unwrap().to_path_buf();

  let combined_vertex = if let Some(output) =
    import_resolver::import_resolver::try_resolve_imports(&file_path, input_folder_parent)
  {
    output
  } else {
    return;
  };

  type_script::generate_types_file(combined_vertex, &file_path, &output_folder);
}
