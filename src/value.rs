use crate::symbol::Symbol;
use crate::type_tag::TypeTag;
use std::collections::HashMap;
use std::rc::Rc;
use std::fmt::Debug;
pub trait IFn : Debug {
    fn invoke(&self,args: &[&Value]) -> Value;
}
#[derive(Debug)]
pub enum Value {
    I32(i32),
    Symbol(Symbol),
    IFn(Rc<dyn IFn>),
    Condition(String)
}
impl Value {
    fn type_tag(&self) -> TypeTag
    {
	match self {
	    Value::I32(_) => TypeTag::I32,
	    Value::Symbol(_) => TypeTag::Symbol,
	    Value::IFn(_) => TypeTag::IFn,
	    Value::Condition(_) => TypeTag::Condition
	}
    }
    fn eval(self : Rc<Value>, environment: &HashMap<Symbol,Rc<Value>>) -> Rc<Value> {
	match &*self {
	    Value::Symbol(symbol) => match environment.get(symbol) {
		Some(val) => Rc::clone(val),
		_ => Rc::new(Value::Condition(format!("Undefined symbol {}",symbol.name)))
	    }
	    // I32, Fn 
	    _ => Rc::clone(&self),
	}
    }
}

pub trait ToValue {
    fn to_value(&self) -> Value; 
}

impl ToValue for i32 {
    fn to_value(&self) -> Value {
	Value::I32(self.clone()) 
    }
}



