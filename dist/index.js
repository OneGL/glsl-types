#!/usr/bin/env node
import fs from "fs";
import path from "path";
import chalk from "chalk";
import { program } from "commander";
// @ts-ignore
import * as glslTypes from "./pkg/glsl_types.cjs";
global.logln = function (message) { return console.log(message); };
global.log = function (message) { return process.stdout.write(message); };
global.log_with_color = function (message, color) {
    // @ts-ignore
    process.stdout.write(chalk[color](message));
};
global.read_file = function (file) { return fs.readFileSync(file, "utf8"); };
global.canonicalize = function (file) { return path.resolve(file); };
global.file_exists = function (file) { return fs.existsSync(file); };
global.create_dir_all = function (dir) { return fs.mkdirSync(dir, { recursive: true }); };
global.write_file = function (file, content) { return fs.writeFileSync(file, content); };
program
    .option("-i, --input <input>", "Input directory", "./shaders")
    .option("-o, --output <output>", "Output directory", "./output")
    .option("-f, --file <file>", "File to process")
    .option("-w, --watch", "Watch for changes", false);
program.parse();
var SHADER_EXTENSIONS = [".vert", ".frag", ".vs", ".fs"];
var options = program.opts();
if (options.watch) {
    process.stdout.write(chalk.green("Watching for changes\n"));
    fs.watch(options.input, { recursive: true }, function (eventType, filename) {
        console.log("File change detected");
        if (!filename)
            return;
        filename = path.resolve(options.input, filename);
        if (SHADER_EXTENSIONS.includes(path.extname(filename))) {
            var start = performance.now();
            glslTypes.start_cli(filename, options.input, options.output);
            var end = performance.now();
            process.stdout.write(chalk.green("[INFO]\t"));
            process.stdout.write("File processed ".concat(chalk.blue(path.relative(options.input, filename))));
            process.stdout.write(chalk.gray(" (".concat((end - start).toFixed(2), "ms)\n")));
        }
    });
}
else {
    if (!options.file) {
        console.error("Please provide a file to process");
        process.exit(1);
    }
    if (!fs.existsSync(options.file)) {
        console.error("File ".concat(options.file, " does not exist"));
        process.exit(1);
    }
    glslTypes.start_cli(options.file, options.input, options.output);
}
