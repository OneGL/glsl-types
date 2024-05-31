import fs from "fs";
import path from "path";
import chalk from "chalk";
import { program } from "commander";
// @ts-ignore
import * as glslTypes from "./pkg/glsl_types.cjs";

// @ts-ignore
global.logln = (message) => console.log(message);
// @ts-ignore
global.log = (message) => process.stdout.write(message);
// @ts-ignore
global.log_with_color = (message, color) => {
  // @ts-ignore
  process.stdout.write(chalk[color](message));
};
// @ts-ignore
global.read_file = (file) => fs.readFileSync(file, "utf8");
// @ts-ignore
global.canonicalize = (file) => path.resolve(file);
// @ts-ignore
global.file_exists = (file) => fs.existsSync(file);
// @ts-ignore
global.create_dir_all = (dir) => fs.mkdirSync(dir, { recursive: true });
// @ts-ignore
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
      glslTypes.start_cli(
        "/home/luis/github/onegl/glsl-types/shaders/program1.vert",
        options.input,
        options.output
      );
    }
  });
} else {
  glslTypes.start_cli(
    "/home/luis/github/onegl/glsl-types/shaders/program1.vert",
    options.input,
    options.output
  );
}
