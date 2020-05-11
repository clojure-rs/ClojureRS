use crate::value::Value;
use std::rc::Rc;

// @TODO should we just exclusively call them protocols as
//       we've been doing (deciding that Clojure without the host
//       has Protocols as its natural abstraction for interfaces)
//       or should interfaces, as an abstraction, exist (and perhaps
//       be used by Protocols even?)
/// A type that implements Protocol is one that has a pseudo downcasting of
///
/// value.as::<ISeq>()
///
/// Where ISeq would implement Protocol
///
/// Then you can call functions on this ISeq, if you got one back, to forward
/// functions to the Value inside, and unwrap() it to get your Value back when
/// you're done
pub trait Protocol: Sized {
    fn try_as_protocol(val: &Rc<Value>) -> Option<Self>; // where Self:Sized;
    /// Panics if your Value isn't an instance of Protocol
    fn as_protocol(val: &Rc<Value>) -> Self {
        Self::try_as_protocol(val).unwrap()
    }
    fn try_unwrap(&self) -> Option<Rc<Value>>;
    /// Panics if your Value isn't an instance of Protocol
    fn unwrap(&self) -> Rc<Value> {
        self.try_unwrap().unwrap()
    }
}
pub trait ProtocolCastable {
    fn try_as_protocol<T: Protocol>(&self) -> Option<T>;
    fn as_protocol<T: Protocol>(&self) -> T;
}
impl ProtocolCastable for Rc<Value> {
    fn try_as_protocol<T: Protocol>(&self) -> Option<T> {
        T::try_as_protocol(self)
    }
    fn as_protocol<T: Protocol>(&self) -> T {
        T::as_protocol(self)
    }
}
