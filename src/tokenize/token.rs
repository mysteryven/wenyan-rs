use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum Token {
    Decl,
    Define,

    Type,

    Number,
    String,
    Identifier,

    Is, // 曰

    Print, // 書之

    // op
    Plus,  // 加
    Minus, // 减
    Star,  // 乘
    Slash, // 除

    // if logic
    EqualEqual,
    BangEqual,
    Greater,
    Less,
    BangGreater,
    BangLess,

    Empty,
    Eof,
    Error(String),
}
