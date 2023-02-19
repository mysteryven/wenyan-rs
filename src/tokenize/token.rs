use serde::Serialize;

#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum Token {
    Decl,
    Define,

    Type,

    Number,
    String,
    True,
    False,
    Identifier,

    Is, // 曰

    Print, // 書之

    // ARITH_BINARY_OP
    Plus,  // 加
    Minus, // 減
    Star,  // 乘

    // preposition
    PrepositionLeft,  // 於
    PrepositionRight, // 以

    // Unary OP
    Invert, // 變

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
