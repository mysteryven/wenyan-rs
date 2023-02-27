use crate::{
    compiler::Parser,
    convert::hanzi2num::{self, hanzi2num},
    opcode,
    tokenize::token::Token,
    value::Value,
};

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

pub fn normal_declaration<'a>(parser: &'a mut Parser, buf: &'a str) {
    parser.advance();
    let start = parser.previous().get_start();
    let end = parser.previous().get_end();
    let num = match hanzi2num(&buf[start..end]).map(|s| s.parse::<f64>()) {
        Some(res) => match res {
            Ok(value) => Some(value as u8),
            Err(_) => None,
        },
        None => None,
    };

    // skip strict type judgment for now
    parser.advance();

    if let Some(num) = num {
        for _ in 0..num {
            parser.consume(Token::Is, "expect '曰' in declaration.");
            parser.expression()
        }

        let mut offset = (num - 1) as u8;
        let mut has_define_statement = false;
        while parser.is_match(Token::NameIs) {
            has_define_statement = true;
            let global = parse_variable(parser, "Expect variable name.");
            if let Some(global) = global {
                parser.emit_u8(opcode::DEFINE_GLOBAL);
                parser.emit_u32(global);
                parser.emit_u8(offset);
                if offset > 0 {
                    offset -= 1;
                } else {
                    break;
                }
            }
        }

        if has_define_statement {
            if offset != 0 {
                parser.error("expect named all variable you declared.")
            } else {
                for _ in 0..num {
                    parser.emit_u8(opcode::POP);
                }
            }
        }
    } else {
        parser.error("expect a number in declaration.");
    }
}

fn parse_variable(parser: &mut Parser, error: &str) -> Option<u32> {
    parser.consume(Token::Identifier, error);

    parser.identifier_constant()
}

pub fn assign_statement<'a>(parser: &'a mut Parser) {
    parser.advance(); // skip '昔之'
    parser.advance(); // skip 'variable name'
    let arg = parser.identifier_constant().unwrap();
    parser.consume(Token::Conjunction, "expect '者' in assign statement");
    parser.consume(Token::AssignTo, "expect '今' in assign statement.");
    parser.expression();
    parser.consume(Token::Sure, "expect '是矣' in assign statement.");
    parser.emit_u8(opcode::SET_GLOBAL);
    parser.emit_u32(arg);
}
