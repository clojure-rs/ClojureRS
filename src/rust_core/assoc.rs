use crate::ifn::IFn;
use crate::value::{Value, ToValue};
use std::rc::Rc;
use crate::util::IsEven;
use itertools::Itertools;
use crate::persistent_list_map::IPersistentMap;

/// (assoc map key val & kvs)
///
// General assoc fn; however,  currently just implemented
// for our one map type, PersistentListMap
#[derive(Debug, Clone)]
pub struct AssocFn {}
impl ToValue for AssocFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for AssocFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        // We don't want even args, because assoc works like
        // (assoc {} :a 1) ;; 3 args
        // (assoc {} :a 1 :b 2) ;; 5 args
        // (assoc {} :a 1 :b 2 :c 3) ;; 7 args ...
        if args.len() < 3 || args.len().is_even() {
            return Value::Condition(format!(
                "Wrong number of arguments given to function (Given: {}, Expected: 3 | 5 | 7 | ..)",
                args.len()
            ));
        }

        if let Value::PersistentListMap(pmap) = &*(args.get(0).unwrap().clone()) {
            let mut retval = pmap.clone();
            for (key_value, val_value) in args.into_iter().skip(1).tuples() {
                let key = key_value.to_rc_value();
                let val = val_value.to_rc_value();
                println!("key: {:?}, val: {:?}", key, val);
                retval = pmap.assoc(key, val);
            }
            return Value::PersistentListMap(retval);
        }

        Value::Nil
    }
}
