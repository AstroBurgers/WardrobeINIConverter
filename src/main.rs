use std::io;
use std::time::Instant;

mod converter;
mod parser;
mod structs;

use crate::converter::convert;
use crate::parser::parse_file;

fn pause(message: &str) {
    let mut input = String::new();
    println!("{}", message);
    let _ = io::stdin().read_line(&mut input);
}

fn main() {
    println!("Welcome!!");
    pause("Press Enter to continue...");

    let start = Instant::now();
    convert(parse_file("plugins/EUP/Wardrobe.ini"));
    let elapsed = start.elapsed();

    println!(
        "Elapsed Time: {:.2?} — {}s",
        elapsed,
        elapsed.as_secs_f64()
    );
    pause("Press Enter to exit...");
}