use crate::error_message;
use crate::ifn::IFn;
use crate::iterable::Iterable;
use crate::protocol::ProtocolCastable;
use crate::type_tag::TypeTag;
use crate::value::{ToValue, Value};
use std::rc::Rc;

/// (first x)
/// returns first element or nil
/// TODO: support for strings
#[derive(Debug, Clone)]
pub struct FirstFn {}
impl ToValue for FirstFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for FirstFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        }
        match args.get(0).unwrap().try_as_protocol::<Iterable>() {
            Some(iterable) => match iterable.iter().next() {
                Some(val) => val.to_value(),
                _ => Value::Nil,
            },
            _ => error_message::type_mismatch(TypeTag::ISeq, args.get(0).unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    mod first_tests {
        use crate::ifn::IFn;
        use crate::persistent_list::PersistentList;
        use crate::rust_core::first::FirstFn;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn first_on_empty_iterable() {
            let first = FirstFn {};
            let args = vec![Rc::new(Value::PersistentList(
                vec![].into_iter().collect::<PersistentList>(),
            ))];
            assert_eq!(Value::Nil, first.invoke(args));
        }

        #[test]
        fn first_on_iterable_with_one_value_list() {
            let first = FirstFn {};
            let args = vec![Rc::new(Value::PersistentList(
                vec![Rc::new(Value::Boolean(true))]
                    .into_iter()
                    .collect::<PersistentList>(),
            ))];
            assert_eq!(Value::Boolean(true), first.invoke(args));
        }

        #[test]
        #[should_panic]
        fn first_on_non_iterable_value() {
            let first = FirstFn {};
            let args = vec![Rc::new(Value::Nil)];
            assert_eq!(Value::Nil, first.invoke(args));
        }
    }
}
