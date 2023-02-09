use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
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
