use crate::{
    compiler::{Compiler, FunctionType, Parser},
    convert::hanzi2num::hanzi2num,
    opcode::{self},
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

    parser.add_local(parser.get_prev_token_string());
}

pub fn assign_statement<'a>(parser: &'a mut Parser) {
    parser.advance(); // skip '昔之'
    parser.advance(); // skip 'variable name'

    let arg = parser.resolve_local(parser.get_prev_token_string());

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
    } else {
        parser.emit_u8(opcode::DEFINE_LOCAL);
    }

    // pick top of stack and pop top of stack immediately,
    // this is different from normal declaration, because it always choose top of stack.
    parser.emit_u8(0);
    parser.emit_u8(opcode::POP);
}

pub fn if_statement<'a>(parser: &'a mut Parser) {
    parser.advance();
    expression_statement(parser);
    parser.consume(Token::Conjunction, "expect '者'");
    let then_jump = parser.emit_jump(opcode::JUMP_IF_FALSE);
    parser.emit_u8(opcode::POP);
    block_statement(parser, [Token::Else]);

    let else_jump = parser.emit_jump(opcode::JUMP);
    parser.patch_jump(then_jump);
    parser.emit_u8(opcode::POP);

    if parser.check(Token::Else) {
        parser.advance();
        block_statement(parser, []);
    }

    parser.patch_jump(else_jump);
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

pub fn for_while_statement<'a>(parser: &'a mut Parser) {
    let break_jump = parser.emit_jump(opcode::RECORD_BREAK);
    parser.advance();
    let loop_start = parser.current_code_len();
    block_statement(parser, []);
    parser.emit_loop(loop_start);
    parser.patch_jump(break_jump);
    parser.emit_u8(opcode::DISCARD_BREAK);
}

pub fn break_statement<'a>(parser: &'a mut Parser) {
    parser.advance();
    parser.emit_u8(opcode::BREAK)
}

pub fn for_statement<'a>(parser: &'a mut Parser) {
    parser.advance();
    parser.expression();
    parser.consume(Token::ForMid, "expect '遍' in for statement.");

    let name = String::from("inner_for_loop_var");
    parser.begin_scope();

    let break_jump = parser.emit_jump(opcode::RECORD_BREAK);

    // 吾有一數。曰「inner_for_loop_var」。名之曰「inner_for_loop_var」。
    parser.define_local_variable(name.as_str());

    let slot = parser
        .resolve_local(name)
        .expect("should inject temp var into for loop.");

    let loop_start = parser.current_code_len();

    // 「inner_for_loop_var」大於零
    parser.emit_bytes(opcode::GET_LOCAL, slot);
    parser.emit_constant(Value::Number(0.0));
    parser.emit_u8(opcode::GREATER);

    let exit_jump = parser.emit_jump(opcode::JUMP_IF_FALSE);
    parser.emit_u8(opcode::POP);

    let body_jump = parser.emit_jump(opcode::JUMP);
    let increase_start = parser.current_code_len();

    // 減「inner_for_loop_var」以一
    // 昔之「inner_for_loop_var」今其是矣
    parser.emit_bytes(opcode::GET_LOCAL, slot);

    parser.emit_constant(Value::Number(1.0));
    parser.emit_u8(opcode::SUBTRACT);
    parser.emit_u8(opcode::PREPOSITION_RIGHT);
    parser.emit_u8(opcode::SET_LOCAL);
    parser.emit_u32(slot);

    parser.emit_loop(loop_start);
    parser.patch_jump(body_jump);

    block_statement(parser, []);

    parser.emit_loop(increase_start);
    parser.patch_jump(exit_jump);
    parser.emit_u8(opcode::POP);

    parser.patch_jump(break_jump);
    parser.emit_u8(opcode::DISCARD_BREAK);

    parser.end_scope();
}

pub fn fun_statement<'a>(parser: &'a mut Parser) {
    parser.advance();
    parser.consume(Token::NameIs, "expect '名之曰' in function declaration.");
    let global = parse_variable(parser, "expect function name.");

    function(parser, FunctionType::Function);

    parser.emit_bytes(opcode::DEFINE_GLOBAL, global.unwrap());
}

pub fn function<'a>(parser: &'a mut Parser, kind: FunctionType) {
    parser.enter_compiler(kind);
    parser.begin_scope();

    if parser.is_match(Token::FunctionReady) {
        parser.consume(Token::FunctionArg, "expect '必先得'");

        while !parser.check(Token::FunctionBodyBegin) {
            if parser.is_match(Token::Is) {
                parser.current_compiler_mut().function_mut().add_arity(1);
                parse_variable(parser, "expect a parameter name.");
                parser.emit_u8(opcode::DEFINE_LOCAL);
                parser.emit_u8(0);
            } else {
                parser.advance();
            }
        }
    }

    parser.consume(Token::FunctionBodyBegin, "expect '是術曰'.");
    block_statement(parser, [Token::FunctionEnd1]);

    parser.consume(Token::Identifier, "expect identifier in function end.");
    parser.consume(Token::FunctionEnd2, "expect '之術也' in function end.");

    if let Some(function) = parser.end_compiler() {
        let fun_id = parser.add_function(function);
        let id = parser
            .make_constant(Value::Function(fun_id))
            .expect("should be able to make constant");

        parser.emit_bytes(opcode::CONSTANT, id);
    }
}

pub fn call_statement<'a>(parser: &'a mut Parser) {
    parser.advance();
    parser.expression();
    parser.consume(
        Token::PrepositionRight,
        "only support '以'' in function call now.",
    );

    let arg_count = argument_list(parser);
    parser.emit_bytes(opcode::CALL, arg_count);
}

pub fn argument_list<'a>(parser: &'a mut Parser) -> u32 {
    let mut arg_count: u32 = 0;
    while parser.is_literal() {
        parser.expression();
        arg_count += 1;
    }

    arg_count
}

pub fn return_statement(parser: &mut Parser) {
    if parser.current_compiler().fun_kind() == FunctionType::Script {
        parser.error_at_current("cannot return from top-level code.");
        return;
    }

    parser.advance();
    parser.expression();
    parser.emit_u8(opcode::RETURN);
}
