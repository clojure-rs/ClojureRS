use crate::error_message;
use crate::ifn::IFn;
use crate::iterable::Iterable;
use crate::persistent_list::PersistentList;
use crate::protocol::ProtocolCastable;
use crate::type_tag::TypeTag;
use crate::util::IsEven;
use crate::value::{ToValue, Value};
use itertools::Itertools;
use std::rc::Rc;

// This is a tide me over rust wrapper, as map is implemented in lower level primitives
// in pure Clojure
// // That being said, I have not decided as to whether or not there is value to having both
#[derive(Debug, Clone)]
pub struct MapFn {}
impl ToValue for MapFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for MapFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.is_empty() {
            return error_message::wrong_arg_count(1, args.len());
        }
        if let Value::IFn(ifn) = &**args.get(0).unwrap() {
            if let Some(iterable) = args.get(1).unwrap().try_as_protocol::<Iterable>() {
                return iterable
                    .iter()
                    .map(|rc_val| Rc::new(ifn.invoke(vec![rc_val])))
                    .collect::<PersistentList>()
                    .to_value();
            }
        }
        Value::Condition(format!(
            "Type mismatch; Expected instance of {}, Recieved type {}",
            TypeTag::IFn,
            args.len()
        ))
    }
}
