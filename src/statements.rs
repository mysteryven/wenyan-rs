use crate::{compiler::Parser, opcode, tokenize::token::Token};

pub fn unary_statement(parser: &mut Parser, token: &Token) {
    parser.advance();

    parser.expression();

    match token {
        Token::Invert => parser.emit_u8(opcode::INVERT),
        _ => {
            eprintln!("unreachable")
        }
    }
}

pub fn expression_statement(parser: &mut Parser) {
    parser.expression();
}

pub fn binary_statement(parser: &mut Parser, token: &Token) {
    parser.advance();

    parser.expression();
    let mut op_code = None;
    if parser.is_match(Token::PrepositionLeft) {
        op_code = Some(opcode::PREPOSITION_LEFT)
    } else if parser.is_match(Token::PrepositionRight) {
        op_code = Some(opcode::PREPOSITION_RIGHT);
    };

    parser.expression();

    if op_code.is_none() {
        parser.error("invalid preposition, you should use '於' or '以'.");
        return;
    }

    match token {
        Token::Plus => Some(opcode::ADD),
        Token::Minus => Some(opcode::SUBTRACT),
        Token::Star => Some(opcode::MULTIPLY),
        _ => None,
    }
    .map(|op_code| parser.emit_u8(op_code));

    parser.emit_u8(op_code.unwrap());
}
