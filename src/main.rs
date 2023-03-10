use std::{collections::HashMap, fs, time::Instant};

#[derive(Debug)]
enum Json {
    String(String),
    Object(HashMap<String, Json>),
    NumberF(f64),
    NumberI(i64),
}

#[derive(Debug, Clone)]
struct ParseError(String);

type Result<T> = std::result::Result<T, ParseError>;
#[derive(Debug)]
enum Parse {
    WaitForType,
    String(String),
    Object(ParseObject),
    Number(ParseNumber),
    WaitForClosure(Json),
    EndWithComma(Json),
    EndWithBracket(Json),
}

#[derive(Debug)]
enum ParseNumber {
    BeforeDot(String),
    AfterDot(String),
}

impl ParseNumber {
    pub fn new_with_char(c: char) -> Result<Self> {
        match c {
            '-' => Ok(ParseNumber::BeforeDot(c.to_string())),
            '0'..='9' => Ok(ParseNumber::BeforeDot(c.to_string())),
            '.' => Ok(ParseNumber::AfterDot(c.to_string())),
            // '0' => Err(ParseError("Unexpected leading zero!".to_string())),
            _ => Err(ParseError("Unexpected non numeric character!".to_string())),
        }
    }
    pub fn transition(self, c: char) -> Result<Self> {
        match self {
            ParseNumber::BeforeDot(s) => match c {
                '.' => Ok(ParseNumber::AfterDot(push(s, c))),
                '0'..='9' => Ok(ParseNumber::BeforeDot(push(s, c))),
                _ => Err(ParseError("Unexpected nonnumeric character".to_string())),
            },
            ParseNumber::AfterDot(s) => match c {
                '0'..='9' => Ok(ParseNumber::AfterDot(push(s, c))),
                _ => Err(ParseError("Unexpected nonnumeric character".to_string())),
            },
        }
    }
    pub fn to_json(self) -> Result<Json> {
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

fn push(mut s: String, c: char) -> String {
    s.push(c);
    s
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
                c => Ok(Parse::String(push(s, c))),
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

#[derive(Debug)]
enum ParseObject {
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
    pub fn transition(self, c: char) -> Result<Self> {
        match self {
            ParseObject::WaitForKey(hm) => match c {
                '"' => Ok(ParseObject::Key(hm, String::new())),
                '}' => Ok(ParseObject::End(Json::Object(hm))),
                _ => Err(ParseError(
                    "Unexpected character when waitng for key!".to_string(),
                )),
            },
            ParseObject::Key(hm, k) => match c {
                '"' => {
                    if hm.contains_key(&k) {
                        return Err(ParseError("Non unique identifier!".to_string()));
                    } else {
                        Ok(ParseObject::WaitForColon(hm, k))
                    }
                }
                c => Ok(ParseObject::Key(hm, push(k, c))),
            },
            ParseObject::WaitForColon(hm, k) => match c {
                ':' => Ok(ParseObject::Value(hm, k, Box::new(Parse::WaitForType))),
                _ => Err(ParseError(
                    "Unexpected character when waitng for colon!".to_string(),
                )),
            },
            ParseObject::Value(mut hm, k, v) => match v.transition(c)? {
                Parse::EndWithComma(v) => {
                    hm.insert(k, v);
                    Ok(ParseObject::WaitForKey(hm))
                }
                Parse::EndWithBracket(v) => {
                    hm.insert(k, v);
                    Ok(ParseObject::End(Json::Object(hm)))
                }
                other => Ok(ParseObject::Value(hm, k, Box::new(other))),
            },
            ParseObject::End(_) => Err(ParseError("Unexpected trailing characters!".to_string())),
        }
    }
}

impl Json {
    fn parse(s: String) -> Result<Self> {
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

fn main() {
    let json_string = fs::read_to_string("large.json").unwrap();
    let now = Instant::now();
    let _ = Json::parse(json_string).unwrap();
    let elapsed = now.elapsed();
    // println!("{:#?}", json);
    println!("Parsing took: {} [ms]", elapsed.as_millis())
}
