use core::slice::Iter;

#[derive(Clone)]
pub struct Config {
    pub allow_trailing_comma: bool,
}

impl Config {
    pub const DEFAULT: Config = Config {
        allow_trailing_comma: false,
    };

    pub fn from_iter(args: &mut Iter<String>) -> Result<Config, String> {
        let mut config = Config::DEFAULT.clone();

        while let Some(args) = args.next() {
            match args.as_str() {
                "--allow-trailing-comma" => config.allow_trailing_comma = true,
                unknown_flag => {
                    return Err(format!("unknown flag: {}", unknown_flag));
                }
            }
        }

        Ok(config)
    }
}
