use std::path::PathBuf;

use clap::Parser;

/// Adds a white background to convert the image to opaque.
/// It also generates an image scaled to fit the size of the X(formerly Twitter) header image.
#[derive(Debug, Parser)]
#[command(version, about)]
pub(crate) struct Cli {
    /// Original image.
    pub(crate) original_image: PathBuf,

    /// Output directory for processed images.
    pub(crate) output_dir: PathBuf,
}
