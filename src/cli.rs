extern crate chrono;
use std::time::Duration;

use crate::debounce;
use crate::generator::type_script;
use clap::Parser;
use colored::Colorize;
use notify::{Event, RecursiveMode, Result, Watcher};

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

  /// Output language
  /// Supported languages: ts, rs
  /// Default: ts
  #[arg(short, long, default_value = "ts")]
  language: String,
}

#[tokio::main]
pub async fn start(args: Vec<String>) -> String {
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
  let language = args.language.clone();

  print!("\x1B[2J\x1B[1;1H");
  println!("{}", "GLSL Types Generator".bold());
  print!("{}", "[INFO]\t".green());
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
    let file_extension = file_path.extension().unwrap();

    // Ignore files that do not end with .vert or .frag extension
    if file_extension != "vert" && file_extension != "frag" {
      return;
    }

    let input_folder_canon = std::fs::canonicalize(&input_folder).unwrap();
    let input_folder_parent = input_folder_canon.parent().unwrap();
    let file_path = file_path.strip_prefix(&input_folder_parent).unwrap();

    // The user should create both the vertex and fragment shader files
    // if the user only creates one of them, we will show an error message
    // and ignore the file.
    let file_folder = file_path.parent().unwrap();
    let file_stem = file_path.file_stem().unwrap().to_str().unwrap().to_string();
    let vertex_shader = file_folder.join(file_stem.clone() + ".vert");
    let fragment_shader = file_folder.join(file_stem.clone() + ".frag");

    if !vertex_shader.exists() || !fragment_shader.exists() {
      print!("{}", "[ERROR]\t".red());
      println!("Missing shader files.");
      println!("");

      if !vertex_shader.exists() {
        println!(
          "Please create a vertex shader file: {}",
          vertex_shader.to_str().unwrap().blue().underline()
        );
      } else {
        println!(
          "Please create a fragment shader file: {}",
          fragment_shader.to_str().unwrap().blue().underline()
        );
      }

      println!("");
      println!(
        "When creating a shader, you need to create both the vertex and fragment shader files."
      );
      println!("For example, if you create a shader file called {}, you also need to create a file called {}.", "example.vert".cyan(), "example.frag".cyan());
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
      return;
    }

    print!("{}", "[INFO]\t".green());

    print!(
      "Types generated for the shader file: {}",
      file_path.to_str().unwrap().blue().underline()
    );
    // Measure the time it takes to generate the types
    let start = std::time::Instant::now();
    generate_types(&vertex_shader, &output_folder, &language);
    println!(
      " {}",
      format!("({:?})", start.elapsed()).truecolor(130, 130, 130)
    );
  });

  let mut watcher = notify::recommended_watcher(move |res: Result<Event>| match res {
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

fn generate_types(input: &std::path::PathBuf, output: &std::path::PathBuf, language: &str) {
  if language == "ts" {
    type_script::generate_ts_types_file(input, output);
  } else {
    panic!("Unsupported language: {}", language);
  }
}
