pub mod cli {
    use clap::{arg, value_parser, ValueEnum, builder::PossibleValue, Command, Arg, ArgAction};
    use image::Rgb;

    use crate::utils::utils::{str_to_rgb, convert_num_to_binary, RemainingBehavior};

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
        pub remaining_behavior: RemainingBehavior,
    }

    impl ValueEnum for RemainingBehavior {
        fn value_variants<'a>() -> &'a [Self] {
            &[RemainingBehavior::PadWithNull, RemainingBehavior::DoNothing, RemainingBehavior::TakeFirsts]
        }

        fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
            Some(match self {
                RemainingBehavior::PadWithNull => PossibleValue::new("pad-with-null").help("Pad with 0s"),
                RemainingBehavior::DoNothing => PossibleValue::new("do-nothing").help("Do nothing"),
                RemainingBehavior::TakeFirsts => PossibleValue::new("take-firsts").help("Take the firsts"),
            })
        }
    }

    pub fn parse() -> CliArguments {
        let cmd_args = Command::new("Semi-Visible Image Steganography")
            .version("0.1.0")
            .author("Myzel394 <github.7a2op@simplelogin.co>")
            .about("Encode data into an image that can be used as an image background then")
            .arg(
                Arg::new("text")
                    .action(ArgAction::Set)
                    .help("Text to encode. Must be ASCII")
                    .required(true)
            )
            .arg(
                Arg::new("output")
                    .action(ArgAction::Set)
                    .short('o')
                    .long("output")
                    .help("Output file")
                    .default_value("output.png")
            )
            .arg(
                Arg::new("image-width")
                    .action(ArgAction::Set)
                    .long("image-width")
                    .help("Width of the image")
                    .value_parser(value_parser!(u32))
                    .default_value("1000")
            )
            .arg(
                Arg::new("image-height")
                    .action(ArgAction::Set)
                    .long("image-height")
                    .help("Height of the image")
                    .value_parser(value_parser!(u32))
                    .default_value("1000")
            )
            .arg(
                Arg::new("pixel-width")
                    .action(ArgAction::Set)
                    .short('x')
                    .long("pixel-width")
                    .help("Width of a pixel")
                    .value_parser(value_parser!(u16))
                    .default_value("8")
            )
            .arg(
                Arg::new("pixel-height")
                    .action(ArgAction::Set)
                    .short('y')
                    .long("pixel-height")
                    .help("Height of a pixel")
                    .value_parser(value_parser!(u16))
                    .default_value("8")
            )
            .arg(
                Arg::new("on-color")
                    .action(ArgAction::Set)
                    .long("on-color")
                    .help("Color of a pixel when the data is 1")
                    .default_value("#FFF")
            )
            .arg(
                Arg::new("off-color")
                    .action(ArgAction::Set)
                    .long("off-color")
                    .help("Color of a pixel when the data is 0")
                    .default_value("#000")
            )
            .arg(
                Arg::new("fill-color")
                    .action(ArgAction::Set)
                    .short('l')
                    .long("fill-color")
                    .help("Default fill color of the image. Pixels will overwrite this color")
                    .default_value("#000")
            )
            .arg(
                Arg::new("remaining-behavior")
                    .action(ArgAction::Set)
                    .short('r')
                    .long("remaining-behavior")
                    .help("What to do with the remaining data")
                    .value_parser(value_parser!(RemainingBehavior))
                    .default_value("take-firsts")
            );
        let matches = cmd_args.get_matches();

        
        CliArguments {
            binary_data: matches.get_one::<String>("text").unwrap().to_string().into_bytes().iter().flat_map(|number| convert_num_to_binary(number)).collect(),
            output: matches.get_one::<String>("output").unwrap().to_string(),
            image_width: *matches.get_one::<u32>("image-width").unwrap(),
            image_height: *matches.get_one::<u32>("image-height").unwrap(),
            pixel_width: *matches.get_one::<u16>("pixel-width").unwrap(),
            pixel_height: *matches.get_one::<u16>("pixel-height").unwrap(),
            on_color: str_to_rgb(matches.get_one::<String>("on-color").unwrap()),
            off_color: str_to_rgb(matches.get_one::<String>("off-color").unwrap()),
            fill_color: str_to_rgb(matches.get_one::<String>("fill-color").unwrap()),
            remaining_behavior: *matches.get_one::<RemainingBehavior>("remaining-behavior").unwrap(),
        }
    }
}

