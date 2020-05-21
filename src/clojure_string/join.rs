use crate::ifn::IFn;
use crate::iterable::Iterable;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;
use crate::protocol::ProtocolCastable;

/// clojure.string/join ; joins a coll of items together as a string
/// (join
///   [coll]
///   [separator coll])
#[derive(Debug, Clone)]
pub struct JoinFn {}
impl ToValue for JoinFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for JoinFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 && args.len() != 2 {
            return error_message::wrong_varg_count(&[1, 2], args.len());
        }

        let separator = if args.len() == 1 {
            String::from("")
        } else {
            args.get(0).unwrap().to_string()
        };

        let coll = if args.len() == 1 {
            args.get(0)
        } else {
            args.get(1)
        };
        if let Some(iterable) = coll.unwrap().try_as_protocol::<Iterable>() {
            Value::String(
                iterable
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<std::string::String>>()
                    .join(&separator),
            )
        } else {
            Value::String(String::from(""))
        }
    }
}

#[cfg(test)]
mod tests {
    mod reverse_tests {
        use crate::clojure_string::join::JoinFn;
        use crate::ifn::IFn;
        use crate::persistent_list::PersistentList;
        use crate::persistent_vector::PersistentVector;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn join_empty_collection_to_empty_string() {
            let join = JoinFn {};
            let args = vec![Rc::new(Value::PersistentList(
                vec![].into_iter().collect::<PersistentList>(),
            ))];
            assert_eq!(Value::String(String::from("")), join.invoke(args));
        }

        #[test]
        fn join_one_item_collection_to_string() {
            let join = JoinFn {};
            let s = "hello";
            let args = vec![Rc::new(Value::PersistentList(
                vec![Rc::new(Value::String(String::from(s)))]
                    .into_iter()
                    .collect::<PersistentList>(),
            ))];
            assert_eq!(Value::String(String::from("hello")), join.invoke(args));
        }

        #[test]
        fn join_multiple_items_in_collection_to_string() {
            let join = JoinFn {};
            let s = "hello";
            let args = vec![Rc::new(Value::PersistentList(
                vec![
                    Rc::new(Value::String(String::from(s))),
                    Rc::new(Value::I32(5)),
                    Rc::new(Value::String(String::from(s))),
                ]
                .into_iter()
                .collect::<PersistentList>(),
            ))];
            assert_eq!(
                Value::String(String::from("hello5hello")),
                join.invoke(args)
            );
        }

        #[test]
        fn join_multiple_items_in_collection_with_separator_to_string() {
            let join = JoinFn {};
            let s = "hello";
            let args = vec![
                Rc::new(Value::String(String::from(", "))),
                Rc::new(Value::PersistentList(
                    vec![
                        Rc::new(Value::String(String::from(s))),
                        Rc::new(Value::I32(5)),
                        Rc::new(Value::String(String::from(s))),
                    ]
                    .into_iter()
                    .collect::<PersistentList>(),
                )),
            ];
            assert_eq!(
                Value::String(String::from("hello, 5, hello")),
                join.invoke(args)
            );
        }

        #[test]
        fn join_multiple_items_in_pvec_collection_with_separator_to_string() {
            let join = JoinFn {};
            let s = "hello";
            let args = vec![
                Rc::new(Value::String(String::from(", "))),
                Rc::new(Value::PersistentVector(
                    vec![
                        Rc::new(Value::String(String::from(s))),
                        Rc::new(Value::I32(5)),
                        Rc::new(Value::String(String::from(s))),
                    ]
                    .into_iter()
                    .collect::<PersistentVector>(),
                )),
            ];
            assert_eq!(
                Value::String(String::from("hello, 5, hello")),
                join.invoke(args)
            );
        }
    }
}
