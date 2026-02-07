# resvg-jpeg

![Build Status](https://github.com/nicupavel/resvg-jpeg/actions/workflows/main.yml/badge.svg)
![Tests](https://img.shields.io/github/actions/workflow/status/nicupavel/resvg-jpeg/main.yml?label=Tests)

A command-line tool to convert SVG files to JPEG images using resvg and tiny-skia.

## Features

- Configurable output dimensions (width).
- Configurable JPEG quality.
- Configurable background color (default: white) to deal with transparent backgrounds.
- Supports custom font directories.

## Installation

### From Source

```bash
cargo install --path .
```

## Usage

```bash
resvg-jpeg [OPTIONS]
```

### Options

- `-i, --input <PATH>`: Input SVG file path. If not provided, reads from Stdin.
- `-o, --output <PATH>`: Output JPEG file path. If not provided, writes to Stdout.
- `-w, --width <WIDTH>`: Target width (maintains aspect ratio). If not set, uses original size.
- `-q, --quality <QUALITY>`: JPEG Quality (1-100). Default: 80.
- `-b, --background <COLOR>`: Background color (e.g., "white", "#FF0000"). Default: "white".
- `--use-fonts-dir <PATH>`: Directory to load custom fonts from.

### Examples

Convert a file with default settings:

```bash
resvg-jpeg -i input.svg -o output.jpg
```

Convert with custom width and quality:

```bash
resvg-jpeg -i input.svg -o output.jpg -w 800 -q 90
```

Convert with a red background:

```bash
resvg-jpeg -i input.svg -o output.jpg -b "#FF0000"
```

Read from Stdin and write to Stdout:

```bash
cat input.svg | resvg-jpeg > output.jpg
```

## License

This project is licensed under the MIT License.
