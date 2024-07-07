use std::path::PathBuf;

use clap::Parser;

/// Converts the specified image to an image that fits the X (formerly Twitter) header.
#[derive(Debug, Parser)]
#[command(version, about)]
pub(crate) struct Cli {
    /// Original image.
    pub(crate) original_image: PathBuf,

    /// Output directory for processed images.
    pub(crate) output_dir: PathBuf,

    /// Configuration file.
    #[arg(short, long)]
    pub(crate) config: Option<PathBuf>,

    /// If this flag is specified, only images processed for headers will be generated.
    /// An image that has only been opaque will not be generated.
    #[arg(long)]
    pub(crate) header_only: bool,
}
