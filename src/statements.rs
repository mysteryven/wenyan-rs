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

pub fn print_statement(parser: &mut Parser) {
    parser.advance();

    parser.emit_u8(opcode::PRINT);
}

pub fn expression_statement(parser: &mut Parser) {
    parser.expression();
}

pub fn binary_if_statement(parser: &mut Parser, token: &Token) {
    let op_code = match token {
        Token::BangEqual | Token::EqualEqual => opcode::EQUAL_EQUAL,
        Token::Greater | Token::BangGreater => opcode::GREATER,
        Token::Less | Token::BangLess => opcode::LESS,
        _ => {
            parser.error_at_current("unknown if binary operator.");
            return;
        }
    };

    parser.advance();
    parser.expression();
    parser.emit_u8(op_code);

    match token {
        Token::BangEqual | Token::BangGreater | Token::BangLess => {
            parser.emit_u8(opcode::INVERT);
        }
        _ => {}
    };
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
