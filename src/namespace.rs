use std::collections::HashMap;
use symbol::Symbol;
use value::Value;

pub struct Namespace {
    name: Symbol,
    mappings: HashMap<Symbol,Value>
}

struct Namespaces(HashMap<Symbol,Namespace>);
