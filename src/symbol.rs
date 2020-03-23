use std::hash::{Hash,Hasher};

#[derive(Hash,PartialEq,Eq,Clone,Debug)]
pub struct Symbol {
    pub name: String
}
