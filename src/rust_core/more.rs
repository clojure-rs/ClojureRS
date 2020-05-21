use crate::error_message;
use crate::ifn::IFn;
use crate::iterable::Iterable;
use crate::persistent_list::PersistentList;
use crate::protocol::ProtocolCastable;
use crate::type_tag::TypeTag;
use crate::value::{ToValue, Value};
use std::rc::Rc;

/// (more x)
/// for rest, returns possibly empty sequence after the first
/// TODO: support for strings
#[derive(Debug, Clone)]
pub struct MoreFn {}
impl ToValue for MoreFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for MoreFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        }
        match args.get(0).unwrap().try_as_protocol::<Iterable>() {
            Some(iterable) => match iterable.iter().collect::<Vec<Rc<Value>>>().split_first() {
                Some((_, more)) => {
                    Value::PersistentList(more.to_vec().into_iter().collect::<PersistentList>())
                }
                _ => Value::PersistentList(vec![].into_iter().collect::<PersistentList>()),
            },
            _ => error_message::type_mismatch(TypeTag::ISeq, args.get(0).unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    mod more_tests {
        use crate::ifn::IFn;
        use crate::persistent_list::PersistentList;
        use crate::rust_core::more::MoreFn;
        use crate::value::Value;
        use std::rc::Rc;

        #[test]
        fn more_on_empty_iterable() {
            let more = MoreFn {};
            let args = vec![Rc::new(Value::PersistentList(
                vec![].into_iter().collect::<PersistentList>(),
            ))];
            assert_eq!(
                Value::PersistentList(vec![].into_iter().collect::<PersistentList>()),
                more.invoke(args)
            );
        }

        #[test]
        fn more_on_iterable_with_one_value_list() {
            let more = MoreFn {};
            let args = vec![Rc::new(Value::PersistentList(
                vec![Rc::new(Value::Boolean(true))]
                    .into_iter()
                    .collect::<PersistentList>(),
            ))];
            assert_eq!(
                Value::PersistentList(vec![].into_iter().collect::<PersistentList>()),
                more.invoke(args)
            );
        }

        #[test]
        fn more_on_iterable_with_three_value_list() {
            let more = MoreFn {};
            let args = vec![Rc::new(Value::PersistentList(
                vec![
                    Rc::new(Value::I32(1)),
                    Rc::new(Value::I32(2)),
                    Rc::new(Value::I32(3)),
                ]
                .into_iter()
                .collect::<PersistentList>(),
            ))];
            assert_eq!(
                Value::PersistentList(
                    vec![Rc::new(Value::I32(2)), Rc::new(Value::I32(3))]
                        .into_iter()
                        .collect::<PersistentList>()
                ),
                more.invoke(args)
            );
        }

        #[test]
        #[should_panic]
        fn more_on_non_iterable_value() {
            let more = MoreFn {};
            let args = vec![Rc::new(Value::Nil)];
            assert_eq!(Value::Nil, more.invoke(args));
        }
    }
}
