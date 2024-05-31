import fs from "fs";
import path from "path";
import chalk from "chalk";
import glsl_types from "./pkg/glsl_types.js";
import { program } from "commander";

global.logln = (message) => console.log(message);
global.log = (message) => process.stdout.write(message);
global.log_with_color = (message, color) => {
  process.stdout.write(chalk[color](message));
};
global.read_file = (file) => fs.readFileSync(file, "utf8");
global.canonicalize = (file) => path.resolve(file);
global.file_exists = (file) => fs.existsSync(file);
global.create_dir_all = (dir) => fs.mkdirSync(dir, { recursive: true });
global.write_file = (file, content) => fs.writeFileSync(file, content);

program
  .option("-i, --input <input>", "Input directory", "./shaders")
  .option("-o, --output <output>", "Output directory", "./output")
  .option("-w, --watch", "Watch for changes", false);

program.parse();

const SHADER_EXTENSIONS = [".vert", ".frag", ".vs", ".fs", ".glsl"];

const options = program.opts();

if (options.watch) {
  process.stdout.write(chalk["green"]("Watching for changes\n"));
  fs.watch(options.input, { recursive: true }, (eventType, filename) => {
    if (!filename) return;

    if (SHADER_EXTENSIONS.includes(path.extname(filename))) {
      glsl_types.start_cli(
        "/home/luis/github/onegl/glsl-types/shaders/program1.vert",
        options.input,
        options.output
      );
    }
  });
} else {
  glsl_types.start_cli(
    "/home/luis/github/onegl/glsl-types/shaders/program1.vert",
    options.input,
    options.output
  );
}
