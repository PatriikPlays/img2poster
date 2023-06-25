mod poster;

use clap::{arg, command, Parser};
use exoquant::Color;
use image::io::Reader as ImageReader;
use image::{imageops::FilterType, DynamicImage, GenericImageView, Pixel};
use poster::*;
use rand::random;
use std::fs;
use std::path::PathBuf;

//const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "INPUT_FILE")]
    input: PathBuf,

    #[arg(short, long, value_name = "OUTPUT_FILE")]
    output: PathBuf,

    #[arg(short = 'x', long, value_name = "SCALE_X")]
    scale_x: Option<u32>,

    #[arg(short = 'y', long, value_name = "SCALE_Y")]
    scale_y: Option<u32>,

    #[arg(short, long, value_name = "LABEL")]
    label: Option<String>,

    #[arg(short = 'L', long = "forcelabel", value_name = "LABEL")]
    force_label: Option<String>,

    #[arg(short = 'T', long = "forcetooltip", value_name = "TOOLTIP")]
    force_tooltip: Option<String>,

    #[arg(short = 'Q', long)]
    per_poster_quantization: bool
}

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
    let cli = Cli::parse();

    let per_poster_quantization_enabled = cli.per_poster_quantization;

    if !cli.input.exists() {
        println!("Input file doesn't exist.");
        return;
    }
    if cli.input.is_dir() {
        println!("Input can't be a directory.");
        return;
    }

    if cli.output.is_dir() {
        println!("Output can't be a directory.");
        return;
    }

    if !cli.output.parent().unwrap().exists() {
        println!("Output file parent directory doesn't exist.");
        return;
    }

    let (image_ok, image) = read_image(&cli.input);
    if !image_ok {
        println!("Failed to decode or open image.");
        return;
    }
    let mut unwrapped_image = image.unwrap();

    let (mut x_size, mut y_size) = unwrapped_image.dimensions();

    {
        let mut resize = false;
        let (mut resize_x, mut resize_y) = (x_size, y_size);

        if let Some(res) = cli.scale_x {
            resize = true;
            resize_x = res;
        }

        if let Some(res) = cli.scale_y {
            resize = true;
            resize_y = res;
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

    let mut forced_label: bool = false;
    let label: String;

    if let Some(txt) = cli.force_label {
        label = txt.to_string();
        forced_label = true;
        if label.len() > 48 {
            println!(
                "Forced label can't be longer than 48 characters, currently {0}",
                label.len()
            );
            return;
        }
    } else if let Some(txt) = cli.label {
        label = txt.to_string();
        if label.len() > 23 {
            println!(
                "Label can't be longer than 25 characters, currently {0}",
                label.len()
            );
            return;
        }
    } else {
        label = "PatriikPlays/img2poster".to_string();
    }

    let mut use_forced_tooltip = false;
    let mut forced_tooltip: String = "".to_string();
    if let Some(txt) = cli.force_tooltip {
        forced_tooltip = txt.to_string();
        use_forced_tooltip = true;
        if forced_tooltip.len() > 256 {
            println!(
                "Forced tooltip can't be longer than 256 characters, currently {0}",
                forced_tooltip.len()
            );
            return;
        }
    }

    let mut posters: Vec<Poster> = Vec::new();

    let print_id = random::<u32>();
    println!("Converting image to posters");

    let block_size = 128;
    if per_poster_quantization_enabled.clone() {
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

                let (dithered_pixels, color_palette) = dither(pixels, block_size as usize);

                let tooltip: PosterTooltip = PosterTooltip {
                    print_id,
                    print_name: label.clone(),
                    total_width: x_size / block_size,
                    total_height: y_size / block_size,
                    pos_x: block_x,
                    pos_y: block_y,
                    info: "https://github.com/PatriikPlays/img2poster".to_string(),
                };

                let tooltip_str: String;
                if use_forced_tooltip {
                    tooltip_str = forced_tooltip.clone();
                } else {
                    tooltip_str = serde_json::to_string(&tooltip)
                        .unwrap()
                        .as_str()
                        .to_string();
                }

                let label_str: String;
                if forced_label {
                    label_str = label.clone();
                } else {
                    label_str = format!(
                        "{0}: ({1},{2})/({3}x{4})",
                        label.clone(),
                        block_x + 1,
                        block_y + 1,
                        x_size / block_size,
                        y_size / block_size
                    )
                }

                let poster: Poster = Poster {
                    label: label_str,
                    tooltip: tooltip_str,
                    palette: color_palette,
                    pixels: dithered_pixels,
                    width: 128,
                    height: 128,
                };

                posters.push(poster);
            }
        }
    } else {
        let mut pixels: Vec<Color> = Vec::with_capacity((block_size * block_size) as usize);

        print!("Parsing image... ");
        for y in 0..y_size {
            for x in 0..x_size {
                let pixel = unwrapped_image.get_pixel(x, y);

                let rgb = pixel.to_rgb();
                pixels.push(Color::new(rgb[0], rgb[1], rgb[2], 255));
            }
        }
        println!("Done");

        print!("Quantizing and dithering image... ");
        let (dithered_pixels, color_palette) = dither(pixels, x_size as usize);
        println!("Done");

        for block_y in 0..y_size / block_size {
            for block_x in 0..x_size / block_size {
                let mut block_pixels: Vec<u8> =
                    Vec::with_capacity((block_size * block_size) as usize);
                for in_block_y in 0..block_size {
                    for in_block_x in 0..block_size {
                        block_pixels.push(
                            dithered_pixels[((block_y * block_size + in_block_y) * x_size
                                + block_x * block_size
                                + in_block_x) as usize],
                        );
                    }
                }

                let tooltip: PosterTooltip = PosterTooltip {
                    print_id,
                    print_name: label.clone(),
                    total_width: x_size / block_size,
                    total_height: y_size / block_size,
                    pos_x: block_x,
                    pos_y: block_y,
                    info: "https://github.com/PatriikPlays/img2poster".to_string(),
                };

                let tooltip_str: String;
                if use_forced_tooltip {
                    tooltip_str = forced_tooltip.clone();
                } else {
                    tooltip_str = serde_json::to_string(&tooltip)
                        .unwrap()
                        .as_str()
                        .to_string();
                }

                let label_str: String;
                if forced_label {
                    label_str = label.clone();
                } else {
                    label_str = format!(
                        "{0}: ({1},{2})/({3}x{4})",
                        label.clone(),
                        block_x + 1,
                        block_y + 1,
                        x_size / block_size,
                        y_size / block_size
                    )
                }

                let poster: Poster = Poster {
                    label: label_str,
                    tooltip: tooltip_str,
                    palette: color_palette.clone(),
                    pixels: block_pixels,
                    width: 128,
                    height: 128,
                };

                posters.push(poster);
            }

            println!(
                "Splitting image into posters: {0}% complete",
                f32::min(
                    100 as f32,
                    f32::max(0 as f32, block_y as f32 / ((y_size / block_size) as f32)) * (100 as f32)
                )
            );
        }
        println!("Splitting image into posters: 100% complete");
    }
    println!("Done, saving to file");

    let mut out_path = cli.output.clone();
    if posters.len() > 1 {
        out_path.set_extension("2dja");
        let json_str = serde_json::to_string(&posters).expect("Failed to serialize this somehow");
        fs::write(out_path, json_str).expect("Failed to write to output file.");
    } else {
        out_path.set_extension("2dj");
        let json_str = serde_json::to_string(&posters[0]).expect("Failed to serialize this somehow");
        fs::write(out_path, json_str).expect("Failed to write to output file.");
    }
}
