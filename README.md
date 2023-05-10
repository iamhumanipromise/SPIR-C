# SPIR-C
**SPIR-C is a software toolchain that enables the conversion of OpenCL 3.0 source code into GLSL 1.10 code, which can then be executed on OpenGL 1.4 GPUs. The toolchain consists of several modules, including ShaderRipper, which is responsible for converting GLSL 4.5 code into SPIR-V binary, and SPIR-V-Cross, which is used to convert SPIR-V binary into GLSL 1.10.

## Steps and Workflow

Here's a brief overview of the SPIR-C workflow:

- Preprocessing: The GLSL code is preprocessed to remove any unsupported OpenCL 3.0 extensions and translate any unsupported syntax to compatible syntax.
- Compilation: The preprocessed GLSL code is then compiled to SPIR-V using the shaderc library.
- Optimization: The SPIR-V code is then optimized using the LLVM library to improve performance.
- Execution: Finally, the optimized SPIR-V code can be executed on a GPU using OpenGL.

OpenCL 3.0 source code → Shaderc (GLSL 4.5) → glslang (GLSL 4.5) → SPIR-V-Cross (SPIR-V binary) → GLSL 1.10 code

```ASCII
+-----------------------------------------------------+
|                       Input                         |
|-----------------------------------------------------|
| OpenCL 3.0 source code                              |
| OpenGL 1.4 compatible hardware and drivers          |
+-----------------------------------------------------+
                           |
                           |
                           v
+-----------------------------------------------------+
|                  Preprocessing                      |
|-----------------------------------------------------|
| Identify unsupported OpenCL 3.0 extensions           |
| Convert OpenCL source code to GLSL 4.5 using         |
| shaderc and glslang                                  |
| Optimize GLSL 4.5 code using glm/cglm/clgl libraries |
+-----------------------------------------------------+
                           |
                           |
                           v
+-----------------------------------------------------+
|              Intermediate Representation             |
|-----------------------------------------------------|
| Convert optimized GLSL 4.5 code to SPIR-V binary     |
| using spirv-cross                                    |
| Optimize SPIR-V code using glm/cglm/clgl libraries   |
+-----------------------------------------------------+
                           |
                           |
                           v
+-----------------------------------------------------+
|                     Translation                      |
|-----------------------------------------------------|
| Convert optimized SPIR-V binary to GLSL 1.10 code    |
+-----------------------------------------------------+
                           |
                           |
                           v
+-----------------------------------------------------+
|                       Output                        |
|-----------------------------------------------------|
| Valid and optimized GLSL 1.10 code                   |
| that can be executed on OpenGL 1.4 compatible        |
| hardware                                             |
+-----------------------------------------------------+`
```

## Installation

Installation is not yet possible. This is a proof-of concept and requires much more work to be fully functional and to be able to be integrated into your specific use case. 

## License

SPIR-C is released under the Apache 2.0 License. See the LICENSE file for more information.

## Contributing

Community Contributions are highly welcome. 
