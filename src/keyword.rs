use crate::symbol::Symbol;
use std::fmt;
use std::hash::Hash;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Keyword {
    // In Clojure proper,  a Keyword wraps a Symbol to share their ..symbolic functionality
    pub sym: Symbol,
}
impl Keyword {
    pub fn intern(name: &str) -> Keyword {
        Keyword {
            sym: Symbol {
                name: String::from(name),
            },
        }
    }
}
impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ":{}", self.sym.name)
    }
}
