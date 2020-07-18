use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;
use crate::iterable::Iterable;
use crate::persistent_list::ToPersistentList;
use crate::protocol::ProtocolCastable;
use crate::type_tag::TypeTag;

/// (count coll)
/// counts items in coll
#[derive(Debug, Clone)]
pub struct CountFn {}
impl ToValue for CountFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for CountFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        }

        let coll_size = match args.get(0).unwrap().try_as_protocol::<Iterable>() {
            Some(iterable) => iterable.iter().count(),
            None => match args.get(0).unwrap().to_value() {
                Value::Nil => 0,
                _unsupported => return error_message::type_mismatch(TypeTag::ISeq, &_unsupported),
            },
        };

        return Value::I32(coll_size as i32);
    }
}

#[cfg(test)]
mod tests {
    mod count_tests {
        use crate::ifn::IFn;
        use crate::persistent_vector::PersistentVector;
        use crate::rust_core::CountFn;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn count_nil() {
            let count = CountFn {};
            let args = vec![Rc::new(Value::Nil)];
            assert_eq!(Value::I32(0), count.invoke(args));
        }

        #[test]
        fn count_vector() {
            let count = CountFn {};
            let args = vec![Rc::new(Value::PersistentVector(
                vec![
                    Rc::new(Value::String(String::from("1"))),
                    Rc::new(Value::String(String::from("2"))),
                ]
                .into_iter()
                .collect::<PersistentVector>(),
            ))];
            assert_eq!(Value::I32(2), count.invoke(args));
        }

        #[test]
        fn count_something_unsupported() {
            let count = CountFn {};
            let args = vec![Rc::new(Value::Boolean(true))];
            assert_eq!(
                Value::Condition(
                    "Type mismatch; Expected instance of clojure.lang.ISeq,  Recieved type true"
                        .to_string()
                ),
                count.invoke(args)
            );
        }
    }
}
