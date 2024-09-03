use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub help: bool,
    pub chars: bool,
    pub file_paths: Vec<String>,
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
                    file_paths: vec![String::from("")],
                })
            }
            Some(_) => false,
            None => return Err("Not enough arguments provided!"),
        };

        match arg {
            Some(s) if s == "-h" || s == "--help" => {
                return Ok(Config {
                    help: true,
                    chars: false,
                    file_paths: vec![String::from("")],
                })
            }
            _ => (),
        }

        let mut file_paths: Vec<String> = Vec::new();
        while let Some(s) = arg {
            file_paths.push(s);
            arg = args.next();
        }

        if file_paths.len() == 0 {
            return Err("No filename provided!");
        }

        Ok(Config {
            help: false,
            chars,
            file_paths,
        })
    }
}

struct Counts {
    lines: usize,
    words: usize,
    bc: usize,
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.help {
        println!("Usage: toitoi [OPTION] [FILE]");
        println!("Print line, word, byte count for FILE\n");
        println!("  -c, --chars  Print the number of characters instead of bytes");
        println!("  -h, --help   Display this help and exit");
        return Ok(());
    }

    if config.chars {
        println!("L:  W:  C:  File:");
    } else {
        println!("L:  W:  B:  File:")
    }

    let mut total = Counts {
        lines: 0,
        words: 0,
        bc: 0,
    };

    for path in &config.file_paths {
        let contents = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) if e.raw_os_error().unwrap_or_default() == 21 => {
                eprintln!("{} is a directory.", path);
                continue;
            }
            Err(e) => return Err(Box::new(e)),
        };

        let count = Counts {
            lines: count_lines(&contents),
            words: count_words(&contents),
            bc: if config.chars {
                count_chars(&contents)
            } else {
                count_bytes(&contents)
            },
        };

        println!("{}  {}  {}  {}", count.lines, count.words, count.bc, path);

        total.lines += count.lines;
        total.words += count.words;
        total.bc += count.bc;
    }

    println!("{}  {}  {}  total", total.lines, total.words, total.bc);

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
