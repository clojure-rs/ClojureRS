use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;
use crate::type_tag::TypeTag;

/// clojure.string/trim_newline trims white space from start of string
#[derive(Debug, Clone)]
pub struct TrimNewlineFn {}
impl ToValue for TrimNewlineFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for TrimNewlineFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        } else {
            match args.get(0).unwrap().to_value() {
                Value::String(s) => {
                    Value::String(s.trim_end_matches(|c| c == '\n' || c == '\r').to_string())
                }
                _a => error_message::type_mismatch(TypeTag::String, &_a.to_value()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod trim_newline_tests {
        use crate::clojure_string::trim_newline::TrimNewlineFn;
        use crate::ifn::IFn;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn trim_newline() {
            let trim_newline = TrimNewlineFn {};
            let s = " \r \t  hello   \n\r";
            let args = vec![Rc::new(Value::String(String::from(s)))];
            assert_eq!(
                Value::String(String::from(" \r \t  hello   ")),
                trim_newline.invoke(args)
            );
        }
        #[test]
        fn trim_newline_does_nothing() {
            let trim_newline = TrimNewlineFn {};
            let s = " \r \t  hello   . ";
            let args = vec![Rc::new(Value::String(String::from(s)))];
            assert_eq!(
                Value::String(String::from(" \r \t  hello   . ")),
                trim_newline.invoke(args)
            );
        }
    }
}
