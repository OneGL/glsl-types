# GLSL-Types

GLSL-Types is a lightweight library designed to generate TypeScript types for GLSL (OpenGL Shading Language) shaders. By leveraging the [glsl](https://docs.rs/glsl/latest/glsl/) crate, this library simplifies the integration between GLSL and TypeScript, enabling developers to safely and efficiently use GLSL code within TypeScript projects.

### Features

- Automatic Type Generation: Automatically generate TypeScript types based on GLSL shader code.
- Type Safety: Helps prevent runtime errors by ensuring type correctness when working with shaders in TypeScript.
- Easy Integration: Seamlessly integrates with GLSL shaders, making it easy to use in projects requiring WebGL or other GLSL-based graphics rendering.

### Installation

```bash
npm install
npm run build
```

## How to use

```bash
node dist/index.js -f ./shaders/test.vert
```

### Contributing

Contributions are welcome! If you'd like to contribute to the project, feel free to open an issue or submit a pull request on GitHub.
