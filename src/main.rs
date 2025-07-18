mod parser;
mod converter;
mod structs;

extern crate simple_stopwatch;
use simple_stopwatch::Stopwatch;

use crate::converter::convert;
use crate::parser::*;

fn main() {
    let sw = Stopwatch::start_new();

    let parsed_shit = parse_file("C:/Users/silas/OneDrive/Desktop/Projects/WardrobeINIConverter/target/debug/wardrobe.ini");
    convert(parsed_shit);

    let elapsed_ms = sw.ms();
    println!("Time taken: {}ms — {}s", elapsed_ms, elapsed_ms / 1000.0);
}
