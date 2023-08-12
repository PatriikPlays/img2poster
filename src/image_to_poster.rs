use exoquant::Color;
use image::{DynamicImage, GenericImageView, Pixel};
use crate::poster;
use crate::poster::{Poster};

pub fn image_to_posters<F1,F2>(image: DynamicImage, label_generator: F1, tooltip_generator: F2, per_poster_quantization: bool) -> poster::PosterArray
where
    F1: Fn(u32, u32, u32, u32) -> String, // label_generator:   pos_x, pos_y, width, height
    F2: Fn(u32, u32, u32, u32) -> String, // tooltip_generator: pos_x, pos_y, width, height
{
    //let mut poster_array.pages: Vec<Poster> = Vec::new();

    let block_size = 128;
    let (x_size, y_size) = image.dimensions();

    let mut poster_array: poster::PosterArray = poster::PosterArray {
        pages: Vec::new(),
        width: x_size/block_size,
        height: y_size/block_size,
        title: "untitled".to_string(), // TODO: do title
    };

    if per_poster_quantization {
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
                        let pixel = image
                            .get_pixel(x + block_x * block_size, y + block_y * block_size);

                        let rgb = pixel.to_rgb();
                        pixels.push(Color::new(rgb[0], rgb[1], rgb[2], 255));
                    }
                }

                let (dithered_pixels, color_palette) = poster::dither(pixels, block_size as usize);

                let poster: Poster = Poster {
                    label: label_generator(block_x, block_y, x_size / block_size, y_size / block_size),
                    tooltip: tooltip_generator(block_x, block_y, x_size / block_size, y_size / block_size),
                    palette: color_palette,
                    pixels: dithered_pixels,
                    width: block_size,
                    height: block_size,
                };

                poster_array.pages.push(poster);
            }
        }
    } else {
        let mut pixels: Vec<Color> = Vec::with_capacity((block_size * block_size) as usize);

        print!("Parsing image... ");
        for y in 0..y_size {
            for x in 0..x_size {
                let pixel = image.get_pixel(x, y);

                let rgb = pixel.to_rgb();
                pixels.push(Color::new(rgb[0], rgb[1], rgb[2], 255));
            }
        }
        println!("Done");

        print!("Quantizing and dithering image... ");
        let (dithered_pixels, color_palette) = poster::dither(pixels, x_size as usize);
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

                let poster: Poster = Poster {
                    label: label_generator(block_x, block_y, x_size / block_size, y_size / block_size),
                    tooltip: tooltip_generator(block_x, block_y, x_size / block_size, y_size / block_size),
                    palette: color_palette.clone(),
                    pixels: block_pixels,
                    width: block_size,
                    height: block_size,
                };

                poster_array.pages.push(poster);
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

    return poster_array;
}