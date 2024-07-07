use std::{fs, io::Write};

use anyhow::{bail, Context};
use clap::Parser;

mod command;
mod config;
mod processing;

fn main() -> anyhow::Result<()> {
    let cli = command::Cli::parse();

    let config::Config { settings } = match &cli.config {
        Some(config_path) => {
            let config_contents = fs::read_to_string(config_path)
                .with_context(|| "Failed to load configuration file.")?;
            toml::from_str(&config_contents).with_context(|| "Failed to setup configuration.")?
        }
        None => config::Config::default(),
    };

    let filter = match settings.filter.as_str() {
        "Nearest" => image::imageops::FilterType::Nearest,
        "Triangle" => image::imageops::FilterType::Triangle,
        "CatmullRom" => image::imageops::FilterType::CatmullRom,
        "Gaussian" => image::imageops::FilterType::Gaussian,
        "Lanczos3" => image::imageops::FilterType::Lanczos3,
        filter => bail!(format!("`{}` is an unknown filter type.", filter)),
    };

    print!("Reading original image ... ");
    let original_image = image::open(&cli.original_image)
        .with_context(|| "Failed to load the original image.")?
        .into_rgba8();
    println!("OK");

    // Creates output directory for processed images.
    fs::create_dir_all(&cli.output_dir)
        .with_context(|| "Failed to create output directory for processed images.")?;

    if !cli.header_only {
        let opaque_image = processing::create_opaque_image(&original_image);
        print!("Saving opaque image ... ");
        std::io::stdout().flush().unwrap();
        opaque_image
            .save(cli.output_dir.join(&settings.opaque_image))
            .with_context(|| "Failed to save the opaque image.")?;
        println!("OK");
    }

    let header_image = processing::create_header_image(&original_image, filter);
    print!("Saving header image ... ");
    std::io::stdout().flush().unwrap();
    header_image
        .save(cli.output_dir.join(&settings.header_image))
        .with_context(|| "Failed to save processed image for header.")?;
    println!("OK");

    Ok(())
}
