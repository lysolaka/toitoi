use std::{env, process};

use toitoi::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    if let Err(e) = toitoi::run(&config) {
        eprintln!("{e}");
        process::exit(1);
    }
}
