use crate::{
    parse::Parse,
    parse_error::{ParseError, Result},
};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Json {
    String(String),
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
    NumberF(f64),
    NumberI(i64),
}

impl Json {
    pub fn parse(s: String) -> Result<Self> {
        let mut parse = Parse::WaitForType;
        for c in s.chars() {
            parse = parse.transition(c)?;
            // println!("{}:{:?}", c, parse);
        }
        match parse {
            Parse::WaitForType => Err(ParseError("No value!".to_string())),
            Parse::WaitForClosure(v) => Ok(v),
            _ => Err(ParseError("Failed to parse!".to_string())),
        }
    }
}
