use std::{fs, time::Instant};

use crate::json::Json;

mod json;
mod parse;
mod parse_array;
mod parse_error;
mod parse_number;
mod parse_object;
mod utils;

fn generate_test(size: usize, depth: i8) -> String {
    let mut output = format!("{{");
    for i in 0..(size - 1) {
        output.push_str(&format!("\"test{}\": ", i));
        if depth > 0 {
            output.push_str(&generate_test(size, depth - 1));
        } else {
            output.push_str("0")
        }
        output.push_str(", \n");
    }
    output.push_str(&format!("\"test{}\": 1", size));
    output.push_str(&format!("}}"));
    output
}

fn main() {
    let _json_string = fs::read_to_string("array.json").unwrap();
    let now = Instant::now();
    let _json = Json::parse(generate_test(100, 2)).unwrap();
    let elapsed = now.elapsed();
    // println!("{:#?}", json);
    println!("Parsing took: {} [ms]", elapsed.as_millis());
}
