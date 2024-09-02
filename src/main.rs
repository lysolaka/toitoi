use std::{env, process};

use toitoi::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    println!("{config:?}");
}
