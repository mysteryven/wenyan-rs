use crate::{
    interner::StrId,
    object::{ClosureId, FunId},
};

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Nil,
    Number(f64),
    Bool(bool),
    String(StrId),
    Function(FunId),
    Closure(ClosureId),
    UpValue(*const u8),
}

pub fn value_equal(a: Value, b: Value) -> bool {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => a == b,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        _ => false,
    }
}

pub fn is_less(a: Value, b: Value) -> bool {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => a < b,
        _ => false,
    }
}

pub fn is_falsy(value: &Value) -> bool {
    match value {
        Value::Bool(false) => true,
        _ => false,
    }
}

pub fn is_function_or_closure(value: &Value) -> bool {
    match value {
        Value::Function(_) | Value::Closure(_) => true,
        _ => false,
    }
}
