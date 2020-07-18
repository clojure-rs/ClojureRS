use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;

/// (rem x y)
/// Calculate remainder
#[derive(Debug, Clone)]
pub struct RemFn {}
impl ToValue for RemFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for RemFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        match args.len() {
            2 => {
                match args.get(0).unwrap().to_value() {
                    Value::I32(a_) => match args.get(1).unwrap().to_value() {
                        Value::I32(0) => Value::Condition("Divide by zero".to_string()),
                        Value::F64(0.0) => Value::Condition("Divide by zero".to_string()),
                        Value::I32(b_) => Value::I32(a_ % b_),
                        Value::F64(b_) => Value::F64(a_ as f64 % b_),
                        _b => Value::Condition(format!(
                            // TODO: what error message should be returned regarding using typetags?
                            "Type mismatch; Expecting: (i32 | i64 | f32 | f64), Found: {}",
                            _b.type_tag()
                        )),
                    },
                    Value::F64(a_) => match args.get(1).unwrap().to_value() {
                        Value::I32(0) => Value::Condition("Divide by zero".to_string()),
                        Value::F64(0.0) => Value::Condition("Divide by zero".to_string()),
                        Value::I32(b_) => Value::F64(a_ % b_ as f64),
                        Value::F64(b_) => Value::F64(a_ % b_),
                        _b => Value::Condition(format!(
                            // TODO: what error message should be returned regarding using typetags?
                            "Type mismatch; Expecting: (i32 | i64 | f32 | f64), Found: {}",
                            _b.type_tag()
                        )),
                    },
                    _a => Value::Condition(format!(
                        // TODO: what error message should be returned regarding using typetags?
                        "Type mismatch: Expecting: (i32 | i64 | f32 | f64), Found: {}",
                        _a.type_tag()
                    )),
                }
            }
            _ => error_message::wrong_arg_count(2, args.len()),
        }
    }
}

#[cfg(test)]
mod tests {
    mod rem_tests {
        use crate::ifn::IFn;
        use crate::rust_core::rem::RemFn;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn rem_without_arguments_returns_error() {
            let rem = RemFn {};
            let args = vec![];
            assert_eq!(
                Value::Condition(String::from(
                    "Wrong number of arguments given to function (Given: 0, Expected: 2)"
                )),
                rem.invoke(args)
            );
        }

        #[test]
        fn rem_with_two_integer_argument_returns_remainder() {
            let rem = RemFn {};
            let args = vec![Rc::new(Value::I32(10)), Rc::new(Value::I32(3))];
            assert_eq!(Value::I32(1), rem.invoke(args));
        }
    }
}
