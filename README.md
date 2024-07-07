Converts the specified image to an image that fits the X (formerly Twitter) header[^1].

The following two images are generated.
- An image with opacity only.
- An image that is not only opacified but also resized and margins are added to fit the X (formerly Twitter) header.

The default file names are `opaque.png` and `header.png` respectively.
The settings can be changed by specifying the configuration file described bellow.

# Installation

Install from GitHub with the following command.

```sh
cargo install --git https://github.com/atcoder8/rating-graph-processor
```

# Usage

Specify the original image and the output directory as arguments.

The help message when you run `rating-graph-processor -h` is as follows.

```
Converts the specified image to an image that fits the X (formerly Twitter) header

Usage: rating-graph-processor [OPTIONS] <ORIGINAL_IMAGE> <OUTPUT_DIR>

Arguments:
  <ORIGINAL_IMAGE>  Original image
  <OUTPUT_DIR>      Output directory for processed images

Options:
  -c, --config <CONFIG>  Configuration file
      --header-only      If this flag is specified, only images processed for headers will be generated. An image that has only been opaque will not be generated
  -h, --help             Print help
  -V, --version          Print version
```

# Configuration

The following settings are available.

| Item         | Description                                                    | Default    |
| ------------ | -------------------------------------------------------------- | ---------- |
| opaque_image | Filename of the image with opaque processing only.             | opaque.png |
| header_image | Filename of the processed for the X (formerly Twitter) header. | header.png |
| filter       | Type of sampling filter to use when resizing the image.        | CatmullRom |

You can specify a configuration file using the `--config` option.
The configuration file is written in TOML format, and each setting is written in the `settings` table.

The following is an example of a configuration file.

```toml
[settings]

# Filename of the image with opaque processing only.
# Default: opaque.png
opaque_image = "opaque.png"

# Filename of the processed for the X (formerly Twitter) header.
# Default: header.png
header_image = "header.png"

# Type of sampling filter to use when resizing the image.
# Available filters: [Nearest, Triangle, CatmullRom, GAussian, Lanczos3]
# Default: CatmullRom
filter = "CatmullRom"
```

[^1]: The header image size is 1500x500.
