use std::{fs, io::Write};

use clap::Parser;

mod command;
mod processing;

fn main() -> anyhow::Result<()> {
    let cli = command::Cli::parse();

    print!("Reading original image ... ");
    let original_image = image::open(&cli.original_image)?.into_rgba8();
    println!("OK");

    // Creates output directory for processed images.
    fs::create_dir_all(&cli.output_dir)?;

    let opaque_image = processing::create_opaque_image(&original_image);
    print!("Saving opaque image ... ");
    std::io::stdout().flush().unwrap();
    opaque_image.save(cli.output_dir.join("opaque.png"))?;
    println!("OK");

    let header_image = processing::create_header_image(&original_image);
    print!("Saving header image ... ");
    std::io::stdout().flush().unwrap();
    header_image.save(cli.output_dir.join("header.png"))?;
    println!("OK");

    Ok(())
}
