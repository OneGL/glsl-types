#!/usr/bin/env node

import fs from "fs";
import path from "path";
import chalk from "chalk";
import { program } from "commander";
// @ts-ignore
import * as glslTypes from "./pkg/glsl_types.cjs";

global.logln = (message) => console.log(message);
global.log = (message) => process.stdout.write(message);
global.log_with_color = (message, color) => {
  // @ts-ignore
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
  .option("-f, --file <file>", "File to process")
  .option("-w, --watch", "Watch for changes", false);

program.parse();

const SHADER_EXTENSIONS = [".vert", ".frag", ".vs", ".fs"];

const options = program.opts();

if (options.watch) {
  process.stdout.write(chalk.green("Watching for changes\n"));
  fs.watch(options.input, { recursive: true }, (eventType, filename) => {
    console.log("File change detected");
    if (!filename) return;
    filename = path.resolve(options.input, filename);

    if (SHADER_EXTENSIONS.includes(path.extname(filename))) {
      const start = performance.now();
      glslTypes.start_cli(filename, options.input, options.output);
      const end = performance.now();

      process.stdout.write(chalk.green("[INFO]\t"));
      process.stdout.write(
        `File processed ${chalk.blue(path.relative(options.input, filename))}`
      );
      process.stdout.write(chalk.gray(` (${(end - start).toFixed(2)}ms)\n`));
    }
  });
} else {
  if (!options.file) {
    console.error("Please provide a file to process");
    process.exit(1);
  }

  if (!fs.existsSync(options.file)) {
    console.error(`File ${options.file} does not exist`);
    process.exit(1);
  }

  glslTypes.start_cli(options.file, options.input, options.output);
}

declare global {
  var logln: (message: string) => void;
  var log: (message: string) => void;
  var log_with_color: (message: string, color: string) => void;
  var read_file: (file: string) => string;
  var canonicalize: (file: string) => string;
  var file_exists: (file: string) => boolean;
  var create_dir_all: (dir: string) => void;
  var write_file: (file: string, content: string) => void;
}
