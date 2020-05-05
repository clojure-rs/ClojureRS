use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

/// (* x y & xys)
///
#[derive(Debug, Clone)]
pub struct MultiplyFn {}
impl ToValue for MultiplyFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for MultiplyFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        args.into_iter().fold(1_i32.to_value(), |a, b| match a {
            Value::I32(a_) => match *b {
                Value::I32(b_) => Value::I32(a_ * b_),
                Value::F64(b_) => Value::F64(a_ as f64 * b_),
                _ => Value::Condition(format!(
                    // TODO: what error message should be returned regarding using typetags?
                    "Type mismatch; Expecting: (i32 | i64 | f32 | f64), Found: {}",
                    b.type_tag()
                )),
            },
            Value::F64(a_) => match *b {
                Value::I32(b_) => Value::F64(a_ * b_ as f64),
                Value::F64(b_) => Value::F64(a_ * b_),
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
    mod multiply_tests {
        use crate::ifn::IFn;
        use crate::rust_core::_multiply_::MultiplyFn;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn multiply_without_arguments_returns_one() {
            let multiply = MultiplyFn {};
            let args = vec![];
            assert_eq!(Value::I32(1), multiply.invoke(args));
        }

        #[test]
        fn multiply_with_one_argument_returns_identity() {
            let multiply = MultiplyFn {};
            let args = vec![Rc::new(Value::I32(5))];
            assert_eq!(Value::I32(5), multiply.invoke(args));
        }

        #[test]
        fn multiply_with_two_argument_returns_product() {
            let multiply = MultiplyFn {};
            let args = vec![Rc::new(Value::I32(5)), Rc::new(Value::I32(6))];
            assert_eq!(Value::I32(30), multiply.invoke(args));
        }
    }
}
