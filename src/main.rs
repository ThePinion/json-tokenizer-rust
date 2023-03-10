use std::{fs, time::Instant};

use crate::json::Json;

mod json;
mod parse;
mod parse_error;
mod parse_number;
mod parse_object;
mod utils;

fn main() {
    let json_string = fs::read_to_string("large.json").unwrap();
    let now = Instant::now();
    let _ = Json::parse(json_string).unwrap();
    let elapsed = now.elapsed();
    // println!("{:#?}", json);
    println!("Parsing took: {} [ms]", elapsed.as_millis())
}
