//! General map utilities
use crate::value::Value;
use std::rc::Rc;

#[derive(Debug,Clone,PartialEq,Hash)]
pub struct MapEntry {
    // We need to box key to avoid an infinite cycle of
    // Value::Persistent*Map { Persistent*Map { MapEntry { Value <--- cycle restarts , val }}}
    // Implemented with an Rc because inevitably,  our system tends to live in Rcs 
    pub key: Rc<Value>,
    pub val: Rc<Value>
}
