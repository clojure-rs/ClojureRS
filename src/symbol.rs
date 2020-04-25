use std::fmt;
use std::hash::Hash;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Symbol {
    pub name: String,
}
impl Symbol {
    pub fn intern(name: &str) -> Symbol {
        Symbol {
            name: String::from(name),
        }
    }
}
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
