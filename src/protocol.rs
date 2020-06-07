use crate::value::Value;
use std::rc::Rc;

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
    fn instanceof(val: &Rc<Value>) -> bool;

    // The following exist so we can build the rest of the trait functions on this generic behavior
    /// Simply wraps the value in this protocol
    /// whether its a valid cast or not
    fn raw_wrap(val: &Rc<Value>) -> Self;

    /// Simply retrieves the Rc<Value> stored in the Protocol,
    /// whether its valid or not
    fn raw_unwrap(&self) -> Rc<Value>;

    // @TODO consider renaming downcast 
    fn try_as_protocol(val: &Rc<Value>) -> Option<Self> {
        if Self::instanceof(val) {
            Some(Self::raw_wrap(val))
        }
        else {
            None 
        }
    }

    /// Panics if your Value isn't an instance of Protocol
    fn as_protocol(val: &Rc<Value>) -> Self {
        Self::try_as_protocol(val).unwrap()
    }

    fn try_unwrap(&self) -> Option<Rc<Value>>{
        let inner_value = self.raw_unwrap();
        if Self::instanceof(&inner_value) {
            Some(inner_value)
        }
        else {
            None 
        }
    }
    // Realistically, the fact that you unwrap to get an upcast is just an implementation detail, so
    // @TODO change to upcast 
    /// Panics if Value not instance of Protocol
    fn unwrap(&self) -> Rc<Value> {
        self.try_unwrap().unwrap()
    }
}
pub trait ProtocolCastable {
    fn instanceof<T: Protocol>(&self) -> bool;
    fn try_as_protocol<T: Protocol>(&self) -> Option<T>;
    fn as_protocol<T: Protocol>(&self) -> T;
}

impl ProtocolCastable for Rc<Value> {
    fn instanceof<T: Protocol>(&self) -> bool {
        T::instanceof(self)
    }
    fn try_as_protocol<T: Protocol>(&self) -> Option<T> {
        T::try_as_protocol(self)
    }
    fn as_protocol<T: Protocol>(&self) -> T {
        T::as_protocol(self)
    }
}

// @TODO Consider changing syntax to differentiate protocol from variants
// @TODO Consider trade offs of having a very plain function like macro of macro!(a,b,c)
// and having a clearer one like define_protocol(Iterable = A | B | C) 
#[macro_export]
macro_rules! define_protocol {
    // define_protocol!(Protocol = A | B)
    ($protocol:ident = $($variant:ident) |*) => {
        #[derive(Debug, Clone)]
        pub struct $protocol {
            value: Rc<Value>
        }
        impl crate::protocol::Protocol for $protocol {
            fn raw_wrap(val: &Rc<Value>) -> Self {
                $protocol { value: Rc::clone(val) }
            }
            fn raw_unwrap(&self) -> Rc<Value> {
                Rc::clone(&self.value)
            }
            fn instanceof(val: &Rc<Value>) -> bool {
                match &**val {
                    $(
                        crate::value::Value::$variant(_) => true,
                    )*
                    _ => false
                }
            }
        }
    };
    ($protocol:ident, $($variant:ident), *) => {
        #[derive(Debug, Clone)]
        pub struct $protocol {
            value: Rc<Value>
        }
        impl crate::protocol::Protocol for $protocol {
            fn raw_wrap(val: &Rc<Value>) -> Self {
                $protocol { value: Rc::clone(val) }
            }
            fn raw_unwrap(&self) -> Rc<Value> {
                Rc::clone(&self.value)
            }
            fn instanceof(val: &Rc<Value>) -> bool {
                match &**val {
                    $(
                        crate::value::Value::$variant(_) => true,
                    )*
                    _ => false
                }
            }
        }
    };
}
// @TODO next trick;  extend_protocol, so that it actually wraps trait equivalent of protocol
//   Ie, make it so that protocol::IFn is itself of trait IFn, and automatically
//   wraps its trait functions
//   @TODO Think, however, whether its worth it to always have a trait and protocol
//   I believe so -- I think basically the idea is the trait wraps true Rust values,
//   like symbol::Symbol, and the protocol wraps its ClojureRS equivalent, like Value::Symbol
//
//
//   traits and traits are both to give us interface behavior, traits give that to us
//   for our rust primitives like symbol::Symbol, but too much of the power of both is at compile time.
//   Just as we made the Value to give us a value whose rust type could be unknown at runtime,
//   we created the Protocol to give us an interface we could cast Values to and from at runtime,
//   as well as some other benefits we need
//
// macro_rules! extend_protocol {
//     ($protocol:ident, $fn_name:ident, $fn:expr, $($variant:ident), *) => {
//
//         impl $protocol {
//             fn $fn_name(val: &Rc<Value>) -> Option<Self> {
//                 match &**val {
//                     $(
//                         crate::value::Value::$variant(_) => Some($protocol {
//                             value: Rc::clone(val),
//                         }),
//                     )*
//                     _ => None
//                 }
//             }
//             fn try_unwrap(&self) -> Option<Rc<Value>> {
//                 match &*self.value {
//                     $(
//                         crate::value::Value::$variant(_) => Some(Rc::clone(&self.value)),
//                     )*
//                     _ => None
//                 }
//             }
//         }
//     };
// }
//
