use crate::generator::type_script;
use crate::import_resolver;
use crate::log;
use crate::log::print_level;
use crate::{canonicalize, create_dir_all, file_exists};
use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

const DEFAULT_INPUT_FOLDER: &str = "shaders";
const DEFAULT_OUTPUT_FOLDER: &str = "output";

/// This program reads a glsl file and generates TS types for the uniforms
#[derive(Parser)]
#[command()]
struct Args {
  /// Input folder with glsl files
  #[arg(short, long, default_value = DEFAULT_INPUT_FOLDER)]
  input_float: std::path::PathBuf,

  /// Output folder for the generated types
  #[arg(short, long, default_value = DEFAULT_OUTPUT_FOLDER)]
  output_folder: std::path::PathBuf,
}

pub fn start(file_path: String) {
  let args: Vec<String> = vec![];
  let args = Args::try_parse_from(args).expect("Failed to parse arguments");

  if !file_exists(args.input_float.to_str().unwrap()) {
    if args.input_float.to_str().unwrap() == DEFAULT_INPUT_FOLDER {
      create_dir_all(&args.input_float.to_str().unwrap());
    } else {
      panic!("Input folder does not exist");
    }
  }

  if !file_exists(args.output_folder.to_str().unwrap()) {
    if args.output_folder.to_str().unwrap() == DEFAULT_OUTPUT_FOLDER {
      create_dir_all(&args.output_folder.to_str().unwrap());
    } else {
      panic!("Output folder does not exist");
    }
  }

  let input_folder = args.input_float.clone();
  let output_folder = args.output_folder.clone();

  generate(
    PathBuf::from(file_path),
    input_folder.clone(),
    output_folder.clone(),
  );
}

fn generate(file_path: PathBuf, input_folder: PathBuf, output_folder: PathBuf) {
  // Update the file path to be relative to the input folder
  let input_folder_canon = PathBuf::from(canonicalize(&input_folder.to_str().unwrap()));
  let input_folder_parent = &input_folder_canon.parent().unwrap().to_path_buf();
  let file_path_relative_to_input = file_path.strip_prefix(input_folder_parent).unwrap();

  // Measure the time it takes to generate the types

  let combined_vertex = if let Some(output) =
    import_resolver::import_resolver::try_resolve_imports(&file_path, input_folder_parent)
  {
    output
  } else {
    return;
  };

  let success = type_script::generate_types_file(combined_vertex, &file_path, &output_folder);

  if success {
    print_level(log::Level::INFO);
    print!(
      "Files processed: {}",
      file_path_relative_to_input
        .to_str()
        .unwrap()
        .blue()
        .underline()
    );
  }
}
