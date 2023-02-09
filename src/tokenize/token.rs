use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum Token {
    Decl,
    Empty,
    Eof,
    Error(String),
    Identifier,
    Name,
    Number,
    String,
    Type,
    Print,
}
