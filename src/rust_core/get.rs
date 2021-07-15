use crate::ifn::IFn;
use crate::persistent_list_map::IPersistentMap;
use crate::value::{ToValue, Value};
use std::rc::Rc;

// General get fn; however,  currently just implemented
// for our one map type, PersistentListMap
#[derive(Debug, Clone)]
pub struct GetFn {}
impl ToValue for GetFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for GetFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 2 {
            return Value::Condition(format!(
                "Wrong number of arguments given to function (Given: {}, Expected: 2)",
                args.len()
            ));
        }

        if let Value::PersistentListMap(pmap) = &*(args.get(0).unwrap().clone()) {
            let key = args.get(1).unwrap();
            return pmap.get(key).to_value();
        }
        // @TODO add error in here with erkk's new error tools

        Value::Nil
    }
}
