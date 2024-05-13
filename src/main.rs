mod generator;
mod uniforms;

use clap::Parser;

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

fn main() {
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

    if args.language == "ts" {
        generator::type_script::generate_ts_types_file(args.input, args.output);
    } else if args.language == "rs" {
        generator::rust::generate_rs_types_file(args.input, args.output);
    } else {
        panic!("Unsupported language: {}", args.language);
    }
}
