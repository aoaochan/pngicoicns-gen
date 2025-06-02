# rswasm-icongen

This project is a WebAssembly-based tool for converting images into icons. It transforms input images into various sizes of PNG, ICO, and ICNS files, generating icons in a rounded square shape.

## Features

- Converts images into various sizes (16x16, 32x32, 64x64, 128x128, 256x256, 512x512) of icons.
- Supports PNG, ICO, and ICNS formats.
- Generates icons in a rounded square shape.

## Usage

1. Provide an image file as input.
2. Use the WebAssembly module to convert the image into icons.
3. Download or use the generated icon files.

## Build Instructions

### Requirements

- Rust
- wasm-pack
- Node.js

### Build Commands

```bash
# Build in development mode
./dev.bat

# Build in production mode
./build.bat
```

## License

This project is licensed under the MIT License.
