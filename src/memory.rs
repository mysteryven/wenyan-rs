use crate::interpreter::Runtime;

pub fn free_object(runtime: &mut Runtime) {
    let interner = runtime.interner_mut();
    interner.free();
}
