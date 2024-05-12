export const uniformValueMap = {
  sampler2d: "WebGLTexture",

  float: "number",
  vec2: "[number, number]",
  vec3: "[number, number, number]",
  vec4: "[number, number, number, number]",

  int: "number",
  ivec2: "[number, number]",
  ivec3: "[number, number, number]",
  ivec4: "[number, number, number, number]",

  uint: "number",
  uvec2: "[number, number]",
  uvec3: "[number, number, number]",
  uvec4: "[number, number, number, number]",

  bool: "boolean",
  bvec2: "[boolean, boolean]",
  bvec3: "[boolean, boolean, boolean]",
  bvec4: "[boolean, boolean, boolean, boolean]",

  mat2: "[number, number, number, number]",
  mat3: "[number, number, number, number, number, number, number, number, number]",
  mat4: `[
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number
    ]`,
};

export const uniformTypes = Object.keys(uniformValueMap);
export type UnifromType = keyof typeof uniformValueMap;
export type Uniform = {
  type: UnifromType;
  name: string;
};
