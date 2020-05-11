use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;
use crate::type_tag::TypeTag;

/// clojure.string/blank? ; returns true if nil, empty or whitespace
#[derive(Debug, Clone)]
pub struct EndsWithFn {}
impl ToValue for EndsWithFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for EndsWithFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 2 {
            return error_message::wrong_arg_count(2, args.len());
        } else {
            match (
                args.get(0).unwrap().to_value(),
                args.get(1).unwrap().to_value(),
            ) {
                (Value::String(s), Value::String(substring)) => {
                    Value::Boolean(s.ends_with(&substring))
                }
                _a => error_message::type_mismatch(TypeTag::String, &_a.1.to_value()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod reverse_tests {
        use crate::clojure_string::ends_with_qmark_::EndsWithFn;
        use crate::ifn::IFn;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn hello_ends_with_lo() {
            let blank = EndsWithFn {};
            let s = "hello";
            let substring = "lo";
            let args = vec![
                Rc::new(Value::String(String::from(s))),
                Rc::new(Value::String(String::from(substring))),
            ];
            assert_eq!(Value::Boolean(true), blank.invoke(args));
        }

        #[test]
        fn hello_does_not_end_with_klo() {
            let blank = EndsWithFn {};
            let s = "hello";
            let substring = "klo";
            let args = vec![
                Rc::new(Value::String(String::from(s))),
                Rc::new(Value::String(String::from(substring))),
            ];
            assert_eq!(Value::Boolean(false), blank.invoke(args));
        }

        #[test]
        fn hello_ends_with_empty_string() {
            let blank = EndsWithFn {};
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
