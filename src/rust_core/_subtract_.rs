use crate::ifn::IFn;
use crate::value::{Value, ToValue};
use std::rc::Rc;

use crate::error_message;

/// (- x y & xys)
///
#[derive(Debug, Clone)]
pub struct SubtractFn {}
impl ToValue for SubtractFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for SubtractFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        match args.len() {
            0 =>  { error_message::zero_arg_count(args.len()) },
            1 => {
                let val = args.get(0).unwrap().to_value();
                match val {
                    Value::I32(a_) => Value::I32(-a_),
                    Value::F64(f_) => Value::F64(-f_),
                    _ => Value::Condition(format!( // TODO: what error message should be returned regarding using typetags?
                                                   "Type mismatch; Expecting: (i32 | i64 | f32 | f64), Found: {}",
                                                   val.type_tag()
                    ))
                }
            },
            _ => {
                let mut args_iterator = args.into_iter();
                let first_arg = args_iterator.next().unwrap();
                args_iterator.fold(first_arg.to_value(), |a, b| match a {
                    Value::I32(a_) => match *b {
                        Value::I32(b_) => Value::I32(a_ - b_),
                        Value::F64(b_) => Value::F64(a_ as f64 - b_),
                        _ => Value::Condition(format!( // TODO: what error message should be returned regarding using typetags?
                                                       "Type mismatch; Expecting: (i32 | i64 | f32 | f64), Found: {}",
                                                       b.type_tag()
                        )),
                    },
                    Value::F64(a_) => match *b {
                        Value::I32(b_) => Value::F64(a_ - b_ as f64),
                        Value::F64(b_) => Value::F64(a_ - b_),
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
    }
}

#[cfg(test)]
mod tests {
    mod subtract_tests {
        use crate::rust_core::_subtract_::SubtractFn;
        use crate::ifn::IFn;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn subtract_without_arguments_returns_one() {
            let subtract = SubtractFn {};
            let args = vec![];
            assert_eq!(Value::Condition(String::from("Wrong number of arguments given to function (Given: 0)")),
                       subtract.invoke(args));
        }

        #[test]
        fn subtract_with_one_positive_argument_returns_negated_value() {
            let subtract = SubtractFn {};
            let args = vec![Rc::new(Value::I32(5))];
            assert_eq!(Value::I32(-5), subtract.invoke(args));
        }

        #[test]
        fn subtract_with_one_negative_argument_returns_positive_value() {
            let subtract = SubtractFn {};
            let args = vec![Rc::new(Value::I32(-5))];
            assert_eq!(Value::I32(5), subtract.invoke(args));
        }

        #[test]
        fn subtract_with_two_argument_returns_negative_difference() {
            let subtract = SubtractFn {};
            let args = vec![Rc::new(Value::I32(5)), Rc::new(Value::I32(6))];
            assert_eq!(Value::I32(-1), subtract.invoke(args));
        }

        #[test]
        fn subtract_with_two_argument_returns_positive_difference() {
            let subtract = SubtractFn {};
            let args = vec![Rc::new(Value::I32(6)), Rc::new(Value::I32(5))];
            assert_eq!(Value::I32(1), subtract.invoke(args));
        }

        #[test]
        fn subtract_with_multiple_arguments_returns_difference_case1() {
            let subtract = SubtractFn {};
            let args = vec![
                Rc::new(Value::I32(-3)),
                Rc::new(Value::I32(7)),
                Rc::new(Value::I32(7)),
                Rc::new(Value::I32(4))];
            assert_eq!(Value::I32(-21), subtract.invoke(args));
        }

        #[test]
        fn subtract_with_multiple_arguments_returns_difference_case2() {
            let subtract = SubtractFn {};
            let args = vec![
                Rc::new(Value::I32(-3)),
                Rc::new(Value::I32(7)),
                Rc::new(Value::I32(-7)),
                Rc::new(Value::I32(-4))];
            assert_eq!(Value::I32(1), subtract.invoke(args));
        }
    }
}