pub mod image_resizer;

use clap::Parser;
use core::panic;

use crate::image_resizer::{
    get_default_command, get_default_interpolation_method, validate_commands, validate_methods,
    ImageResizer,
};

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value=get_default_command())]
    command: String,
    #[arg(short, long)]
    path: std::path::PathBuf,
    #[arg(short, long, default_value=get_default_interpolation_method())]
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

    if !validate_commands(&command) {
        panic!("Not allowed command {:?}", command)
    }

    if !validate_methods(&algo) {
        panic!("Not allowed algorithm {:?}", algo)
    };

    let resizer = ImageResizer {
        image_path: args.path,
    };

    resizer.resize(
        command.as_str(),
        algo.as_str(),
        args.width,
        args.height,
        args.times,
    )
}
