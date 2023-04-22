mod poster;

use clap::{arg, command, value_parser};
use exoquant::Color;
use image::io::Reader as ImageReader;
use image::{imageops::FilterType, DynamicImage, GenericImageView, Pixel};
use poster::*;
use rand::random;
use std::fs;
use std::path::PathBuf;

//const VERSION: &str = env!("CARGO_PKG_VERSION");

fn read_image(image_file: &PathBuf) -> (bool, Option<DynamicImage>) {
    let image_reader = ImageReader::open(image_file);
    if image_reader.is_err() {
        return (false, None);
    }

    let mut decoder = image_reader.unwrap();
    decoder.no_limits();

    let decoded_image = decoder.decode();
    if decoded_image.is_err() {
        return (false, None);
    }

    return (true, Some(decoded_image.unwrap()));
}

fn main() {
    let matches = command!()
        .arg(
            arg!(-i --input <INPUT_FILE> "Sets input image file")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-o --output <OUTPUT_FILE> "Sets output file (file extension is automatically set, do not specify)")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-x --scalex <X_RESOLUTION> "Scales input image to specified X resolution")
                .required(false)
                .value_parser(value_parser!(u32)),
        )
        .arg(
            arg!(-y --scaley <Y_RESOLUTION> "Scales input image to specified Y resolution")
                .required(false)
                .value_parser(value_parser!(u32)),
        )
        .arg(arg!(-l --label <LABEL> "Poster label").required(false))
        .get_matches();

    if let Some(input) = matches.get_one::<PathBuf>("input") {
        if let Some(output) = matches.get_one::<PathBuf>("output") {
            if !input.exists() {
                println!("Input file doesn't exist.");
                return;
            }
            if input.is_dir() {
                println!("Input can't be a directory.");
                return;
            }

            if output.is_dir() {
                println!("Output can't be a directory.");
                return;
            }

            if !output.parent().unwrap().exists() {
                println!("Output file parent directory doesn't exist.");
                return;
            }

            let (image_ok, image) = read_image(input);
            if !image_ok {
                println!("Failed to decode or open image.");
                return;
            }
            let mut unwrapped_image = image.unwrap();

            let (mut x_size, mut y_size) = unwrapped_image.dimensions();

            {
                let mut resize = false;
                let (mut resize_x, mut resize_y) = (x_size, y_size);

                if let Some(res) = matches.get_one::<u32>("scalex") {
                    resize = true;
                    resize_x = *res;
                }

                if let Some(res) = matches.get_one::<u32>("scaley") {
                    resize = true;
                    resize_y = *res;
                }

                if resize && (resize_x < 1 || resize_y < 1) {
                    println!("Can't resize to x:{0} y:{1}", resize_x, resize_y);
                    return;
                }

                if resize && ((resize_x % 128 != 0) || (resize_y % 128 != 0)) {
                    println!("Image resolutions have to be multiples of 128 (Attempted to resize to x:{0} y:{1})",resize_x, resize_y);
                    return;
                }

                if resize {
                    println!(
                        "Resizing image to x:{0} y:{1} (from x:{2} y:{3})",
                        resize_x, resize_y, x_size, y_size
                    );

                    x_size = resize_x;
                    y_size = resize_y;
                    unwrapped_image =
                        unwrapped_image.resize_exact(resize_x, resize_y, FilterType::CatmullRom);
                }
            }

            if (x_size % 128 != 0) || (y_size % 128 != 0) {
                println!(
                    "Image resolutions have to be multiples of 128 (Currently x:{0} y:{1})",
                    x_size, y_size
                );
                return;
            }

            let label: String;
            if let Some(txt) = matches.get_one::<u32>("label") {
                label = txt.to_string();
                if label.len() > 23 {
                    println!(
                        "Label cant be longer than 25 characters, currently {0}",
                        label.len()
                    );
                    return;
                }
            } else {
                label = "PatriikPlays/img2poster".to_string();
            }

            let mut posters: Vec<String> = Vec::new();
            println!("Converting image to posters");
            let block_size = 128;
            for block_y in 0..y_size / block_size {
                println!(
                    "{0}% complete",
                    f32::min(
                        100 as f32,
                        f32::max(
                            0 as f32,
                            (block_y as f32) / ((y_size as f32) / (block_size as f32))
                        ) * (100 as f32)
                    )
                );

                for block_x in 0..x_size / block_size {
                    let mut pixels: Vec<Color> = Vec::new();

                    for y in 0..block_size {
                        for x in 0..block_size {
                            let pixel = unwrapped_image
                                .get_pixel(x + block_x * block_size, y + block_y * block_size);

                            let rgb = pixel.to_rgb();
                            pixels.push(Color::new(rgb[0], rgb[1], rgb[2], 255));
                        }
                    }

                    let (dithered_pixels, color_palette) = poster::dither(pixels);

                    let tooltip: PosterTooltip = PosterTooltip {
                        print_id: random::<u32>(),
                        print_name: label.clone(),
                        total_width: x_size / block_size,
                        total_height: y_size / block_size,
                        pos_x: block_x,
                        pos_y: block_y,
                        info: "https://github.com/PatriikPlays/img2poster".to_string(),
                    };

                    let poster: Poster = Poster {
                        label: format!(
                            "{0}: ({1},{2})/({3}x{4})",
                            label.clone(),
                            block_x+1,
                            block_y+1,
                            x_size / block_size,
                            y_size / block_size
                        ),
                        tooltip: serde_json::to_string(&tooltip)
                            .unwrap()
                            .as_str()
                            .to_string(),
                        palette: color_palette,
                        pixels: dithered_pixels.as_slice(),
                        width: 128,
                        height: 128,
                    };

                    let json = serde_json::to_string(&poster).unwrap();
                    posters.push(json.as_str().to_string());
                }
            }
            println!("100% complete");
            let mut out_path = output.clone();
            if posters.len() > 1 {
                out_path.set_extension("2dja");
                fs::write(out_path, format!("[{0}]", posters.join(","))).expect("Failed to write to output file.");
            } else {
                out_path.set_extension("2dj");
                fs::write(out_path, &posters[0]).expect("Failed to write to output file.");
            }
        } else {
            panic!("Output argument doesn't exist, this shouldn't have happened");
        }
    } else {
        panic!("Input argument doesn't exist, this shouldn't have happened");
    }
}