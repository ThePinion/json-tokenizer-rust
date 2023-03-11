use std::collections::HashMap;

use crate::json::Json;
use crate::parse::Parse;
use crate::parse_error::{ParseError, ParseResult};
use crate::utils;

#[derive(Debug, Clone)]
pub enum ParseObject {
    WaitForKey(HashMap<String, Json>),
    Key(HashMap<String, Json>, String),
    WaitForColon(HashMap<String, Json>, String),
    Value(HashMap<String, Json>, String, Box<Parse>),
    End(Json),
}

impl ParseObject {
    pub fn new() -> Self {
        ParseObject::WaitForKey(HashMap::new())
    }
    pub fn transition(self, c: char) -> ParseResult<Self> {
        match self {
            ParseObject::WaitForKey(hm) => match c {
                '"' => Ok(ParseObject::Key(hm, String::new())),
                '}' => Ok(ParseObject::End(Json::Object(hm))),
                _ => Err((
                    ParseObject::WaitForKey(hm),
                    ParseError("Unexpected character when waitng for key!".to_string()),
                )),
            },
            ParseObject::Key(hm, k) => match c {
                '"' => {
                    if hm.contains_key(&k) {
                        return Err((
                            ParseObject::Key(hm, k),
                            ParseError("Non unique identifier!".to_string()),
                        ));
                    } else {
                        Ok(ParseObject::WaitForColon(hm, k))
                    }
                }
                c => Ok(ParseObject::Key(hm, utils::push(k, c))),
            },
            ParseObject::WaitForColon(hm, k) => match c {
                ':' => Ok(ParseObject::Value(hm, k, Box::new(Parse::WaitForType))),
                _ => Err((
                    ParseObject::WaitForColon(hm, k),
                    ParseError("Unexpected character when waitng for colon!".to_string()),
                )),
            },
            ParseObject::Value(mut hm, k, v) => match v.transition(c) {
                Ok(v) => match v {
                    Parse::EndWithComma(v) => {
                        hm.insert(k, v);
                        Ok(ParseObject::WaitForKey(hm))
                    }
                    Parse::EndWithBracket(v) => {
                        hm.insert(k, v);
                        Ok(ParseObject::End(Json::Object(hm)))
                    }
                    pewsb @ Parse::EndWithSquareBracket(_) => Err((
                        ParseObject::Value(hm, k, Box::new(pewsb)),
                        ParseError("Unexpected square bracket!".to_string()),
                    )),
                    other => Ok(ParseObject::Value(hm, k, Box::new(other))),
                },
                Err((p, e)) => Err((ParseObject::Value(hm, k, Box::new(p)), e)),
            },
            end @ ParseObject::End(_) => Err((
                end,
                ParseError("Unexpected trailing characters!".to_string()),
            )),
        }
    }
}
