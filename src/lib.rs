#[derive(Debug)]
pub struct Config {
    pub chars: bool,
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let mut arg = args.next();

        let chars = match arg {
            Some(s) if s == "-c" || s == "--chars" => {
                arg = args.next();
                true
            },
            Some(_) => false,
            None => return Err("Not enough arguments provided!"),
        };

        let file_path = match arg {
            Some(s) => s,
            None => return Err("No filename provided!"),
        };

        Ok(Config { chars, file_path })
    }
}
