use serde::Serialize;

#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum Token {
    Decl,
    DeclShort,
    Define,

    Type,

    Number,
    String,
    True,
    False,
    Identifier,

    Is,     // 曰
    NameIs, // 名之曰

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

    AssignFrom, //  '昔之'
    AssignTo,   // 今
    Prev,       // 其

    Conjunction, // 者
    Sure,        // 是矣
    YunYun,      // 云云
    Ye,          // 也

    If,   // 若
    Else, // 若非

    Empty,
    Eof,
    Error(String),
    LeftBlock,
    RightBlock,
}
