use std::collections::HashMap;

use icns::{IconFamily, IconType, Image, PixelFormat};
use image::DynamicImage;

pub fn build_icns(icns_images: &HashMap<IconType, DynamicImage>, out: &str) {
    let mut icon_family = IconFamily::new();
    for (icon_type, img) in icns_images.iter() {
        let buffer = img.to_rgba8().into_raw();
        let width = img.width() as u32;
        let height = img.height() as u32;
        let format: PixelFormat = PixelFormat::RGBA;
        let image = Image::from_data(format, width, height, buffer).unwrap();
        icon_family.add_icon_with_type(&image, *icon_type).unwrap();
    }
    let path = format!("{}.icns", out);
    let file = std::fs::File::create(&path).unwrap();
    icon_family.write(file).unwrap();
}