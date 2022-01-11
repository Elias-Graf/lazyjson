use core::slice::Iter;

#[derive(Clone)]
pub struct Config {
    pub allow_trailing_commas: bool,
    pub allow_line_comments: bool,
}

impl Config {
    pub const DEFAULT: Config = Config {
        allow_trailing_commas: false,
        allow_line_comments: false,
    };

    pub fn from_iter(args: &mut Iter<String>) -> Result<Config, String> {
        let mut config = Config::DEFAULT.clone();

        while let Some(args) = args.next() {
            match args.as_str() {
                "--allow-trailing-commas" => config.allow_trailing_commas = true,
                "--allow-line-comments" => config.allow_line_comments = true,
                unknown_flag => {
                    return Err(format!("unknown flag: {}", unknown_flag));
                }
            }
        }

        Ok(config)
    }
}
