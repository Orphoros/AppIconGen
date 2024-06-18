use image::DynamicImage;

use super::ImageSet;

pub struct AppIconGenerator<'a> {
    pub out: &'a str,
    pub input_img: &'a DynamicImage,
    pub input_file: &'a str,
    pub icns_images: Option<ImageSet>,
}

impl<'a> AppIconGenerator<'a> {
    pub fn new(out: &'a str, input_img: &'a DynamicImage, input_file: &'a str) -> Self {

        AppIconGenerator {
            out,
            input_img,
            input_file,
            icns_images: None,
        }
    }
}
