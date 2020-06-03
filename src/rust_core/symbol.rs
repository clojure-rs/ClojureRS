use crate::environment::Environment;
use crate::error_message;
use crate::ifn::IFn;
use crate::symbol::Symbol;
use crate::type_tag::TypeTag;
use crate::value::{ToValue, Value};
use std::rc::Rc;

/// Returns a symbol (symbol ns name), for interop purposes
#[derive(Debug, Clone)]
pub struct SymbolFn {
    enclosing_environment: Rc<Environment>,
}
impl SymbolFn {
    pub fn new(enclosing_environment: Rc<Environment>) -> SymbolFn {
        SymbolFn {
            enclosing_environment,
        }
    }
}
impl ToValue for SymbolFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for SymbolFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 2 {
            return error_message::wrong_arg_count(2, args.len());
        }

        match args.get(0).unwrap().to_value() {
            Value::String(ns) => match args.get(1).unwrap().to_value() {
                Value::String(name) => {
                    let symbol = Symbol::intern_with_ns(&ns, &name);
                    println!("symbol got: {}", &symbol.name);
                    match self
                        .enclosing_environment
                        .get_symbol(&symbol) //&Symbol::unqualified(&symbol))
                        .to_value()
                    {
                        Value::Condition(error) => {
                            println!("cond {}", &error);
                            error_message::unknown_err(error)
                        }
                        Value::Symbol(found) => {
                            println!("found symbol: {}/{}", &found.ns, &found.name);
                            Value::Symbol(symbol)
                        }
                        e_ => {
                            println!(" other{}", &e_.to_string());
                            error_message::unknown_err(e_.to_string())
                        }
                    }
                }
                _ => error_message::type_mismatch(TypeTag::Symbol, args.get(1).unwrap()),
            },
            _ => error_message::type_mismatch(TypeTag::Symbol, args.get(0).unwrap()),
        }
    }
}
