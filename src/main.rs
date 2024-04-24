use std::collections::HashMap;

use clap::Parser;
use ico_builder::IcoBuilder;
use image::{io::Reader as ImageReader, DynamicImage};
use icns::{IconFamily, IconType, Image, PixelFormat};

#[derive(Parser, Debug)]
#[command(name = "appicongen", version, about, after_help = "Make sure that the input PNG image is square and has a resolution of at least 1024x1024 pixels.")]
struct Args {
    /// Name and path of the output file without extension
    #[arg(short, long, default_value = "appicon")]
    out: String,

    /// Generate ICO file
    #[arg(short = 'i', long, default_value = "false")]
    ico: bool,

    /// Generate ICNS file
    #[arg(short = 'I', long, default_value = "false")]
    icns: bool,

    /// Dump all the generated image file sizes
    #[arg(short, long, default_value = "false")]
    dump: bool,

    /// Generate PNG file for the system tray
    #[arg(short, long, default_value = "false")]
    tray: bool,

    /// Path to the PNG image
    #[clap(default_value = "icon.png")]
    path: String,
}

fn main() {
    let args = Args::parse();

    let img = ImageReader::open(&args.path);
    if img.is_err() {
        eprintln!("Error: Failed to open the image file.");
    }

    let img = img.unwrap();

    let img = img.decode();
    if img.is_err() {
        eprintln!("Error: Failed to decode the image file.");
        return;
    }

    let input_img = img.unwrap();
    let img = input_img.to_rgba8();

    let (width, height) = img.dimensions();
    if width < 1024 || height < 1024 {
        eprintln!("Error: The image resolution is too low. It must be at least 1024x1024 pixels.");
        return;
    }

    if width != height {
        eprintln!("Error: The image is not square.");
        return;
    }

    if args.tray {
        let resized = input_img.resize(512, 512, image::imageops::FilterType::Lanczos3);
        let mut resized = resized.to_rgba8();
        for pixel in resized.pixels_mut() {
            if pixel[0] > 0 || pixel[1] > 0 || pixel[2] > 0 {
                pixel[0] = 255;
                pixel[1] = 255;
                pixel[2] = 255;
            }
        }
        let path = format!("{}_tray.png", args.out);
        resized.save(path).unwrap();
    }

    let mut icns_images: HashMap<IconType, DynamicImage> = HashMap::new();

    if args.dump && args.icns {
        for size in &[16, 32, 64, 128, 256, 512, 1024] {
            let resized = input_img.resize(*size, *size, image::imageops::FilterType::Lanczos3);
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
    }

    if args.dump {
        for (icon_type, img) in icns_images.iter() {
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
            let path = format!("{}_{}.png", args.out, type_name);
            img.save(path).unwrap();
        }
    }

    if args.icns {
        let mut icon_family = IconFamily::new();
        for (icon_type, img) in icns_images.iter() {
            let buffer = img.to_rgba8().into_raw();
            let width = img.width() as u32;
            let height = img.height() as u32;
            let format: PixelFormat = PixelFormat::RGBA;
            let image = Image::from_data(format, width, height, buffer).unwrap();
            icon_family.add_icon_with_type(&image, *icon_type).unwrap();
        }
        let path = format!("{}.icns", args.out);
        let file = std::fs::File::create(&path).unwrap();
        icon_family.write(file).unwrap();
    }

    if args.ico {
        IcoBuilder::default()
        .sizes(&[16, 32, 48, 96, 256])
        .add_source_file(&args.path)
        .build_file(&format!("{}.ico", args.out)).unwrap();
    }
}
