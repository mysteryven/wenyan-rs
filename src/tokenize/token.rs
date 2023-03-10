use serde::Serialize;

#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum Token {
    Decl,
    DeclShort,
    Type,

    Number,
    String,
    True,
    False,
    Identifier,

    Is,     // 曰
    NameIs, // 名之曰

    Print, // 書之

    Plus,  // 加
    Minus, // 減
    Star,  // 乘

    PrepositionLeft,  // 於
    PrepositionRight, // 以

    EqualEqual,  // 等於
    BangEqual,   // 不等於
    Greater,     // 大於
    Less,        // 小於
    BangGreater, // 不大於
    BangLess,    // 不小於

    AssignFrom, // '昔之'
    AssignTo,   // 今
    Prev,       // 其
    Fu,         // 夫
    Invert,     // 變

    Conjunction, // 者
    Sure,        // 是矣
    YunYun,      // 云云
    Ye,          // 也

    If,   // 若
    Else, // 若非

    Loop,   // 恆為是
    For,    // 為是
    ForMid, // 遍
    Break,  // 乃止

    And, // 中無陰乎,
    Or,  // 中有陽乎

    Fun,               // 吾有一術
    FunctionReady,     // 欲行是術
    FunctionArg,       // 必先得
    FunctionBodyBegin, // 是術曰
    FunctionEnd1,      // 是謂
    FunctionEnd2,      // 之術也
    Call,              // 施
    Return,            // 乃得

    Eof,
    Error(String),
}
