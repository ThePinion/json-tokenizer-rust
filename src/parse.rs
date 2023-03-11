use crate::parse_array::ParseArray;
use crate::parse_error::{ParseError, ParseResult};
use crate::utils;
use crate::{json::Json, parse_number::ParseNumber, parse_object::ParseObject};

#[derive(Debug, Clone)]
pub enum Parse {
    WaitForType,
    String(String),
    Object(ParseObject),
    Array(ParseArray),
    Number(ParseNumber),
    WaitForClosure(Json),
    EndWithComma(Json),
    EndWithBracket(Json),
    EndWithSquareBracket(Json),
}

impl Parse {
    pub fn transition(self, c: char) -> ParseResult<Self> {
        if let ' ' | '\n' | '\r' = c {
            //TODO: Not that simple :)
            return Ok(self);
        }
        match self {
            Parse::WaitForType => match c {
                '"' => Ok(Parse::String(String::new())),
                '{' => Ok(Parse::Object(ParseObject::new())),
                '[' => Ok(Parse::Array(ParseArray::new())),
                '0'..='9' | '.' | '-' => match ParseNumber::new_with_char(c) {
                    Ok(v) => Ok(Parse::Number(v)),
                    Err(e) => Err((self, e)),
                },
                _ => Err((
                    self,
                    ParseError("Unexpected character when waiting for type!".to_string()),
                )),
            },
            Parse::String(s) => match c {
                '"' => Ok(Parse::WaitForClosure(Json::String(s))),
                c => Ok(Parse::String(utils::push(s, c))),
            },
            Parse::Number(pn) => match c {
                ',' => match pn.to_json() {
                    Ok(v) => Ok(Parse::EndWithComma(v)),
                    Err(e) => Err((Parse::Number(pn), e)),
                },
                '}' => match pn.to_json() {
                    Ok(v) => Ok(Parse::EndWithBracket(v)),
                    Err(e) => Err((Parse::Number(pn), e)),
                },
                ']' => match pn.to_json() {
                    Ok(v) => Ok(Parse::EndWithSquareBracket(v)),
                    Err(e) => Err((Parse::Number(pn), e)),
                },
                ' ' | '\n' => match pn.to_json() {
                    Ok(v) => Ok(Parse::WaitForClosure(v)),
                    Err(e) => Err((Parse::Number(pn), e)),
                },
                '0'..='9' | '.' => match pn.transition(c) {
                    Ok(v) => Ok(Parse::Number(v)),
                    Err(_) => todo!(),
                },
                _ => Err((
                    Parse::Number(pn),
                    ParseError("Unexpected character when parsing number!".to_string()),
                )),
            },
            Parse::WaitForClosure(json) => match c {
                ',' => Ok(Parse::EndWithComma(json)),
                '}' => Ok(Parse::EndWithBracket(json)),
                ']' => Ok(Parse::EndWithSquareBracket(json)),
                _ => Err((
                    Parse::WaitForClosure(json),
                    ParseError("Unexpected character when waiting for closure!".to_string()),
                )),
            },
            Parse::Object(po) => match po.transition(c) {
                Ok(ParseObject::End(v)) => Ok(Parse::WaitForClosure(v)),
                Ok(other) => Ok(Parse::Object(other)),
                Err((po, e)) => Err((Parse::Object(po), e)),
            },
            Parse::Array(pa) => match pa.transition(c) {
                Ok(ParseArray::End(v)) => Ok(Parse::WaitForClosure(v)),
                Ok(other) => Ok(Parse::Array(other)),
                Err((pa, e)) => Err((Parse::Array(pa), e)),
            },
            Parse::EndWithBracket(_) | Parse::EndWithComma(_) | Parse::EndWithSquareBracket(_) => {
                Err((
                    self,
                    ParseError("Unexpected trailing characters!".to_string()),
                ))
            }
        }
    }
}
