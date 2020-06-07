use std::convert::From;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::FromIterator;
use std::rc::Rc;

use crate::value::{ToValue, Value};
use crate::persistent_list_map::PersistentListMap;
use crate::traits;
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct PersistentVector {
    pub vals: Vec<Rc<Value>>,
}
impl traits::IMeta for PersistentVector {
    fn meta(&self) -> PersistentListMap {
        // @TODO implement
        PersistentListMap::Empty
    }
}
impl traits::IObj for PersistentVector {
    fn with_meta(&self,meta: PersistentListMap) -> PersistentVector {
        // @TODO implement
        self.clone()
    }
}
impl fmt::Display for PersistentVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = self
            .vals
            .iter()
            .map(|rc_arg| rc_arg.to_string_explicit())
            .collect::<Vec<std::string::String>>()
            .join(" ");
        write!(f, "[{}]", str)
    }
}

impl From<Vec<Rc<Value>>> for PersistentVector {
    fn from(item: Vec<Rc<Value>>) -> Self {
        item.into_iter().collect::<PersistentVector>()
    }
}
// impl Hash for PersistentVector {
//     fn hash<H: Hasher>(&self, state: &mut H) {
// 	let as_vec = Rc::new(self.clone()).iter().collect::<Vec<Rc<Value>>>();
// 	as_vec.hash(state)
//     }
// }
//
// Mostly to just make some code more concise
// @TODO ~lookup proper rust conversion traits~
// @TODO ok, proper conversions found, start removing these
pub trait ToPersistentVector {
    // Uses 'into' instead of typical 'to_..' because this is actually meant to be
    // (into [] self), a sort of building block of our eventual `into` function
    fn into_vector(self) -> PersistentVector;
    fn into_vector_value(self: Self) -> Value
    where
        Self: Sized,
    {
        self.into_vector().to_value()
    }
}
impl ToPersistentVector for Vec<Rc<Value>> {
    fn into_vector(self) -> PersistentVector {
        self.into_iter().collect::<PersistentVector>()
    }
}

pub trait ToPersistentVectorIter {
    fn iter(&self) -> PersistentVectorIter;
}
impl ToPersistentVectorIter for Rc<PersistentVector> {
    fn iter(&self) -> PersistentVectorIter {
        PersistentVectorIter {
            vector: Rc::clone(self),
            ind: 0,
        }
        // self.vals.iter().map(|rc_ref| Rc::clone(rc_ref))
    }
}
pub struct PersistentVectorIter {
    vector: Rc<PersistentVector>,
    ind: usize,
}
impl Iterator for PersistentVectorIter {
    type Item = Rc<Value>;
    fn next(&mut self) -> Option<Self::Item> {
        let retval = (&*self.vector.vals)
            .get(self.ind)
            .map(|rc_val| Rc::clone(rc_val));
        self.ind += 1;
        retval
    }
}
impl FromIterator<Rc<Value>> for PersistentVector {
    //
    // @TODO Surely we can just forward Vec's from_iter and wrap it
    //
    fn from_iter<I: IntoIterator<Item = Rc<Value>>>(iter: I) -> Self {
        // @TODO see if we can directly loop through our original iter backwards, and avoid
        // dumping into this vector just to loop through again backwards
        let mut coll_as_vec = vec![];

        for i in iter {
            coll_as_vec.push(i);
        }
        PersistentVector { vals: coll_as_vec }
    }
}
