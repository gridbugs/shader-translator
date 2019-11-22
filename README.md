# Shader Translator

This is a binary wrapper of [shaderc](https://crates.io/crates/shaderc) which compiles
GLSL shader source code read from stdin, into SPIR-V source code written to stdout.

## Usage
```bash
$ ./shader-translator --help
Usage: ./shader-translator [options]

Options:
    -f, --fragment      fragment shader
    -v, --vertex        vertex shader
    -h, --help          print this help menu
```

## Example

### Basic
```bash
$ echo -e "#version 450\nvoid main() {}" | ./shader-translator --fragment > /tmp/frag.spv
$ file /tmp/frag.spv
/tmp/frag.spv: Khronos SPIR-V binary, little-endian, version 0x00010000, generator 0x000d0007
```
