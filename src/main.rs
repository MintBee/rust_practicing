use std::env;
use std::process;

use rust_practicing::post_practicing;
use rust_practicing::config::{Config, print_file_text};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Wrong parameter by this reason: {}", err);
        process::exit(1);
    });

    if let Err(e) = print_file_text(config) {
        println!("Can't read file by this reason: {}", e);
    }
}