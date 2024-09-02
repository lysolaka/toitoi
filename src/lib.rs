use std::error::Error;

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

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn count_lines(contents: &str) -> usize {
    contents.lines().count()
}

fn count_words(contents: &str) -> usize {
    contents.split_whitespace().count()
}

fn count_bytes(contents: &str) -> usize {
    contents.bytes().count()
}

fn count_chars(contents: &str) -> usize {
    contents.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn line_count() {
        let text = "\
        Some random text,
        needs a few lines,
        hopefully 3 is enough,
        nevermind, I'll put 4 for
        SaFeTy, ohhh... it's 5 now";

        assert_eq!(count_lines(text), 5);
    }

    #[test]
    fn word_count() {
        let text = "\
        Some random text,
        needs a few lines,
        hopefully 3 is enough,
        nevermind, I'll put 4 for
        SaFeTy, ohhh... it's 5 now";

        assert_eq!(count_words(text), 21);
    }

    #[test]
    fn byte_count() {
        let s1 = "haha ascii only";
        let s2 = "hąhą ńó ąśćii";

        assert_eq!(count_bytes(s1), 15);
        assert_eq!(count_bytes(s2), 20);
    }

    #[test]
    fn char_count() {
        let s1 = "haha ascii only";
        let s2 = "hąhą ńó ąśćii";

        assert_eq!(count_chars(s1), 15);
        assert_eq!(count_chars(s2), 13);       
    }

}
