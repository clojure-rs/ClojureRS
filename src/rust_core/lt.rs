use crate::error_message;
use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

/// (lt x y)
/// x < y
#[derive(Debug, Clone)]
pub struct LtFn {}
impl ToValue for LtFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for LtFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 2 {
            return error_message::wrong_arg_count(2, args.len());
        }
        match args.get(0).unwrap().to_value() {
            Value::I32(a) => match args.get(1).unwrap().to_value() {
                Value::I32(b) => Value::Boolean(a < b),
                Value::F64(b) => Value::Boolean(a < b as i32),
                b_ => Value::Condition(format!(
                    // TODO: what error message should be returned regarding using typetags?
                    "Type mismatch; Expecting: (i32 | i64 | f32 | f64), Found: {}",
                    b_.type_tag()
                )),
            },
            Value::F64(a) => match args.get(0).unwrap().to_value() {
                Value::I32(b) => Value::Boolean(a < b as f64),
                Value::F64(b) => Value::Boolean(a < b),
                b_ => Value::Condition(format!(
                    // TODO: what error message should be returned regarding using typetags?
                    "Type mismatch; Expecting: (i32 | i64 | f32 | f64), Found: {}",
                    b_.type_tag()
                )),
            },
            a_ => Value::Condition(format!(
                // TODO: what error message should be returned regarding using typetags?
                "Type mismatch; Expecting: (i32 | i64 | f32 | f64), Found: {}",
                a_.type_tag()
            )),
        }
    }
}
