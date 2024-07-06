use std::{fs, io::Write};

use anyhow::Context;
use clap::Parser;

mod command;
mod processing;

fn main() -> anyhow::Result<()> {
    let cli = command::Cli::parse();

    print!("Reading original image ... ");
    let original_image = image::open(&cli.original_image)
        .with_context(|| "Failed to load the original image.")?
        .into_rgba8();
    println!("OK");

    // Creates output directory for processed images.
    fs::create_dir_all(&cli.output_dir)
        .with_context(|| "Failed to create output directory for processed images.")?;

    let opaque_image = processing::create_opaque_image(&original_image);
    print!("Saving opaque image ... ");
    std::io::stdout().flush().unwrap();
    opaque_image
        .save(cli.output_dir.join("opaque.png"))
        .with_context(|| "Failed to save the opaque image.")?;
    println!("OK");

    let header_image = processing::create_header_image(&original_image);
    print!("Saving header image ... ");
    std::io::stdout().flush().unwrap();
    header_image
        .save(cli.output_dir.join("header.png"))
        .with_context(|| "Failed to save processed image for header.")?;
    println!("OK");

    Ok(())
}
