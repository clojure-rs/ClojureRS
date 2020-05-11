use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;
use crate::type_tag::TypeTag;

/// clojure.string/blank? ; returns true if nil, empty or whitespace
#[derive(Debug, Clone)]
pub struct StartsWithFn {}
impl ToValue for StartsWithFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for StartsWithFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 2 {
            return error_message::wrong_arg_count(2, args.len());
        } else {
            match (
                args.get(0).unwrap().to_value(),
                args.get(1).unwrap().to_value(),
            ) {
                (Value::String(s), Value::String(substring)) => {
                    Value::Boolean(s.starts_with(&substring))
                }
                _a => error_message::type_mismatch(TypeTag::String, &_a.1.to_value()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod reverse_tests {
        use crate::clojure_string::starts_with_qmark_::StartsWithFn;
        use crate::ifn::IFn;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn hello_starts_with_hel() {
            let blank = StartsWithFn {};
            let s = "hello";
            let substring = "hel";
            let args = vec![
                Rc::new(Value::String(String::from(s))),
                Rc::new(Value::String(String::from(substring))),
            ];
            assert_eq!(Value::Boolean(true), blank.invoke(args));
        }

        #[test]
        fn hello_does_not_start_with_leh() {
            let blank = StartsWithFn {};
            let s = "hello";
            let substring = "leh";
            let args = vec![
                Rc::new(Value::String(String::from(s))),
                Rc::new(Value::String(String::from(substring))),
            ];
            assert_eq!(Value::Boolean(false), blank.invoke(args));
        }

        #[test]
        fn hello_starts_with_empty_string() {
            let blank = StartsWithFn {};
            let s = "hello";
            let substring = "";
            let args = vec![
                Rc::new(Value::String(String::from(s))),
                Rc::new(Value::String(String::from(substring))),
            ];
            assert_eq!(Value::Boolean(true), blank.invoke(args));
        }
    }
}
