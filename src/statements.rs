use crate::{compiler::Parser, convert::hanzi2num::hanzi2num, opcode, tokenize::token::Token};

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

    let current = parser.current().get_value();

    match current {
        Token::BangEqual
        | Token::EqualEqual
        | Token::BangGreater
        | Token::BangLess
        | Token::Less
        | Token::Greater => binary_if_expression(parser),
        _ => {}
    }
}

pub fn binary_if_expression(parser: &mut Parser) {
    let token = parser.current().get_value().clone();

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
            // 名之曰
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
            } else {
                parser.emit_u8(opcode::DEFINE_LOCAL);
                parser.emit_u8(offset);
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
    declare_variable(parser);
    if parser.get_scope() > 0 {
        return None;
    }

    parser.identifier_constant()
}

fn declare_variable(parser: &mut Parser) {
    if parser.get_scope() == 0 {
        return;
    }

    let token = parser.previous().get_value().clone();
    parser.add_local(token);
}

pub fn assign_statement<'a>(parser: &'a mut Parser) {
    parser.advance(); // skip '昔之'
    parser.advance(); // skip 'variable name'

    let arg = parser.resolve_local(parser.previous().get_value().clone());

    let (x, y) = match arg {
        Some(arg) => (opcode::SET_LOCAL, arg),
        None => (opcode::SET_GLOBAL, parser.identifier_constant().unwrap()),
    };

    parser.consume(Token::Conjunction, "expect '者' in assign statement");
    parser.consume(Token::AssignTo, "expect '今' in assign statement.");
    parser.expression();
    parser.consume(Token::Sure, "expect '是矣' in assign statement.");
    parser.emit_u8(x);
    parser.emit_u32(y);
}

pub fn block_statement<'a, const N: usize>(parser: &'a mut Parser, stop_before_tokens: [Token; N]) {
    parser.begin_scope();
    let preset_stop_tokens = [Token::YunYun, Token::Ye];

    while parser.check_not_in_vec(&stop_before_tokens)
        && parser.check_not_in_vec(&preset_stop_tokens)
        && !parser.check(Token::Eof)
    {
        parser.declaration()
    }

    if parser.check_in_vec(&preset_stop_tokens) {
        parser.advance();
    } else if parser.check_in_vec(&stop_before_tokens) {
    } else {
        parser.error("expect end token in block statement.")
    }

    parser.end_scope();
}

pub fn name_is_statement<'a>(parser: &'a mut Parser) {
    parser.advance();
    let global = parse_variable(parser, "Expect variable name.");
    if let Some(global) = global {
        parser.emit_u8(opcode::DEFINE_GLOBAL);
        parser.emit_u32(global);
        parser.emit_u8(0);
        // pick top of stack and pop top of stack immediately,
        // this is different from normal declaration, because it always choose top of stack.
        parser.emit_u8(opcode::POP);
    } else {
        parser.emit_u8(opcode::DEFINE_LOCAL);
    }
}

pub fn if_statement<'a>(parser: &'a mut Parser) {
    parser.advance();
    parser.statement();
    parser.consume(Token::Conjunction, "expect '者'");
    let then_jump = parser.emit_jump(opcode::JUMP_IF_FALSE);
    parser.emit_u8(opcode::POP);
    block_statement(parser, [Token::Else]);

    if parser.check(Token::Else) {
        parser.advance();
        let else_jump = parser.emit_jump(opcode::JUMP);
        parser.patch_jump(then_jump);

        parser.emit_u8(opcode::POP);
        block_statement(parser, []);
        parser.patch_jump(else_jump);
    } else {
        parser.patch_jump(then_jump);
    }
}

pub fn boolean_algebra_statement<'a>(parser: &'a mut Parser) {
    parser.expression();
    match parser.current().get_value() {
        Token::And => parser.emit_u8(opcode::AND),
        Token::Or => parser.emit_u8(opcode::OR),
        _ => parser.error_at_current("expect '中有陽乎' or '中無陰乎'."),
    }
    parser.advance()
}
