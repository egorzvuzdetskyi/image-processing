use std::path::PathBuf;
pub mod commands;
pub mod interpolation_methods;
pub mod traits;

use image::{DynamicImage, GenericImageView};

use commands::Commands;
use interpolation_methods::InterpolationMethod;
use traits::StrEnum;

pub struct ImageResizer {
    pub image_path: PathBuf,
}

impl ImageResizer {
    fn get_file_extension(&self) -> &str {
        return self
            .image_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap();
    }

    fn get_file_name(&self) -> &str {
        return self
            .image_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap();
    }

    fn read_image(&self) -> DynamicImage {
        return image::open(&self.image_path).unwrap();
    }

    pub fn resize(
        &self,
        command: &str,
        method: &str,
        width: Option<u32>,
        height: Option<u32>,
        times: Option<u32>,
    ) {
        match command {
            s if s == Commands::Resize.as_str() => {
                self.resize_internal(
                    width.unwrap_or_else(|| panic!("Width field required for resize command")),
                    height.unwrap_or_else(|| panic!("Height field required for resize command")),
                    InterpolationMethod::from_str(&method).unwrap(),
                );
                std::process::exit(0);
            }

            s if s == Commands::Upscale.as_str() => {
                self.upscale(
                    times.unwrap_or_else(|| panic!("times field required for upscale command")),
                    InterpolationMethod::from_str(&method).unwrap(),
                );
                std::process::exit(0);
            }

            s if s == Commands::Downscale.as_str() => {
                self.downscale(
                    times.unwrap_or_else(|| panic!("times field required for downscale command")),
                    InterpolationMethod::from_str(&method).unwrap(),
                );
                std::process::exit(0);
            }

            _ => panic!(
                "Not allowed command {command}. allowd commands: [resize, upscale, downscale]"
            ),
        }
    }

    pub fn resize_internal(&self, width: u32, height: u32, method: InterpolationMethod) {
        println!(
            "I will resize image by path: {:?}, for width: {:?}, height: {:?}, using method: {:?}",
            &self.image_path,
            width,
            height,
            method.as_str()
        );

        let image = &self.read_image();
        let new_image = image.resize(width, height, method.to_external_image_methods());
        let file_extension = self.get_file_extension();
        let image_path_wo_extension = self.get_file_name();

        let result_path =
            format!("{image_path_wo_extension}_{width}_{height}_{method}_resized.{file_extension}");

        new_image.save(result_path).unwrap();
    }

    pub fn upscale(&self, times: u32, method: InterpolationMethod) {
        let image = &self.read_image();
        let (width, height) = image.dimensions();
        let (new_width, new_height) = (width * times, height * times);

        self.resize_internal(new_width, new_height, method);
    }

    pub fn downscale(&self, times: u32, method: InterpolationMethod) {
        let image = &self.read_image();
        let (width, height) = image.dimensions();
        let (new_width, new_height) = (width / times, height / times);

        self.resize_internal(new_width, new_height, method);
    }
}

pub fn get_default_interpolation_method() -> &'static str {
    return InterpolationMethod::Nearest.as_str();
}

pub fn get_default_command() -> &'static str {
    return Commands::Resize.as_str();
}

pub fn validate_commands(command: &String) -> bool {
    return Commands::matches_str(command);
}

pub fn validate_methods(method: &String) -> bool {
    return InterpolationMethod::matches_str(method);
}
