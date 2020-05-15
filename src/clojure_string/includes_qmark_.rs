use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;
use crate::type_tag::TypeTag;

/// clojure.string/includes? ; returns true if string contains substring
#[derive(Debug, Clone)]
pub struct IncludesFn {}
impl ToValue for IncludesFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for IncludesFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 2 {
            return error_message::wrong_arg_count(2, args.len());
        } else {
            match (
                args.get(0).unwrap().to_value(),
                args.get(1).unwrap().to_value(),
            ) {
                (Value::String(s), Value::String(substring)) => {
                    Value::Boolean(s.contains(&substring))
                }
                _a => error_message::type_mismatch(TypeTag::String, &_a.1.to_value()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod includes_tests {
        use crate::clojure_string::includes_qmark_::IncludesFn;
        use crate::ifn::IFn;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn hello_includes_ell() {
            let includes = IncludesFn {};
            let s = "hello";
            let substring = "ell";
            let args = vec![
                Rc::new(Value::String(String::from(s))),
                Rc::new(Value::String(String::from(substring))),
            ];
            assert_eq!(Value::Boolean(true), includes.invoke(args));
        }

        #[test]
        fn hello_does_not_include_leh() {
            let includes = IncludesFn {};
            let s = "hello";
            let substring = "leh";
            let args = vec![
                Rc::new(Value::String(String::from(s))),
                Rc::new(Value::String(String::from(substring))),
            ];
            assert_eq!(Value::Boolean(false), includes.invoke(args));
        }

        #[test]
        fn hello_includes_empty_string() {
            let includes = IncludesFn {};
            let s = "hello";
            let substring = "";
            let args = vec![
                Rc::new(Value::String(String::from(s))),
                Rc::new(Value::String(String::from(substring))),
            ];
            assert_eq!(Value::Boolean(true), includes.invoke(args));
        }
    }
}
