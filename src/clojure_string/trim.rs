use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;
use crate::type_tag::TypeTag;

/// clojure.string/trim trims white space from start and end of string
#[derive(Debug, Clone)]
pub struct TrimFn {}
impl ToValue for TrimFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for TrimFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        } else {
            match args.get(0).unwrap().to_value() {
                Value::String(s) => Value::String(s.trim().to_string()),
                _a => error_message::type_mismatch(TypeTag::String, &_a.to_value()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod trim_tests {
        use crate::clojure_string::trim::TrimFn;
        use crate::ifn::IFn;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn trim() {
            let trim = TrimFn {};
            let s = " \r \t  hello   \n";
            let args = vec![Rc::new(Value::String(String::from(s)))];
            assert_eq!(Value::String(String::from("hello")), trim.invoke(args));
        }
    }
}
