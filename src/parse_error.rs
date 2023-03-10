#[derive(Debug, Clone)]
pub struct ParseError(pub String);

pub type Result<T> = std::result::Result<T, ParseError>;
