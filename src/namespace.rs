use crate::symbol::Symbol;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Namespace {
    pub name: Symbol,
    mappings: RefCell<HashMap<Symbol, Rc<Value>>>,
}
impl Namespace {
    pub fn new(name: Symbol, mappings: RefCell<HashMap<Symbol, Rc<Value>>>) -> Namespace {
        Namespace { name, mappings }
    }
    pub fn insert(&self, sym: Symbol, val: Rc<Value>) {
        self.mappings.borrow_mut().insert(sym, val);
    }
    pub fn get(&self, sym: &Symbol) -> Rc<Value> {
        match self.mappings.borrow_mut().get(sym) {
            Some(val) => Rc::clone(val),
            None => Rc::new(Value::Condition(format!("Undefined symbol {}", sym.name))),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Namespaces(pub RefCell<HashMap<Symbol, Namespace>>);
