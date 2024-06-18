use clap::Parser;
use image::io::Reader as ImageReader;

mod icon_gen;

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

    /// List of resolutions for the ICO file
    #[arg(short = 'r', long, default_value = "16,32,48,96,256")]
    ico_resolutions: String,

    /// Dump all the generated image file sizes
    #[arg(short, long, default_value = "false")]
    dump: bool,

    /// Generate PNG file for the system tray
    #[arg(short, long, default_value = "false")]
    tray: bool,

    /// Resolution for the tray PNG file
    #[arg(short = 'T', long, default_value = "512")]
    tray_resolution: u32,

    /// Path to the PNG image
    #[clap(default_value = "icon.png")]
    path: String,
}

fn main() {
    let args = Args::parse();

    let all_flags = !args.ico && !args.icns && !args.dump && !args.tray;

    let ico_resolutions: Vec<u32> = args.ico_resolutions
        .split(',')
        .map(|s| {
            let element = s.trim();
            element.parse().map_err(|_| {
                eprintln!("Warning: Failed to parse the resolution '{}' for ICO, ignoring.", element);
            })
        })
        .filter_map(Result::ok)
        .filter(|&x| {
            if x < 16 || x > 1024 {
                eprintln!("Warning: The resolution '{}' for ICO is out of bounds (16-1024), ignoring.", x);
                false
            } else {
                true
            }
        })
        .collect();

    if ico_resolutions.is_empty() {
        eprintln!("Error: No valid resolutions for ICO were provided.");
        return;
    }

    if args.tray_resolution < 16 || args.tray_resolution > 2048 {
        eprintln!("Error: The resolution for the tray PNG file is out of bounds (16-2048).");
        return;
    }

    let img = ImageReader::open(&args.path);
    if img.is_err() {
        eprintln!("Error: Failed to open the image file '{}'.", args.path);
    }

    let img = img.expect("Error: Failed to process the image file.");

    let img = img.decode();
    if img.is_err() {
        eprintln!("Error: Failed to decode the image file.");
        return;
    }

    let input_img = img.expect("Error: Failed to decode the image file.");
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

    let mut app_image_builder = icon_gen::definition::AppIconGenerator::new(
        &args.out, &input_img, &args.path, &ico_resolutions, args.tray_resolution
    );

    if args.tray || all_flags{
        app_image_builder.build_tray();
    }

    if args.dump {
        app_image_builder.dump_images();
    }

    if args.icns || all_flags {
        app_image_builder.build_icns();
    }

    if args.ico || all_flags {
        app_image_builder.build_ico();
    }
}
