use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;
use crate::type_tag::TypeTag;

/// clojure.string/lower-case ; converts characters to lower case
#[derive(Debug, Clone)]
pub struct LowerCaseFn {}
impl ToValue for LowerCaseFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for LowerCaseFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        } else {
            match args.get(0).unwrap().to_value() {
                Value::String(s) => Value::String(s.to_lowercase()),
                _a => error_message::type_mismatch(TypeTag::String, &_a.to_value()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod lower_case_tests {
        use crate::clojure_string::lower_case::LowerCaseFn;
        use crate::ifn::IFn;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn lower_case_string() {
            let lower_case = LowerCaseFn {};
            let s = "1.2.3 HELLO";
            let args = vec![Rc::new(Value::String(String::from(s)))];
            assert_eq!(
                Value::String(String::from("1.2.3 hello")),
                lower_case.invoke(args)
            );
        }
    }
}
