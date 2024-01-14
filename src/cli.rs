pub mod cli {
    use clap::Parser;
    use image::Rgb;

    use crate::utils::utils::{str_to_rgb, convert_num_to_binary};

    #[derive(Parser)]
    #[command(name = "NoNameYet")]
    #[command(version = "0.1")]
    #[command(about = "Encode text into image binary", long_about = None)]
    struct Cli {
        text: String,

        #[arg(long, short, default_value_t = String::from("image.png"))]
        output: String,

        #[arg(long, default_value_t = String::from("#FFF"))]
        on_color: String,

        #[arg(long, default_value_t = String::from("#000"))]
        off_color: String,

        #[arg(long, default_value_t = String::from("#050"))]
        fill_color: String,

        #[arg(long, default_value_t = 1920)]
        image_width: u32,
        #[arg(long, default_value_t = 1080)]
        image_height: u32,

        #[arg(long, default_value_t = 8)]
        pixel_width: u16,
        #[arg(long, default_value_t = 8)]
        pixel_height: u16,
    }

    pub struct CliArguments {
        pub binary_data: Vec<bool>,
        pub output: String,
        pub on_color: Rgb<u8>,
        pub off_color: Rgb<u8>,
        pub fill_color: Rgb<u8>,
        pub image_width: u32,
        pub image_height: u32,
        pub pixel_width: u16,
        pub pixel_height: u16,
    }

    pub fn parse() -> CliArguments {
        let args = Cli::parse();
        
        CliArguments {
            binary_data: args.text.into_bytes().iter().flat_map(|number| convert_num_to_binary(number)).collect(),
            output: args.output,
            image_width: args.image_width,
            image_height: args.image_height,
            pixel_width: args.pixel_width,
            pixel_height: args.pixel_height,
            on_color: str_to_rgb(&args.on_color),
            off_color: str_to_rgb(&args.off_color),
            fill_color: str_to_rgb(&args.fill_color),
        }
    }
}

