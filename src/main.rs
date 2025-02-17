pub mod commands;
pub mod image_resizer;
pub mod interpolation_methods;

use clap::Parser;
use core::panic;

use crate::commands::Commands;
use crate::image_resizer::ImageResizer;
use crate::interpolation_methods::{InterpolationMethod, StrEnum};

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value=Commands::Resize.as_str())]
    command: String,
    #[arg(short, long)]
    path: std::path::PathBuf,
    #[arg(short, long, default_value=InterpolationMethod::Nearest.as_str())]
    algorithm: String,
    #[arg(long)]
    width: Option<u32>,
    #[arg(long)]
    height: Option<u32>,
    #[arg(long)]
    times: Option<u32>,
}

fn main() {
    let args = Cli::parse();

    let command = args.command;
    let algo = args.algorithm;

    if !Commands::matches_str(&command) {
        panic!("Not allowed command {:?}", command)
    }

    if !InterpolationMethod::matches_str(&algo) {
        panic!("Not allowed algorithm {:?}", algo)
    };

    let resizer = ImageResizer {
        image_path: args.path,
    };

    match command.as_str() {
        s if s == Commands::Resize.as_str() => {
            resizer.resize(
                args.width
                    .unwrap_or_else(|| panic!("Width field required for resize command")),
                args.height
                    .unwrap_or_else(|| panic!("Height field required for resize command")),
                InterpolationMethod::from_str(&algo).unwrap(),
            );
            std::process::exit(0);
        }

        s if s == Commands::Upscale.as_str() => {
            resizer.upscale(
                args.times
                    .unwrap_or_else(|| panic!("times field required for upscale command")),
                InterpolationMethod::from_str(&algo).unwrap(),
            );
            std::process::exit(0);
        }

        s if s == Commands::Downscale.as_str() => {
            resizer.downscale(
                args.times
                    .unwrap_or_else(|| panic!("times field required for downscale command")),
                InterpolationMethod::from_str(&algo).unwrap(),
            );
            std::process::exit(0);
        }

        _ => panic!("Not allowed command {command}. allowd commands: [resize, upscale, downscale]"),
    }
}
