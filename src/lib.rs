use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub help: bool,
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
            }
            Some(s) if s == "-h" || s == "--help" => {
                return Ok(Config {
                    help: true,
                    chars: false,
                    file_path: String::from(""),
                })
            }
            Some(_) => false,
            None => return Err("Not enough arguments provided!"),
        };

        let file_path = match arg {
            Some(s) => s,
            None => return Err("No filename provided!"),
        };

        Ok(Config {
            help: false,
            chars,
            file_path,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.help {
        println!("Usage: toitoi [OPTION] [FILE]");
        println!("Print line, word, byte count for FILE\n");
        println!("  -c, --chars  Print the number of characters instead of bytes");
        println!("  -h, --help   Display this help and exit");
        return Ok(())
    }

    let contents = fs::read_to_string(&config.file_path)?;

    let line_count = count_lines(&contents);
    let word_count = count_words(&contents);
    match config.chars {
        true => {
            let char_count = count_chars(&contents);
            println!("File: {}", config.file_path);
            println!("Lines: {}", line_count);
            println!("Words: {}", word_count);
            println!("Characters: {}", char_count);
        }
        false => {
            let byte_count = count_bytes(&contents);
            println!("File: {}", config.file_path);
            println!("Lines: {}", line_count);
            println!("Words: {}", word_count);
            println!("Bytes: {}", byte_count);
        }
    }
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
