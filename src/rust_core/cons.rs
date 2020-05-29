use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;
use crate::iterable::Iterable;
use crate::persistent_list::ToPersistentList;
use crate::protocol::ProtocolCastable;

/// (cons x seq)
/// inserts x as first element of seq
#[derive(Debug, Clone)]
pub struct ConsFn {}
impl ToValue for ConsFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for ConsFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 2 {
            return error_message::wrong_arg_count(2, args.len());
        }

        let mut coll_vec = match args.get(1).unwrap().try_as_protocol::<Iterable>() {
            Some(iterable) => iterable.clone().iter().collect(),
            _ => vec![],
        };

        coll_vec.insert(0, args.get(0).unwrap().to_owned());

        return Value::PersistentList(coll_vec.into_list());
    }
}
