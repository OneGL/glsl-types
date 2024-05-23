use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use crate::error_check::error_check;
use crate::generator::type_script;
use crate::import_resolver;
use crate::import_resolver::import_resolver::ImportError;
use crate::log;
use crate::utils::get_shader_type::get_shader_type;
use crate::utils::get_shader_type::ShaderType;
use crate::utils::get_shader_type::FRAGMENT_SHADER_EXTENSIONS;
use crate::utils::get_shader_type::VERTEX_SHADER_EXTENSIONS;
use crate::{debounce, log::print_level, log::Level};
use clap::Parser;
use colored::Colorize;
use notify::{Event, RecursiveMode, Watcher};

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

pub fn start(args: Vec<String>) -> () {
  let args = Args::try_parse_from(args).expect("Failed to parse arguments");

  if !args.input_float.exists() {
    if args.input_float.to_str().unwrap() == DEFAULT_INPUT_FOLDER {
      std::fs::create_dir_all(&args.input_float).unwrap();
    } else {
      panic!("Input folder does not exist");
    }
  }

  if !args.output_folder.exists() {
    if args.output_folder.to_str().unwrap() == DEFAULT_OUTPUT_FOLDER {
      std::fs::create_dir_all(&args.output_folder).unwrap();
    } else {
      panic!("Output folder does not exist");
    }
  }

  let input_folder = args.input_float.clone();
  let output_folder = args.output_folder.clone();

  print!("\x1B[2J\x1B[1;1H");
  println!("{}", "GLSL Types Generator".bold());

  print_level(log::Level::INFO);

  println!(
    "Watching for changes in the folder: {}",
    input_folder.to_str().unwrap().cyan()
  );

  let debounced = debounce::Debouncer::new(Duration::from_millis(10), move |event: Event| {
    // If it is a folder event, ignore it
    if event.paths.len() == 0 {
      return;
    }

    let file_path = event.paths.first().unwrap();

    // Update the file path to be relative to the input folder
    let input_folder_canon = std::fs::canonicalize(&input_folder).unwrap();
    let input_folder_parent = input_folder_canon.parent().unwrap();
    let file_path_relative_to_input = file_path.strip_prefix(&input_folder_parent).unwrap();

    let (vertex_path, fragment_path) = match ensure_both_shader_files_exist(file_path.clone()) {
      Ok((vertex_path, fragment_path)) => (vertex_path, fragment_path),
      Err(err) => match err {
        LoadShaderError::InvalidInputFile => {
          // Ignore invalid input files
          return;
        }
        LoadShaderError::MissingShaderPair(shader_type) => {
          log_missing_shader_error(file_path_relative_to_input, shader_type);
          return;
        }
      },
    };

    // Measure the time it takes to generate the types
    // let start = std::time::Instant::now();
    // let success = type_script::generate_ts_types_file(&vertex_path, &fragment_path, &output_folder);

    // if success {
    //   print_level(log::Level::INFO);
    //   print!(
    //     "Types generated for the shader file: {}",
    //     file_path_relative_to_input
    //       .to_str()
    //       .unwrap()
    //       .blue()
    //       .underline()
    //   );

    //   println!(
    //     " {}",
    //     format!("({:?})", start.elapsed()).truecolor(130, 130, 130)
    //   );

    match import_resolver::import_resolver::resolve_imports(&vertex_path) {
      Ok(combined_vertex) => {
        println!("{}", combined_vertex);

        match error_check(&combined_vertex) {
          Ok(errors) => {
            for error in errors {
              println!("Error: {:?}", error);
            }
          }
          Err(_) => {
            println!("Error checking failed");
          }
        }
      }
      Err(err) => match err {
        ImportError::CouldNotParseFile(file_path) => {
          println!("Could not parse file: {}", file_path.to_str().unwrap());
        }
        ImportError::CycleDetected => {
          println!("Cycle detected");
        }
        ImportError::FileNotFound(file_path) => {
          println!("File not found: {}", file_path.to_str().unwrap());
        }
        ImportError::DuplicateImportIdentifier(identifier) => {
          println!("Duplicate import identifier: {}", identifier);
        }
        ImportError::FileDoesNotExportFunction {
          fn_name,
          import_identifier,
          import_path,
        } => {
          println!("File does not export function: {}", fn_name);
          println!("Import identifier: {}", import_identifier);
          println!("Import path: {}", import_path.to_str().unwrap());
        }
        ImportError::InvalidFilePath(path) => {
          println!("Invalid file path: {}", path);
        }
      },
    }

    // let combined_vertex = .unwrap();
    // println!("{}", combined_vertex);

    // println!("");
    // println!("");
    // println!("");

    // let combined_fragment =
    //   import_resolver::import_resolver::resolve_imports(&fragment_path).unwrap();
    // println!("{}", combined_fragment);
    // }
  });

  let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| match res {
    Ok(event) => {
      debounced.call(event);
    }
    Err(e) => {
      println!("watch error: {:?}", e);
    }
  })
  .unwrap();

  watcher
    .watch(&args.input_float, RecursiveMode::Recursive)
    .unwrap();

  loop {
    std::thread::sleep(std::time::Duration::from_millis(100));
  }
}

