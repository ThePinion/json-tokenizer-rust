use crate::json::Json;
use crate::parse::Parse;
use crate::parse_error::{ParseError, Result};

#[derive(Debug)]
pub enum ParseArray {
    Value(Vec<Json>, Box<Parse>),
    End(Json),
}

impl ParseArray {
    pub fn new() -> Self {
        ParseArray::Value(vec![], Box::new(Parse::WaitForType))
    }
    pub fn transition(self, c: char) -> Result<Self> {
        match self {
            ParseArray::Value(mut a, v) => match v.transition(c)? {
                Parse::EndWithComma(v) => {
                    a.push(v);
                    Ok(ParseArray::Value(a, Box::new(Parse::WaitForType)))
                }
                Parse::EndWithSquareBracket(v) => {
                    a.push(v);
                    Ok(ParseArray::End(Json::Array(a)))
                }
                Parse::EndWithBracket(_) => {
                    Err(ParseError("Unexpected curly bracket!".to_string()))
                }
                other => Ok(ParseArray::Value(a, Box::new(other))),
            },
            ParseArray::End(_) => Err(ParseError("Unexpected trailing characters!".to_string())),
        }
    }
}
