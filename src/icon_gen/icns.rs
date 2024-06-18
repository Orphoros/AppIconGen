use icns::{IconFamily, Image, PixelFormat};

use super::definition::AppIconGenerator;

impl AppIconGenerator<'_> {
    pub fn build_icns(&mut self) {
        let mut icon_family = IconFamily::new();

        if self.icns_images.is_none() {
            self.generate_images();
        }

        for (icon_type, img) in self.icns_images.as_ref().expect("Image set for ICNS not yet generated.").iter() {
            let buffer = img.to_rgba8().into_raw();
            let width = img.width() as u32;
            let height = img.height() as u32;
            let format: PixelFormat = PixelFormat::RGBA;
            let image = Image::from_data(format, width, height, buffer).expect("Failed to create image sub image for ICNS.");
            icon_family.add_icon_with_type(&image, *icon_type).expect("Failed to add image to ICNS set.");
        }
        let path = format!("{}.icns", self.out);
        let file = std::fs::File::create(&path).expect("Failed to create ICNS file.");
        icon_family.write(file).expect("Failed to write ICNS file.");
    }
}
