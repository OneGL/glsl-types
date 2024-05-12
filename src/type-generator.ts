import fs from "fs";
import path from "path";
import { uniformTypes, uniformValueMap } from "./uniform-types";
import type { Uniform, UnifromType } from "./uniform-types";
import { upperCaseFirstLetter } from "./utils";

export function generateTypes({
  filename,
  outputFolder,
}: {
  filename: string;
  outputFolder: string;
}): void {
  const uniforms = extractUniforms(filename);
  const fileNameNoExtension = path.parse(filename).name;

  if (!uniforms) {
    return;
  }

  let result = "";

  // Create a TypeScript uniform type for each uniform
  for (const uniform of uniforms) {
    result += `type ${uniform.name} = ${uniformValueMap[uniform.type]};\n`;
  }

  // Create a type with the name of the file without the extension
  // and with the uniforms as properties
  result += `\n
export type ${upperCaseFirstLetter(fileNameNoExtension)} = {
    uniforms: {
${uniforms
  .map((uniform) => `     ${uniform.name}: ${uniform.name};`)
  .join("\n")}
    };
};
`;

  const outputFileName = `${fileNameNoExtension}.ts`;
  const outputPath = path.join(outputFolder, outputFileName);
  fs.writeFileSync(outputPath, result);
}

function extractUniforms(filename: string): Uniform[] | undefined {
  const file = fs.readFileSync(`./shaders/${filename}`, "utf-8");
  const uniforms = file.match(/uniform\s+\w+\s+\w+\s*;/g);

  if (!uniforms) {
    return;
  }

  return uniforms.map((uniform) => {
    const words = uniform.split(" ");

    let uniformType = words[1];
    let uniformName = words[2];

    if (uniformName.endsWith(";")) {
      uniformName = uniformName.slice(0, -1);
    }

    if (!uniformTypes.includes(uniformType)) {
      throw new Error(`Unknown uniform type: ${uniformType}`);
    }

    return {
      // After the previous check, we can safely cast type to UnifromType
      type: uniformType as UnifromType,
      name: uniformName,
    };
  });
}
