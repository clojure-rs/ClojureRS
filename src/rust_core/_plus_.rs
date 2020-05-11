use crate::ifn::IFn;
use crate::value::{ToValue, Value};
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
                _ => Value::Condition(format!(
                    // TODO: what error message should be returned regarding using typetags?
                    "Type mismatch; Expecting: (i32 | i64 | f32 | f64), Found: {}",
                    b.type_tag()
                )),
            },
            Value::F64(a_) => match *b {
                Value::I32(b_) => Value::F64(a_ + b_ as f64),
                Value::F64(b_) => Value::F64(a_ + b_),
                _ => Value::Condition(format!(
                    // TODO: what error message should be returned regarding using typetags?
                    "Type mismatch; Expecting: (i32 | i64 | f32 | f64), Found: {}",
                    b.type_tag()
                )),
            },
            _ => Value::Condition(format!(
                // TODO: what error message should be returned regarding using typetags?
                "Type mismatch: Expecting: (i32 | i64 | f32 | f64), Found: {}",
                a.type_tag()
            )),
        })
    }
}

#[cfg(test)]
mod tests {
    mod plus_tests {
        use crate::ifn::IFn;
        use crate::rust_core::AddFn;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn plus_without_arguments_returns_zero() {
            let addition = AddFn {};
            let args = vec![];
            assert_eq!(Value::I32(0), addition.invoke(args));
        }

        #[test]
        fn plus_with_one_argument_returns_identity() {
            let addition = AddFn {};
            let args = vec![Rc::new(Value::I32(5))];
            assert_eq!(Value::I32(5), addition.invoke(args));
        }

        #[test]
        fn plus_with_two_argument_returns_product() {
            let addition = AddFn {};
            let args = vec![Rc::new(Value::I32(5)), Rc::new(Value::I32(6))];
            assert_eq!(Value::I32(11), addition.invoke(args));
        }
    }
}
