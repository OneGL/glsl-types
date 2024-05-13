mod generator;
mod uniforms;

use clap::Parser;
use notify::{RecursiveMode, Result, Watcher, Event};

const DEFAULT_INPUT_FOLDER: &str = "shaders/example.vert";
const DEFAULT_OUTPUT_FOLDER: &str = "shaders";

/// This program reads a glsl file and generates TS types for the uniforms
#[derive(Parser)]
#[command()]
struct Args {
    /// Input folder with glsl files
    #[arg(short, long, default_value = DEFAULT_INPUT_FOLDER)]
    input: std::path::PathBuf,

    /// Output folder for the generated types
    #[arg(short, long, default_value = DEFAULT_OUTPUT_FOLDER)]
    output: std::path::PathBuf,

    /// Output language
    /// Supported languages: ts, rs
    /// Default: ts
    #[arg(short, long, default_value = "ts")]
    language: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.input.exists() {
        if args.input.to_str().unwrap() == DEFAULT_INPUT_FOLDER {
            std::fs::create_dir_all(&args.input).unwrap();
        } else {
            panic!("Input folder does not exist");
        }
    }

    if !args.output.exists() {
        if args.output.to_str().unwrap() == DEFAULT_OUTPUT_FOLDER {
            std::fs::create_dir_all(&args.output).unwrap();
        } else {
            panic!("Output folder does not exist");
        }
    }

    let input = args.input.clone();
    let output = args.output.clone();
    let language = args.language.clone();

    let mut watcher = notify::recommended_watcher(move |res: Result<Event>| match res {
        Ok(event) => {
            println!("");
            println!("Detected change inside the shader file: {:?}", event.paths.first().unwrap().to_str().unwrap());
            generate_types(&input, &output, &language);
            println!("Types generated for the shader file");
        }
        Err(e) => {
            println!("watch error: {:?}", e);
        }
    })?;

    watcher.watch(&args.input, RecursiveMode::Recursive)?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn generate_types(input: &std::path::PathBuf, output: &std::path::PathBuf, language: &str) {
    if language == "ts" {
        generator::type_script::generate_ts_types_file(input, output);
    } else if language == "rs" {
        generator::rust::generate_rs_types_file(input, output);
    } else {
        panic!("Unsupported language: {}", language);
    }
}
