use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;

/// (/ x y & xys)
///
#[derive(Debug, Clone)]
pub struct DivideFn {}
impl ToValue for DivideFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for DivideFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        match args.len() {
            0 => error_message::zero_arg_count(args.len()),
            1 => {
                let val = args.get(0).unwrap().to_value();
                match val {
                    Value::I32(a_) => Value::F64(1.0 / a_ as f64),
                    Value::F64(f_) => Value::F64(1.0 / f_),
                    _ => Value::Condition(format!(
                        // TODO: what error message should be returned regarding using typetags?
                        "Type mismatch; Expecting: (i32 | i64 | f32 | f64), Found: {}",
                        val.type_tag()
                    )),
                }
            }
            _ => {
                let mut args_iterator = args.into_iter();
                let first_arg = args_iterator.next().unwrap();
                args_iterator.fold(first_arg.to_value(), |a, b| match a {
                    Value::I32(a_) => match *b {
                        Value::I32(b_) => Value::I32(a_ / b_),
                        Value::F64(b_) => Value::F64(a_ as f64 / b_),
                        _ => Value::Condition(format!(
                            // TODO: what error message should be returned regarding using typetags?
                            "Type mismatch; Expecting: (i32 | i64 | f32 | f64), Found: {}",
                            b.type_tag()
                        )),
                    },
                    Value::F64(a_) => match *b {
                        Value::I32(b_) => Value::F64(a_ / b_ as f64),
                        Value::F64(b_) => Value::F64(a_ / b_),
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
    }
}

#[cfg(test)]
mod tests {
    mod divide_tests {
        use crate::ifn::IFn;
        use crate::rust_core::_divide_::DivideFn;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn divide_without_arguments_returns_error() {
            let divide = DivideFn {};
            let args = vec![];
            assert_eq!(
                Value::Condition(String::from(
                    "Wrong number of arguments given to function (Given: 0)"
                )),
                divide.invoke(args)
            );
        }

        #[test]
        fn divide_with_one_positive_argument_returns_reciprocal() {
            let divide = DivideFn {};
            let args = vec![Rc::new(Value::I32(5))];
            assert_eq!(Value::F64(1.0 / 5.0), divide.invoke(args));
        }

        #[test]
        fn divide_with_one_negative_argument_returns_reciprocal() {
            let divide = DivideFn {};
            let args = vec![Rc::new(Value::I32(-5))];
            assert_eq!(Value::F64(1.0 / -5.0), divide.invoke(args));
        }

        #[test]
        fn divide_with_two_integer_argument_returns_quotient() {
            let divide = DivideFn {};
            let args = vec![Rc::new(Value::I32(24)), Rc::new(Value::I32(6))];
            assert_eq!(Value::I32(4), divide.invoke(args));
        }

        #[test]
        fn divide_with_one_double_argument_returns_quotient() {
            let divide = DivideFn {};
            let args = vec![Rc::new(Value::I32(24)), Rc::new(Value::F64(1.5))];
            assert_eq!(Value::F64(16.0), divide.invoke(args));
        }

        #[test]
        fn divide_with_multiple_integer_arguments_returns_quotient() {
            let divide = DivideFn {};
            let args = vec![
                Rc::new(Value::I32(100)),
                Rc::new(Value::I32(5)),
                Rc::new(Value::I32(4)),
            ];
            assert_eq!(Value::I32(5), divide.invoke(args));
        }

        #[test]
        fn divide_with_multiple_mixed_arguments_returns_quotient() {
            let divide = DivideFn {};
            let args = vec![
                Rc::new(Value::I32(100)),
                Rc::new(Value::I32(5)),
                Rc::new(Value::I32(4)),
                Rc::new(Value::F64(2.0)),
            ];
            assert_eq!(Value::F64(2.5), divide.invoke(args));
        }
    }
}
