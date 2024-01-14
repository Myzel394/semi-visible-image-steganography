pub mod information {
    use std::iter;

    use image::Rgb;

    use crate::cli::cli::CliArguments;

    #[derive(Debug)]
    pub struct Information {
        pub on_color: Rgb<u8>,
        pub off_color: Rgb<u8>,
        pub pixel_width: u16,
        pub pixel_height: u16,
    }

    impl Information {
        pub fn width_as_u32(&self) -> u32 {
            u32::try_from(self.pixel_width).unwrap()
        }

        pub fn height_as_u32(&self) -> u32 {
            u32::try_from(self.pixel_height).unwrap()
        }

        pub fn get_padded_data(&self, binary_data: Vec<bool>, image_width: &u32) -> Vec<bool> {
            let elements_per_row: usize = usize::try_from(image_width / self.width_as_u32()).unwrap();

            let amount = binary_data.len();
            let remaining_amount = ((amount / elements_per_row) + 1) * elements_per_row - amount;

            let padded_items: Vec<bool> = iter::repeat(false).take(remaining_amount).collect();

            [&binary_data[..], &padded_items[..]].concat()
        }
    }

    pub fn from_cli_arguments(args: &CliArguments) -> Information {
        Information {
            on_color: args.on_color,
            off_color: args.off_color,
            pixel_width: args.pixel_width,
            pixel_height: args.pixel_height,
        }
    }
}
