import fs from "fs";
import { generateTypes } from "./type-generator";
import { extractOptions } from "./command";

const { inputFolder, outputFolder } = extractOptions();

fs.watch(inputFolder, (_, filename) => {
  if (!filename) return;

  if (filename.endsWith(".vert")) {
    console.log(`Detected change in ${filename}`);
    generateTypes({ filename, outputFolder });
    console.log(`Generated types for ${filename}`);
  }
});
