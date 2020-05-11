use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;
use crate::type_tag::TypeTag;

/// clojure.string/upper-case ; converts characters to upper case
#[derive(Debug, Clone)]
pub struct UpperCaseFn {}
impl ToValue for UpperCaseFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for UpperCaseFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        } else {
            match args.get(0).unwrap().to_value() {
                Value::String(s) => Value::String(s.to_uppercase()),
                _a => error_message::type_mismatch(TypeTag::String, &_a.to_value()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod upper_case_tests {
        use crate::clojure_string::upper_case::UpperCaseFn;
        use crate::ifn::IFn;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn upper_case_string() {
            let upper_case = UpperCaseFn {};
            let s = "1.2.3 hello";
            let args = vec![Rc::new(Value::String(String::from(s)))];
            assert_eq!(
                Value::String(String::from("1.2.3 HELLO")),
                upper_case.invoke(args)
            );
        }
    }
}
