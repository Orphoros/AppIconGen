use image::DynamicImage;

use super::ImageSet;

pub struct AppIconGenerator<'a> {
    pub out: &'a str,
    pub input_img: &'a DynamicImage,
    pub input_file: &'a str,
    pub icns_images: Option<ImageSet>,
    pub ico_resolutions: &'a Vec<u32>,
    pub tray_resolution: u32,
}

impl<'a> AppIconGenerator<'a> {
    pub fn new(out: &'a str, input_img: &'a DynamicImage, input_file: &'a str, ico_resolutions: &'a Vec<u32>, tray_resolution: u32) -> Self {
        AppIconGenerator {
            out,
            input_img,
            input_file,
            icns_images: None,
            ico_resolutions,
            tray_resolution,
        }
    }
}