enum LoadShaderError {
  MissingShaderPair(ShaderType),
  InvalidInputFile,
}

fn ensure_both_shader_files_exist(
  file_path: PathBuf,
) -> Result<(PathBuf, PathBuf), LoadShaderError> {
  let file_without_extension = file_path.with_extension("");

  let shader_type = match get_shader_type(&file_path) {
    Some(shader_type) => shader_type,
    None => return Err(LoadShaderError::InvalidInputFile),
  };

  match shader_type {
    ShaderType::Fragment => {
      for extension in VERTEX_SHADER_EXTENSIONS.iter() {
        let vertex_shader_path = file_without_extension.with_extension(extension);

        if vertex_shader_path.exists() {
          return Ok((vertex_shader_path, file_path));
        }
      }

      return Err(LoadShaderError::MissingShaderPair(ShaderType::Vertex));
    }
    ShaderType::Vertex => {
      for extension in FRAGMENT_SHADER_EXTENSIONS.iter() {
        let fragment_shader_path = file_without_extension.with_extension(extension);

        if fragment_shader_path.exists() {
          return Ok((file_path, fragment_shader_path));
        }
      }

      return Err(LoadShaderError::MissingShaderPair(ShaderType::Fragment));
    }
  }
}

fn log_missing_shader_error(file_path: &Path, shader_type: ShaderType) {
  let file_without_extension = file_path.with_extension("");

  println!("");
  print_level(Level::ERROR);

  match shader_type {
    ShaderType::Vertex => {
      println!(
        "Missing vertex shader for: {}",
        file_path.to_str().unwrap().blue().underline()
      );
      println!("");
      println!(
        "Please create a vertex shader file: {}{}",
        file_without_extension.to_str().unwrap().blue().underline(),
        (".".to_string() + VERTEX_SHADER_EXTENSIONS[0])
          .blue()
          .underline()
          .bold()
      );
    }
    ShaderType::Fragment => {
      println!(
        "Missing fragment shader for: {}",
        file_path.to_str().unwrap().blue().underline()
      );
      println!("");
      println!(
        "Please create a fragment shader file: {}{}",
        file_without_extension.to_str().unwrap().blue().underline(),
        (".".to_string() + FRAGMENT_SHADER_EXTENSIONS[0])
          .blue()
          .underline()
          .bold()
      );
    }
  }

  println!("");
  println!("When creating a shader, you need to create both the vertex and fragment shader files.");
  println!(
    "For example, if you create a shader file called {}, you also need to create a file called {}.",
    "example.vert".cyan(),
    "example.frag".cyan()
  );
  println!("");
  println!("Example:");
  println!("");
  println!("├── shaders");
  println!("│   ├── {}.vert", "example".cyan());
  println!("│   └── {}.frag", "example".cyan());
  println!("│");
  println!("├── output");
  println!("    └── {}.ts", "example".cyan());
  println!("");
}
