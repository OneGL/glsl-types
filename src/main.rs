mod generator;
mod uniforms;

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

fn main() -> Result<()> {
    let args = Args::parse();

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

    let mut watcher = notify::recommended_watcher(move |res: Result<Event>| match res {
        Ok(event) => {
            // Clear the screen
            print!("\x1B[2J\x1B[1;1H");

            let file_path = event.paths.first().unwrap();
            let file_extension = file_path.extension().unwrap();

            // Ignore files that do not end with .vert or .frag extension
            if file_extension != "vert" && file_extension != "frag" {
                return;
            }

            // The user should create both the vertex and fragment shader files
            // if the user only creates one of them, we will show an error message
            // and ignore the file.
            let file_stem = file_path.file_stem().unwrap().to_str().unwrap().to_string();
            let vertex_shader = input_folder.join(file_stem.clone() + ".vert");
            let fragment_shader = input_folder.join(file_stem.clone() + ".frag");

            if !vertex_shader.exists() || !fragment_shader.exists() {
                print!("{}", "[ERROR] ".red());
                println!("Missing shader files.");
                println!("");

                if !vertex_shader.exists() {
                    println!(
                        "Please create a vertex shader file: {}",
                        vertex_shader.to_str().unwrap().cyan()
                    );
                } else {
                    println!(
                        "Please create a fragment shader file: {}",
                        fragment_shader.to_str().unwrap().cyan()
                    );
                }

                println!("");
                println!("When creating a shader, you need to create both the vertex and fragment shader files.");
                println!("For example, if you create a shader file called {}, you also need to create a file called {}.", "program1.vert".cyan(), "program1.frag".cyan());
                println!("");
                println!("Example:");
                println!("");
                println!("├── shaders");
                println!("│   ├── {}.vert", "program1".cyan());
                println!("│   └── {}.frag", "program1".cyan());
                println!("│");
                println!("├── output");
                println!("    └── {}.ts", "program1".cyan());
                println!("");
                return;
            }

            println!("");
            print!("{}", "[INFO] ".green());
            println!(
                "Detected change inside the shader file: {:?}",
                file_path.to_str().unwrap()
            );
            generate_types(&vertex_shader, &output_folder, &language);
            println!("Types generated for the shader file");
        }
        Err(e) => {
            println!("watch error: {:?}", e);
        }
    })?;

    watcher.watch(&args.input_float, RecursiveMode::Recursive)?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn generate_types(input: &std::path::PathBuf, output: &std::path::PathBuf, language: &str) {
    if language == "ts" {
        generator::type_script::generate_ts_types_file(input, output);
    } else {
        panic!("Unsupported language: {}", language);
    }
}
