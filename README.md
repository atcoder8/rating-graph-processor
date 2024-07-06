Adds a white background to the rating graph image, making it opaque.
It also generates an image scaled to fit the size of the X (formerly Twitter) header image.

# Installation

Install from GitHub with the following command.

```sh
cargo install --git https://github.com/atcoder8/rating-graph-processor
```

# Usage

Specify the original image and the output directory as arguments.

The help message when you run `rating-graph-processor -h` is as follows.

```
Adds a white background to convert the image to opaque. It also generates an image scaled to fit the size of the X(formerly Twitter) header image

Usage: rating-graph-processor <ORIGINAL_IMAGE> <OUTPUT_DIR>

Arguments:
  <ORIGINAL_IMAGE>  Original image
  <OUTPUT_DIR>      Output directory for processed images

Options:
  -h, --help     Print help
  -V, --version  Print version
```
