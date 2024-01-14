pub mod utils {
    use std::iter;

    use image::Rgb;

    pub fn str_to_rgb(value: &str) -> Rgb<u8> {
        let css_result = csscolorparser::parse(value).unwrap().to_rgba8();

        Rgb([
            css_result[0],
            css_result[1],
            css_result[2],
        ])
    }

    pub fn convert_num_to_binary(number: &u8) -> Vec<bool> {
        (0..8).rev().map(|n| (number >> n) & 1 == 1).to_owned().collect()
    }


    // Returns the data, plus padding so that the row in the image
    // will be filled with `false`s.
    // This function assumes image_width is divisible by width without
    // any reminder
    pub fn get_padded_data(data: &Vec<bool>, image_width: &u32, pixel_width: &u32) -> Vec<bool> {
        let elements_per_row = (image_width / pixel_width) as usize;

        let amount = data.len();
        let remaining_amount = ((amount / elements_per_row) + 1) * elements_per_row - amount;

        let padded_items: Vec<bool> = iter::repeat(false).take(remaining_amount).collect();

        [&data[..], &padded_items[..]].concat()
    }

    pub enum RemainingBehavior {
        PadWithNull,
        DoNothing,
        TakeFirsts,
    }

    pub fn repeat_data_until_full(
        data: &Vec<bool>, 
        image_width: &u32, 
        image_height: &u32, 
        pixel_width: &u32,
        pixel_height: &u32,
        remaining_behavior: RemainingBehavior
    ) -> Vec<bool> {
        let full_width = image_width * image_height;
        let available_pixels = (full_width / (pixel_width * pixel_height)) as usize;
        let repeats = available_pixels / data.len();

        dbg!(available_pixels, repeats, data.len());

        let mut result: Vec<bool> = (0..repeats).flat_map(|_| data.clone()).collect();

        match (remaining_behavior) {
             RemainingBehavior::DoNothing => {}
             RemainingBehavior::PadWithNull => {
                let remaining = available_pixels - result.len();

                result.extend(iter::repeat(false).take(remaining));
             }
             RemainingBehavior::TakeFirsts => {
                let remaining = available_pixels - result.len();

                result.extend(&data[0..(remaining)]);
             }
        }

        result
    }

    // Creates a number to binary representation
    // using division approach
    // pub fn num_to_binary_using_div(number: &u8) -> Vec<bool> {
    //     let mut values: Vec<bool> = vec![];
    //     let nums = [128, 64, 32, 16, 8, 4, 2, 1];
    //
    //     let mut current_num = number.clone();
    //
    //     for num in nums {
    //         let int_div = current_num / num;
    //         let float_div = current_num as f64 / num as f64;
    //         let divisible: bool = int_div as f64 == float_div;
    //
    //         values.push(divisible);
    //     }
    //
    //     values
    // }
}
