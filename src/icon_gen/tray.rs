use crate::error_exit;

use super::definition::AppIconGenerator;

impl AppIconGenerator<'_> {
    pub fn build_tray(&self) {
        let resized = self.input_img.resize(self.tray_resolution, self.tray_resolution, image::imageops::FilterType::Lanczos3);
        let mut resized = resized.to_rgba8();
        for pixel in resized.pixels_mut() {
            if pixel[0] > 0 || pixel[1] > 0 || pixel[2] > 0 {
                pixel[0] = 255;
                pixel[1] = 255;
                pixel[2] = 255;
            }
        }
        let path = format!("{}_tray.png", self.out);
        resized.save(path).unwrap_or_else(|_| error_exit!("failed to save tray PNG image"));
    }
}
