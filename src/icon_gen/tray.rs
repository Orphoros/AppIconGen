use image::DynamicImage;

pub fn build_tray(input_img: &DynamicImage, out: &str) {
    let resized = input_img.resize(512, 512, image::imageops::FilterType::Lanczos3);
    let mut resized = resized.to_rgba8();
    for pixel in resized.pixels_mut() {
        if pixel[0] > 0 || pixel[1] > 0 || pixel[2] > 0 {
            pixel[0] = 255;
            pixel[1] = 255;
            pixel[2] = 255;
        }
    }
    let path = format!("{}_tray.png", out);
    resized.save(path).unwrap();
}