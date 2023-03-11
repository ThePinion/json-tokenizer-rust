use crate::{
    json::Json,
    parse_error::{ParseError, ParseResult},
    utils,
};

#[derive(Debug, Clone)]
pub enum ParseNumber {
    BeforeDot(String),
    AfterDot(String),
}

impl ParseNumber {
    pub fn new_with_char(c: char) -> Result<Self, ParseError> {
        match c {
            '-' => Ok(ParseNumber::BeforeDot(c.to_string())),
            '0'..='9' => Ok(ParseNumber::BeforeDot(c.to_string())),
            '.' => Ok(ParseNumber::AfterDot(c.to_string())),
            // '0' => Err(ParseError("Unexpected leading zero!".to_string())),
            _ => Err(ParseError("Unexpected non numeric character!".to_string())),
        }
    }
    pub fn transition(self, c: char) -> ParseResult<Self> {
        match self {
            ParseNumber::BeforeDot(s) => match c {
                '.' => Ok(ParseNumber::AfterDot(utils::push(s, c))),
                '0'..='9' => Ok(ParseNumber::BeforeDot(utils::push(s, c))),
                _ => Err((
                    ParseNumber::BeforeDot(s),
                    ParseError("Unexpected nonnumeric character".to_string()),
                )),
            },
            ParseNumber::AfterDot(s) => match c {
                '0'..='9' => Ok(ParseNumber::AfterDot(utils::push(s, c))),
                _ => Err((
                    ParseNumber::AfterDot(s),
                    ParseError("Unexpected nonnumeric character".to_string()),
                )),
            },
        }
    }
    pub fn to_json(&self) -> Result<Json, ParseError> {
        match self {
            ParseNumber::BeforeDot(s) => match &str::parse(&s) {
                Ok(n) => Ok(Json::NumberI(*n)),
                Err(e) => Err(ParseError(e.to_string())),
            },
            ParseNumber::AfterDot(s) => match &str::parse(&s) {
                Ok(n) => Ok(Json::NumberF(*n)),
                Err(e) => Err(ParseError(e.to_string())),
            },
        }
    }
}
