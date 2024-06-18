use std::collections::HashMap;

use icns::IconType;

use super::{definition::AppIconGenerator, ImageSet};

impl AppIconGenerator<'_> {
    pub fn generate_images(&mut self) {
        let mut icns_images: ImageSet = HashMap::new();
        for size in &[16, 32, 64, 128, 256, 512, 1024] {
            let resized = self.input_img.resize(*size, *size, image::imageops::FilterType::Lanczos3);
            let icon_type = match size {
                16 => Some(IconType::RGBA32_16x16),
                32 => Some(IconType::RGBA32_32x32),
                128 => Some(IconType::RGBA32_128x128),
                256 => Some(IconType::RGBA32_256x256),
                512 => Some(IconType::RGBA32_512x512),
                _ => None
            };
            let icon_type_x2 = match size {
                32 => Some(IconType::RGBA32_16x16_2x),
                64 => Some(IconType::RGBA32_32x32_2x),
                256 => Some(IconType::RGBA32_128x128_2x),
                512 => Some(IconType::RGBA32_256x256_2x),
                1024 => Some(IconType::RGBA32_512x512_2x),
                _ => None
            };
            if let Some(icon_type) = icon_type {
                icns_images.insert(icon_type, resized.clone());
            }
            if let Some(icon_type) = icon_type_x2 {
                icns_images.insert(icon_type, resized.clone());
            }
        }
        self.icns_images = Some(icns_images);
    }

    pub fn dump_images(&mut self) {
        if self.icns_images.is_none() {
            self.generate_images();
        }
        for (icon_type, img) in self.icns_images.as_ref().expect("Image set not yet generated.").iter() {
            let type_name = match icon_type {
                IconType::RGBA32_16x16 => "16x16",
                IconType::RGBA32_32x32 => "32x32",
                IconType::RGBA32_128x128 => "128x128",
                IconType::RGBA32_256x256 => "256x256",
                IconType::RGBA32_512x512 => "512x512",
                IconType::RGBA32_16x16_2x => "16x16@2x",
                IconType::RGBA32_32x32_2x => "32x32@2x",
                IconType::RGBA32_128x128_2x => "128x128@2x",
                IconType::RGBA32_256x256_2x => "256x256@2x",
                IconType::RGBA32_512x512_2x => "512x512@2x",
                _ => "unknown"
            };
            let path = format!("{}_{}.png", self.out, type_name);
            img.save(path).expect(&format!("Failed to dump image '{}' to disk.", type_name));
        }
    }
}
