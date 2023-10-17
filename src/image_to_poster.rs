use exoquant::Color;
use image::{DynamicImage, GenericImageView, Pixel};
use crate::poster;
use crate::poster::Poster;

use std::thread;
use std::sync::Arc;
use std::sync::mpsc::{SyncSender, Receiver, sync_channel};

struct PosterWithPosition {
    poster: Poster,
    x: u32,
    y: u32,
}

pub fn image_to_posters<F1,F2>(image: DynamicImage, label_generator: F1, tooltip_generator: F2, per_poster_quantization: (bool, Option<u32>)) -> poster::PosterArray
where
    F1: Fn(u32, u32, u32, u32) -> String + Send + Sync + 'static, // label_generator:   pos_x, pos_y, width, height
    F2: Fn(u32, u32, u32, u32) -> String + Send + Sync + 'static, // tooltip_generator: pos_x, pos_y, width, height
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

    let (per_poster_quantization, per_poster_quantization_thread_count) = per_poster_quantization;

    if per_poster_quantization {
        let (sender, receiver): (SyncSender<(Poster,u32,u32)>, Receiver<(Poster,u32,u32)>) = sync_channel(0);

        let block_count = (x_size/block_size)*(y_size/block_size);

        let per_poster_quantization_thread_count = u32::min(per_poster_quantization_thread_count.unwrap_or(1), block_count);

        let blocks_per_thread = block_count / per_poster_quantization_thread_count;
        let remaining_blocks = block_count % per_poster_quantization_thread_count;

        let mut pos: u32 = 0;

        let label_generator = Arc::new(label_generator);
        let tooltip_generator = Arc::new(tooltip_generator);
        let image = Arc::new(image);

        for i in 0..per_poster_quantization_thread_count {
            let thread_block_count = if i<remaining_blocks { blocks_per_thread+1 } else { blocks_per_thread };

            let thread_range = pos..pos+thread_block_count;
            pos += thread_block_count;

            let label_generator = label_generator.clone();
            let tooltip_generator = tooltip_generator.clone();

            let image = image.clone();
            
            let sender = sender.clone();

            thread::spawn(move || {
                let label_generator = label_generator.clone();
                let tooltip_generator = tooltip_generator.clone();

                for i in thread_range {
                    let (block_x,block_y) = ( i%(x_size/block_size), i/(x_size/block_size));

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

                    sender.send((poster, block_x, block_y)).unwrap();
                }
            });
        }

        let mut pages: Vec<PosterWithPosition> = Vec::new();

        let mut last_percentage: f64 = -1 as f64;

        for i in 0..block_count {
            let (poster,x,y) = receiver.recv().unwrap();
            pages.push(PosterWithPosition {
                poster: poster,
                x: x,
                y: y
            });
            let percentage = (i as f64/block_count as f64)*100 as f64;
            if last_percentage+(1 as f64) < percentage {
                last_percentage = percentage;
                println!("Converting image to posters: {}%", percentage as u32);
            }
        }
        
        pages.sort_by_key(|x| x.x+x.y*(x_size/block_size));

        for page in pages {
            poster_array.pages.push(page.poster);
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