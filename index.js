const fs = require("fs");
const path = require("path");

const glsl_types = require("./pkg/glsl_types.js");

global.log = (message) => console.log(message);
global.read_file = (file) => fs.readFileSync(file, "utf8");
global.canonicalize = (file) => path.resolve(file);

const result = glsl_types.resolve_imports(
  "/home/luis/github/onegl/glsl-types/shaders/program1.vert",
  "/home/luis/github/onegl/glsl-types/shaders"
);
console.log(result);

