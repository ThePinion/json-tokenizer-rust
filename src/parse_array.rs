use crate::json::Json;
use crate::parse::Parse;
use crate::parse_error::{ParseError, ParseResult};

#[derive(Debug, Clone)]
pub enum ParseArray {
    Value(Vec<Json>, Box<Parse>),
    End(Json),
}

impl ParseArray {
    pub fn new() -> Self {
        ParseArray::Value(vec![], Box::new(Parse::WaitForType))
    }
    pub fn transition(self, c: char) -> ParseResult<Self> {
        match self {
            ParseArray::Value(mut a, v) => match v.transition(c) {
                Ok(v) => match v {
                    Parse::EndWithComma(v) => {
                        a.push(v);
                        Ok(ParseArray::Value(a, Box::new(Parse::WaitForType)))
                    }
                    Parse::EndWithSquareBracket(v) => {
                        a.push(v);
                        Ok(ParseArray::End(Json::Array(a)))
                    }
                    eb @ Parse::EndWithBracket(_) => Err((
                        ParseArray::Value(a, Box::new(eb)),
                        ParseError("Unexpected curly bracket!".to_string()),
                    )),
                    other => Ok(ParseArray::Value(a, Box::new(other))),
                },
                Err((p, e)) => Err((ParseArray::Value(a, Box::new(p)), e)),
            },

            ParseArray::End(_) => Err((
                self,
                ParseError("Unexpected trailing characters!".to_string()),
            )),
        }
    }
}
