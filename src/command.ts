import fs from "fs";

export function extractOptions(): {
  inputFolder: string;
  outputFolder: string;
} {
  const options = process.argv.slice(2);

  // Extract the input folder from the command line arguments --input
  const inputFolderIndex = options.indexOf("--input");

  if (inputFolderIndex === -1) {
    throw new Error("Input folder not provided");
  }

  const inputFolder = options[inputFolderIndex + 1];

  // Extract the output folder from the command line arguments --output
  const outputFolderIndex = options.indexOf("--output");

  if (outputFolderIndex === -1) {
    throw new Error("Output folder not provided");
  }

  const outputFolder = options[outputFolderIndex + 1];

  // Check if the input folder is provided
  if (!inputFolder) {
    throw new Error("Input folder not provided");
  }

  // Check if the output folder is provided
  if (!outputFolder) {
    throw new Error("Output folder not provided");
  }

  // Validate the input folder
  if (!fs.existsSync(inputFolder)) {
    throw new Error("Input folder does not exist");
  }

  // Validate the output folder
  if (!fs.existsSync(outputFolder)) {
    throw new Error("Output folder does not exist");
  }

  return {
    inputFolder,
    outputFolder,
  };
}
