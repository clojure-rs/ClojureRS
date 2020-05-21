use crate::error_message;
use crate::ifn::IFn;
use crate::iterable::Iterable;
use crate::protocol::ProtocolCastable;
use crate::type_tag::TypeTag;
use crate::value::{ToValue, Value};
use std::rc::Rc;

/// (second x)
/// returns second element or nil
/// TODO: support for strings
#[derive(Debug, Clone)]
pub struct SecondFn {}
impl ToValue for SecondFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for SecondFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        }
        match args.get(0).unwrap().try_as_protocol::<Iterable>() {
            Some(iterable) => match iterable.iter().nth(1) {
                Some(val) => val.to_value(),
                _ => Value::Nil,
            },
            _ => error_message::type_mismatch(TypeTag::ISeq, args.get(0).unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    mod second_tests {
        use crate::ifn::IFn;
        use crate::persistent_list::PersistentList;
        use crate::rust_core::second::SecondFn;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn second_on_empty_iterable() {
            let second = SecondFn {};
            let args = vec![Rc::new(Value::PersistentList(
                vec![].into_iter().collect::<PersistentList>(),
            ))];
            assert_eq!(Value::Nil, second.invoke(args));
        }

        #[test]
        fn second_on_iterable_with_two_value_list() {
            let second = SecondFn {};
            let args = vec![Rc::new(Value::PersistentList(
                vec![Rc::new(Value::I32(1)), Rc::new(Value::I32(2))]
                    .into_iter()
                    .collect::<PersistentList>(),
            ))];
            assert_eq!(Value::I32(2), second.invoke(args));
        }

        #[test]
        #[should_panic]
        fn second_on_non_iterable_value() {
            let second = SecondFn {};
            let args = vec![Rc::new(Value::Nil)];
            assert_eq!(Value::Nil, second.invoke(args));
        }
    }
}
