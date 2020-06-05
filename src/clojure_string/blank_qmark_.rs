use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;
use crate::type_tag::TypeTag;

/// clojure.string/blank? ; returns true if nil, empty or whitespace
#[derive(Debug, Clone)]
pub struct BlankFn {}
impl ToValue for BlankFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for BlankFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        } else {
            match args.get(0).unwrap().to_value() {
                Value::Nil => Value::Boolean(true),
                Value::String(s) => {
                    if s.len() == 0 {
                        Value::Boolean(true)
                    } else {
                        return Value::Boolean(
                            s.chars()
                                .filter(|c| !c.is_whitespace())
                                .collect::<Vec<char>>()
                                .len()
                                == 0,
                        );
                    }
                }
                _a => error_message::type_mismatch(TypeTag::String, &_a.to_value()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod blank_tests {
        use crate::clojure_string::blank_qmark_::BlankFn;
        use crate::ifn::IFn;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn is_non_empty_string_blank() {
            let blank = BlankFn {};
            let s = "hello";
            let args = vec![Rc::new(Value::String(String::from(s)))];
            assert_eq!(Value::Boolean(false), blank.invoke(args));
        }

        #[test]
        fn is_empty_string_blank() {
            let blank = BlankFn {};
            let s = "";
            let args = vec![Rc::new(Value::String(String::from(s)))];
            assert_eq!(Value::Boolean(true), blank.invoke(args));
        }

        #[test]
        fn is_string_with_whitespace_only_blank() {
            let blank = BlankFn {};
            let s = " \t \n   \r ";
            let args = vec![Rc::new(Value::String(String::from(s)))];
            assert_eq!(Value::Boolean(true), blank.invoke(args));
        }

        #[test]
        fn is_string_with_whitespace_and_text_blank() {
            let blank = BlankFn {};
            let s = " \thello \n   \r ";
            let args = vec![Rc::new(Value::String(String::from(s)))];
            assert_eq!(Value::Boolean(false), blank.invoke(args));
        }

        #[test]
        fn is_nil_blank() {
            let blank = BlankFn {};
            let args = vec![Rc::new(Value::Nil)];
            assert_eq!(Value::Boolean(true), blank.invoke(args));
        }
    }
}
