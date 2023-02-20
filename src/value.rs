#[derive(Debug, Clone, Copy)]
pub enum Value {
    Number(f64),
    Bool(bool),
}

pub fn value_equal(a: Value, b: Value) -> bool {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => a == b,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        _ => false,
    }
}

pub fn is_less(a: Value, b: Value) -> bool {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => a < b,
        _ => false,
    }
}
