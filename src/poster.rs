use exoquant::{convert_to_indexed, ditherer, optimizer, Color};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PosterTooltip {
    pub print_id: u32,
    pub print_name: String,
    pub total_width: u32,
    pub total_height: u32,
    pub pos_x: u32,
    pub pos_y: u32,
    pub info: String,
}

#[derive(Serialize, Deserialize)]
pub struct Poster<'a> {
    pub label: String,
    pub tooltip: String,
    pub palette: Vec<u32>,
    #[serde(with = "serde_bytes")]
    pub pixels: &'a [u8],
    pub width: u32,
    pub height: u32,
}

fn rgb_to_hex(red: u8, green: u8, blue: u8) -> u32 {
    ((red as u32) << 16) | ((green as u32) << 8) | blue as u32
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
