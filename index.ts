import fs from "fs";
import path from "path";
import chalk from "chalk";
import glsl_types from "./pkg/glsl_types.js";

global.console_log = (message) => console.log(message);
global.log_with_color = (message, color) => {
  process.stdout.write(chalk[color](message));
};
global.read_file = (file) => fs.readFileSync(file, "utf8");
global.canonicalize = (file) => path.resolve(file);
global.file_exists = (file) => fs.existsSync(file);
global.create_dir_all = (dir) => fs.mkdirSync(dir, { recursive: true });
global.write_file = (file, content) => fs.writeFileSync(file, content);

glsl_types.start_cli(
  "/home/luis/github/onegl/glsl-types/shaders/program1.vert",
  "./shaders",
  "./output2"
);
