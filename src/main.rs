pub mod information;
pub mod utils;
pub mod painter;
pub mod cli;
pub mod logger;

use image::{RgbImage, ImageBuffer};
use log::info;
use painter::painter::Painter;
use utils::utils::repeat_data_until_full;

const ONE_AS_STRING: &str = "1";
const ZERO_AS_STRING: &str = "0";

fn main() {
    let _ = logger::logger::init();

    let args = cli::cli::parse();

    let mut image: RgbImage = ImageBuffer::new(args.image_width, args.image_height);
    
    let mut painter = Painter::new(&args, &mut image);
    painter.fill(&args.fill_color);

    info!(
        "Binary data: {}", 
        &args.binary_data
        .iter()
        .map(|b| if b == &true {ONE_AS_STRING} else {ZERO_AS_STRING})
        .collect::<Vec<&str>>()
        .join(" ")
    );

    let data = repeat_data_until_full(
            &args.binary_data, 
            &args.image_width, 
            &args.image_height, 
            &(args.pixel_width as u32),
            &(args.pixel_height as u32),
            &args.remaining_behavior,
        );

    painter.paint_data(&data);

    image.save(args.output).unwrap();
}

