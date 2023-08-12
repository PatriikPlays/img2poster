use exoquant::{convert_to_indexed, ditherer, optimizer, Color};
use image::{DynamicImage, ImageBuffer, Pixel, Rgba};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PosterTooltip {
    pub print_id: String,
    pub print_name: String,
    pub total_width: u32,
    pub total_height: u32,
    pub pos_x: u32,
    pub pos_y: u32,
    pub info: String,
}

#[derive(Serialize, Deserialize)]
pub struct Poster {
    pub label: String,
    pub tooltip: String,
    pub palette: Vec<u32>,
    #[serde(with = "serde_bytes")]
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize)]
pub struct PosterArray {
    pub pages: Vec<Poster>,
    pub width: u32,
    pub height: u32,
    pub title: String
}

fn rgb_to_hex(red: u8, green: u8, blue: u8) -> u32 {
    ((red as u32) << 16) | ((green as u32) << 8) | blue as u32
}

pub fn posters_to_dynamic_image(poster_array: &PosterArray) -> DynamicImage {
    let poster_width = poster_array.width;
    let poster_height = poster_array.height;
    let mut image_buffer = vec![0u8; (poster_width * 128 * poster_height * 128 * 4) as usize];

    for (poster_index, poster) in poster_array.pages.iter().enumerate() {
        let offset_x = (poster_index as u32 % poster_width) * 128;
        let offset_y = (poster_index as u32 / poster_width) * 128;

        for (pixel_index, pixel) in poster.pixels.iter().enumerate() {
            let palette_index = *pixel as usize;
            let color_value = poster.palette[palette_index-1];
            let color;

            if palette_index == 0 {
                color = Rgba {
                    0: [0u8,0u8,0u8,0u8]
                };
            } else {
                color = Rgba {
                    0: [
                        ((color_value >> 16) & 0xFF) as u8,
                        ((color_value >> 8) & 0xFF) as u8,
                        (color_value & 0xFF) as u8,
                        255u8,
                    ]
                };
            }

            let x = offset_x + (pixel_index as u32 % 128);
            let y = offset_y + (pixel_index as u32 / 128);
            let index = ((y * (poster_width * 128) + x) * 4) as usize;

            image_buffer[index..index + 4].copy_from_slice(&color.channels());
        }
    }

    let image = ImageBuffer::from_raw(poster_width * 128, poster_height * 128, image_buffer)
        .expect("Failed to create image buffer");

    return DynamicImage::ImageRgba8(image)
}

pub fn dither(image_data: Vec<Color>, width: usize) -> (Vec<u8>, Vec<u32>) {
    let image_data_slice: &[Color] = &image_data;
    let (palette, indexed_data) = convert_to_indexed(
        image_data_slice,
        width,
        63,
        &optimizer::KMeans,
        &ditherer::FloydSteinberg::new(),
    );

    let mut colors: Vec<u32> = Vec::new();
    for i in 0..palette.len() {
        let color = palette[i];
        colors.push(rgb_to_hex(color.r, color.g, color.b));
    }

    let mut ret_indexed_data = Vec::new();
    for i in 0..indexed_data.len() {
        ret_indexed_data.push(indexed_data[i] + 1)
    }

    return (ret_indexed_data, colors);
}
