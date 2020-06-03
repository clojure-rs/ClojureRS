use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;
use crate::iterable::Iterable;
use crate::persistent_list::ToPersistentList;
use crate::protocol::ProtocolCastable;

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
            _ => 0,
        };

        return Value::I32(coll_size as i32);
    }
}
