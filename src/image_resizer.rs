use std::path::PathBuf;

use image::{DynamicImage, GenericImageView};

use crate::interpolation_methods::InterpolationMethod;

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

    pub fn resize(&self, width: u32, height: u32, method: InterpolationMethod) {
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

        self.resize(new_width, new_height, method);
    }

    pub fn downscale(&self, times: u32, method: InterpolationMethod) {
        let image = &self.read_image();
        let (width, height) = image.dimensions();
        let (new_width, new_height) = (width / times, height / times);

        self.resize(new_width, new_height, method);
    }
}
