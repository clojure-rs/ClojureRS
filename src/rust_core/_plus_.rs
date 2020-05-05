use crate::ifn::IFn;
use crate::value::{Value, ToValue};
use std::rc::Rc;

/// (+ x y & xys)
///
#[derive(Debug, Clone)]
pub struct AddFn {}
impl ToValue for AddFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for AddFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        args.into_iter().fold(0_i32.to_value(), |a, b| match a {
            Value::I32(a_) => match *b {
                Value::I32(b_) => Value::I32(a_ + b_),
                Value::F64(b_) => Value::F64(a_ as f64 + b_),
                _ => Value::Condition(format!( // TODO: what error message should be returned regarding using typetags?
                                               "Type mismatch; Expecting: (i32 | i64 | f32 | f64), Found: {}",
                                               b.type_tag()
                )),
            },
            Value::F64(a_) => match *b {
                Value::I32(b_) => Value::F64(a_ + b_ as f64),
                Value::F64(b_) => Value::F64(a_ + b_),
                _ => Value::Condition(format!( // TODO: what error message should be returned regarding using typetags?
                                               "Type mismatch; Expecting: (i32 | i64 | f32 | f64), Found: {}",
                                               b.type_tag()
                )),
            },
            _ => Value::Condition(format!( // TODO: what error message should be returned regarding using typetags?
                                           "Type mismatch: Expecting: (i32 | i64 | f32 | f64), Found: {}",
                                           a.type_tag()
            )),
        })
    }
}