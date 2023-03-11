#[derive(Debug, Clone)]
pub struct ParseError(pub String);

pub type ParseResult<T> = std::result::Result<T, (T, ParseError)>;
