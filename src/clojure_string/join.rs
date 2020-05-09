use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;
use crate::type_tag::TypeTag;
use crate::persistent_list::ToPersistentListIter;
use crate::persistent_vector::ToPersistentVectorIter;
use itertools::Itertools;
use crate::persistent_list::PersistentList::Cons;

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
        if args.len() == 1 || args.len() == 2 {
            let separator = if args.len() == 2 {
                args.get(0).unwrap().to_string()
            } else { String::from("") };
            let coll = if args.len() == 1 { args.get(0) } else { args.get(1) };
            match coll.unwrap().to_value() {
                Value::PersistentList(Cons(head, tail, count)) => {
                    return if count == 0 {
                        Value::String(String::from(""))
                    } else if count == 1 {
                        Value::String(head.to_string())
                    } else {
                        Value::String(
                            String::from(
                                head.to_string()) + separator.as_str() +
                            tail.iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<std::string::String>>()
                                .join(&separator).as_str())
                    }
                },
                Value::PersistentVector(pvec) => {
                    return if pvec.vals.len() == 0 {
                        Value::String(String::from(""))
                    } else {
                        Value::String(String::from(
                            pvec.vals.iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<std::string::String>>()
                                .join(&separator).as_str()))
                    }
                }
                _ => Value::String(String::from(""))
            }
        } else {
            return error_message::wrong_varg_count(&[1,2], args.len());
        }
    }
}

#[cfg(test)]
mod tests {
    mod reverse_tests {
        use crate::value::Value;
        use std::rc::Rc;
        use crate::clojure_string::join::JoinFn;
        use crate::ifn::IFn;
        use crate::persistent_list::PersistentList;
        use crate::persistent_vector::PersistentVector;

        #[test]
        fn join_empty_collection_to_empty_string() {
            let join = JoinFn {};
            let args = vec![Rc::new(Value::PersistentList(vec![].into_iter().collect::<PersistentList>()))];
            assert_eq!(Value::String(String::from("")), join.invoke(args));
        }

        #[test]
        fn join_one_item_collection_to_string() {
            let join = JoinFn {};
            let s = "hello";
            let args = vec![Rc::new(Value::PersistentList(
                vec![Rc::new(Value::String(String::from(s)))].into_iter().collect::<PersistentList>()))];
            assert_eq!(Value::String(String::from("hello")), join.invoke(args));
        }

        #[test]
        fn join_multiple_items_in_collection_to_string() {
            let join = JoinFn {};
            let s = "hello";
            let args = vec![Rc::new(Value::PersistentList(
                vec![Rc::new(Value::String(String::from(s))),
                     Rc::new(Value::I32(5)),
                     Rc::new(Value::String(String::from(s)))]
                    .into_iter().collect::<PersistentList>()))];
            assert_eq!(Value::String(String::from("hello5hello")), join.invoke(args));
        }

        #[test]
        fn join_multiple_items_in_collection_with_separator_to_string() {
            let join = JoinFn {};
            let s = "hello";
            let args = vec![Rc::new(Value::String(String::from(", "))),
                            Rc::new(Value::PersistentList(
                                vec![Rc::new(Value::String(String::from(s))),
                                     Rc::new(Value::I32(5)),
                                     Rc::new(Value::String(String::from(s)))]
                                    .into_iter().collect::<PersistentList>()))];
            assert_eq!(Value::String(String::from("hello, 5, hello")), join.invoke(args));
        }

        #[test]
        fn join_multiple_items_in_pvec_collection_with_separator_to_string() {
            let join = JoinFn {};
            let s = "hello";
            let args = vec![Rc::new(Value::String(String::from(", "))),
                            Rc::new(Value::PersistentVector(
                                vec![Rc::new(Value::String(String::from(s))),
                                     Rc::new(Value::I32(5)),
                                     Rc::new(Value::String(String::from(s)))]
                                    .into_iter().collect::<PersistentVector>()))];
            assert_eq!(Value::String(String::from("hello, 5, hello")), join.invoke(args));
        }
    }
}