use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;
use crate::type_tag::TypeTag;

/// clojure.string/reverse ; reverses a string
/// (defn print-string [string] .. prints single string .. )
#[derive(Debug, Clone)]
pub struct JoinFn {}
impl ToValue for JoinFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for JoinFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() == 1 {
            match args.get(0).unwrap().to_value() {
                Value::Iseq(s) => Value::String(s.chars().rev().collect()),
                _a => error_message::type_mismatch(TypeTag::String, &_a.to_value())
            }

        } else {
            return error_message::wrong_arg_count(2, args.len());

        }
    }
}

#[cfg(test)]
mod tests {
    mod reverse_tests {
        use crate::value::Value;
        use std::rc::Rc;
        use crate::clojure_string::reverse::JoinFn;
        use crate::ifn::IFn;

        #[test]
        fn reverse_string() {
            let reverse = JoinFn {};
            let s = "hello";
            let args = vec![Rc::new(Value::String(String::from(s)))];
            assert_eq!(Value::String(String::from("olleh")), reverse.invoke(args));
        }
    }
}