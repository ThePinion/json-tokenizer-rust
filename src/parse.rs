use crate::parse_error::{ParseError, Result};
use crate::utils;
use crate::{json::Json, parse_number::ParseNumber, parse_object::ParseObject};

#[derive(Debug)]
pub enum Parse {
    WaitForType,
    String(String),
    Object(ParseObject),
    Number(ParseNumber),
    WaitForClosure(Json),
    EndWithComma(Json),
    EndWithBracket(Json),
}

impl Parse {
    pub fn transition(self, c: char) -> Result<Self> {
        if let ' ' | '\n' | '\r' = c {
            //TODO: Not that simple :)
            return Ok(self);
        }
        match self {
            Parse::WaitForType => match c {
                '"' => Ok(Parse::String(String::new())),
                '{' => Ok(Parse::Object(ParseObject::new())),
                '0'..='9' | '.' | '-' => Ok(Parse::Number(ParseNumber::new_with_char(c)?)),
                _ => Err(ParseError(
                    "Unexpected character when waiting for type!".to_string(),
                )),
            },
            Parse::String(s) => match c {
                '"' => Ok(Parse::WaitForClosure(Json::String(s))),
                c => Ok(Parse::String(utils::push(s, c))),
            },
            Parse::Number(pn) => match c {
                ',' => Ok(Parse::EndWithComma(pn.to_json()?)),
                '}' => Ok(Parse::EndWithBracket(pn.to_json()?)),
                ' ' | '\n' => Ok(Parse::WaitForClosure(pn.to_json()?)),
                '0'..='9' | '.' => Ok(Parse::Number(pn.transition(c)?)),
                _ => Err(ParseError(
                    "Unexpected character when parsing number!".to_string(),
                )),
            },
            Parse::WaitForClosure(json) => match c {
                ',' => Ok(Parse::EndWithComma(json)),
                '}' => Ok(Parse::EndWithBracket(json)),
                _ => Err(ParseError(
                    "Unexpected character when waiting for closure!".to_string(),
                )),
            },
            Parse::Object(po) => match po.transition(c)? {
                ParseObject::End(v) => Ok(Parse::WaitForClosure(v)),
                other => Ok(Parse::Object(other)),
            },
            Parse::EndWithBracket(_) | Parse::EndWithComma(_) => {
                Err(ParseError("Unexpected trailing characters!".to_string()))
            }
        }
    }
}
