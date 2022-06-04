use crate::environment::Environment;
use crate::error_message;
use crate::ifn::IFn;
use crate::protocol::ProtocolCastable;
use crate::protocols;
use crate::traits::IMeta;
use crate::type_tag::TypeTag;
use crate::value::{ToValue, Value};
use std::rc::Rc;

/// Returns meta for symbol
/// Todo: currently uses form (meta 'clojure.string/join)
/// should use #'var-form
/// TODO: macro: true/false
/// TODO: argslists for functions
#[derive(Debug, Clone)]
pub struct MetaFn {
    enclosing_environment: Rc<Environment>,
}
impl MetaFn {
    pub fn new(enclosing_environment: Rc<Environment>) -> MetaFn {
        MetaFn {
            enclosing_environment,
        }
    }
}
impl ToValue for MetaFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for MetaFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        }
        match args.get(0).unwrap().try_as_protocol::<protocols::IMeta>() {
            Some(imeta) => imeta.meta().to_value(),
            // In order to avoid having the cryptic error messages of Clojure, we're experimenting here
            // already with some other error messages. As we finds ones we like, they will likewise be
            // abstracted out to their own functions -- for now, they're just one offs
            _ => error_message::custom(&format!(
                "In (meta ..), .. must be an instance of IMeta, and {} is of type {}, which is not",
                args.get(0).unwrap(),
                args.get(0).unwrap().type_tag()
            )), //error_message::cast_error(Cast("IMeta"), TypeTag::PersistentListMap)
        }
    }
}
