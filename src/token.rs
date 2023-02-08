#[derive(Clone, Debug)]
pub enum Token {
    Decl,
    Type,
    Number,
    String,
    Identifier,
    Print,
    Empty,
    Name,
    Eof,
    Error(String),
}
