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

    let all_flags = !args.ico && !args.icns && !args.dump && !args.tray;

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

    let mut app_image_builder = icon_gen::definition::AppIconGenerator::new(&args.out, &input_img, &args.path);

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
