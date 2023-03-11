use crate::{parse::Parse, parse_error::ParseError};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Json {
    String(String),
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
    NumberF(f64),
    NumberI(i64),
}

impl Json {
    pub fn parse(s: String) -> Result<Self, ParseError> {
        let mut parse = Parse::WaitForType;
        for (i, c) in s.chars().enumerate() {
            parse = parse.transition(c).unwrap_or_else(|pe| {
                println!("Position: {} Error: {:?}", i, pe.1);
                pe.0
            });
            // println!("{}:{:?}", c, parse);
        }
        match parse {
            Parse::WaitForType => Err(ParseError("No value!".to_string())),
            Parse::WaitForClosure(v) => Ok(v),
            _ => Err(ParseError("Failed to parse!".to_string())),
        }
    }
}
