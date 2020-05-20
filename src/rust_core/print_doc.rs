use crate::environment::Environment;
use crate::error_message;
use crate::ifn::IFn;
use crate::keyword::Keyword;
use crate::persistent_list_map::IPersistentMap;
use crate::type_tag::TypeTag;
use crate::value::{ToValue, Value};
use std::rc::Rc;

/// Returns doc metadata for symbol
#[derive(Debug, Clone)]
pub struct PrintDocFn {
    enclosing_environment: Rc<Environment>,
}
impl PrintDocFn {
    pub fn new(enclosing_environment: Rc<Environment>) -> PrintDocFn {
        PrintDocFn {
            enclosing_environment,
        }
    }
}
impl ToValue for PrintDocFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for PrintDocFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        }

        match args.get(0).unwrap().to_value() {
            Value::Symbol(s) => match self.enclosing_environment.get(&s).to_value() {
                Value::Condition(error) => error_message::unknown_err(error),
                _ => {
                    println!(
                        "-------------------------\n{}/{}\n([TODO argslist])\n{}",
                        s.ns,
                        s.name,
                        s.meta.get(&Keyword::intern("doc").to_rc_value())
                    );
                    return Value::Nil;
                }
            },
            _ => {
                return error_message::type_mismatch(TypeTag::Symbol, args.get(0).unwrap());
            }
        }
    }
}
