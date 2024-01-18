pub mod painter {
    use image::{RgbImage, Rgb};

    use crate::cli::cli::CliArguments;

    pub struct Painter<'a> {
        cli_arguments: &'a CliArguments,
        image: &'a mut RgbImage,
    }

    impl Painter<'_> {
        pub fn new<'a>(cli_arguments: &'a CliArguments, image: &'a mut RgbImage) -> Painter<'a> {
            Painter {
                cli_arguments,
                image,
            }
        }

        pub fn fill(&mut self, color: &Rgb<u8>) {
            for x in 0..self.image.width() {
                for y in 0..self.image.height() {
                    *self.image.get_pixel_mut(x, y) = *color;
                }
            }
        }

        pub fn _paint_raw_block(
            &mut self, 
            x_position: u32, 
            y_position: u32, 
            color: &Rgb<u8>
        ) {
            for x_offset in 0..self.cli_arguments.pixel_width {
                for y_offset in 0..self.cli_arguments.pixel_height {
                    let x = x_position + (x_offset as u32);
                    let y = y_position + (y_offset as u32);

                    *self.image.get_pixel_mut(x, y) = *color;
                }
            }
        }

        pub fn paint_pixel(
            &mut self,
            pixel_x: &u32,
            pixel_y: &u32,
            color: &Rgb<u8>,
        ) {
            self._paint_raw_block(
                (self.cli_arguments.pixel_width as u32) * pixel_x, 
                (self.cli_arguments.pixel_height as u32) * pixel_y, 
                color,
            )
        }

        pub fn paint_data(
            &mut self,
            data: &Vec<bool>,
        ) {
            let px_width = self.cli_arguments.pixel_width as u32;
            let px_height = self.cli_arguments.pixel_height as u32;

            let image_width = self.cli_arguments.image_width as u32;

            let pixels_per_row = image_width / px_width;

            for (ii, val) in data.iter().enumerate() {
                let index = ii as u32;

                let x = (index % pixels_per_row) * px_width;
                let y = (index / pixels_per_row) * px_height;

                if *val {
                    self._paint_raw_block(x, y, &self.cli_arguments.on_color);
                } else {
                    self._paint_raw_block(x, y, &self.cli_arguments.off_color);
                }
            }
        }
    }
}
