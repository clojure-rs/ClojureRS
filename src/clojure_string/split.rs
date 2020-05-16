use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;
use crate::persistent_vector::PersistentVector;
use crate::type_tag::TypeTag;

/// clojure.string/split [s re & [limit]] splits strings by pattern, optionally maximum of limit
/// amount
#[derive(Debug, Clone)]
pub struct SplitFn {}
impl ToValue for SplitFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for SplitFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 2 && args.len() != 3 {
            return error_message::wrong_varg_count(&[2, 3], args.len());
        } else {
            match (
                args.get(0).unwrap().to_value(),
                args.get(1).unwrap().to_value(),
            ) {
                (Value::String(s), Value::Pattern(re)) => {
                    let splits: Vec<Rc<Value>> = re
                        .split(&s)
                        .collect::<Vec<&str>>()
                        .into_iter()
                        .filter(|ss| !ss.is_empty())
                        .map(|ss| Rc::new(Value::String(ss.to_string())))
                        .collect();
                    return Value::PersistentVector(
                        splits.into_iter().collect::<PersistentVector>(),
                    );
                }
                (_a, Value::Pattern(_)) => error_message::type_mismatch(TypeTag::String, &_a),
                (Value::String(_), _b) => error_message::type_mismatch(TypeTag::Pattern, &_b),
                (_, _) => error_message::unknown_err(String::from("Unknown error")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod split_tests {
        use crate::clojure_string::split::SplitFn;
        use crate::ifn::IFn;
        use crate::persistent_vector::PersistentVector;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn split_by_comma() {
            let split = SplitFn {};
            let s = "hello,world,again";
            let re = ",";
            let args = vec![
                Rc::new(Value::String(String::from(s))),
                Rc::new(Value::Pattern(regex::Regex::new(re).unwrap())),
            ];
            assert_eq!(
                Value::PersistentVector(
                    vec![
                        Rc::new(Value::String(String::from("hello"))),
                        Rc::new(Value::String(String::from("world"))),
                        Rc::new(Value::String(String::from("again")))
                    ]
                    .into_iter()
                    .collect::<PersistentVector>()
                ),
                split.invoke(args)
            );
        }

        #[test]
        fn split_by_comma_not_in_string() {
            let split = SplitFn {};
            let s = "hello world again";
            let re = ",";
            let args = vec![
                Rc::new(Value::String(String::from(s))),
                Rc::new(Value::Pattern(regex::Regex::new(re).unwrap())),
            ];
            assert_eq!(
                Value::PersistentVector(
                    vec![Rc::new(Value::String(String::from("hello world again")))]
                        .into_iter()
                        .collect::<PersistentVector>()
                ),
                split.invoke(args)
            );
        }

        #[test]
        fn split_by_doublequotes() {
            let split = SplitFn {};
            let s = r#"hello world"again""#;
            let re = "\"";
            let args = vec![
                Rc::new(Value::String(String::from(s))),
                Rc::new(Value::Pattern(regex::Regex::new(re).unwrap())),
            ];
            assert_eq!(
                Value::PersistentVector(
                    vec![
                        Rc::new(Value::String(String::from("hello world"))),
                        Rc::new(Value::String(String::from("again")))
                    ]
                    .into_iter()
                    .collect::<PersistentVector>()
                ),
                split.invoke(args)
            );
        }
    }
}
