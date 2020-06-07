use std::fmt;
use std::fmt::Debug;
use std::iter::FromIterator;
use std::rc::Rc;

use crate::value::{ToValue, Value};
use crate::persistent_list_map::PersistentListMap;
use crate::traits;
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum PersistentList {
    // @TODO refactor i32 (our len) into a usize
    Cons(Rc<Value>, Rc<PersistentList>, i32),
    Empty,
}
// @TODO definitely don't do this
#[derive(Debug, Clone, Hash)]
struct EmptyHash {}

use crate::persistent_list::PersistentList::{Cons, Empty};
pub fn cons_rc(head: Rc<Value>, tail: Rc<PersistentList>) -> PersistentList {
    if let Cons(_, _, old_count) = &*tail {
        Cons(Rc::clone(&head), Rc::clone(&tail), old_count + 1)
    } else {
        Cons(Rc::clone(&head), Rc::clone(&tail), 1)
    }
}

/// For building a 'top level' list, that is the first to reference (and own) all of its values
pub fn cons(head: Value, tail: PersistentList) -> PersistentList {
    cons_rc(Rc::new(head), Rc::new(tail))
}
impl PersistentList {
    pub fn len(&self) -> i32 {
        match self {
            Cons(_, _, count) => *count,
            _ => 0,
        }
    }
}

//
// Mostly to just make some code more concise
// @TODO convert to proper Rust idiom for converting between types
// @TODO forget 'into';  used that name because of Clojure's "(into ..)" but
//       its better this just be a plain conversion function
//
/// Convert to a PersistentList
pub trait ToPersistentList {
    fn into_list(self) -> PersistentList;
    fn into_list_value(self: Self) -> Value
    where
        Self: Sized,
    {
        self.into_list().to_value()
    }
}
impl ToPersistentList for Vec<&Value> {
    fn into_list(self) -> PersistentList {
        self.into_iter()
            .map(|val| val.to_rc_value())
            .collect::<PersistentList>()
    }
}
impl ToPersistentList for Vec<Rc<Value>> {
    fn into_list(self) -> PersistentList {
        self.into_iter().collect::<PersistentList>()
    }
}
impl traits::IMeta for PersistentList {
    fn meta(&self) -> PersistentListMap {
        // @TODO implement
        PersistentListMap::Empty
    }
}
impl traits::IObj for PersistentList {
    fn with_meta(&self,meta: PersistentListMap) -> PersistentList {
        // @TODO implement
        self.clone()
    }
}
impl fmt::Display for PersistentList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Cons(head, tail, count) => {
                let tail_str = tail
                    .iter()
                    .map(|rc_arg| rc_arg.to_string_explicit())
                    .collect::<Vec<std::string::String>>()
                    .join(" ");
                if *count == 1 {
                    format!("({})", head.to_string_explicit())
                } else {
                    format!("({} {})", head.to_string_explicit(), tail_str)
                }
            }
            Empty => std::string::String::from("()"),
        };
        write!(f, "{}", str)
    }
}
/////////////////////////////////////////////////////////////////////////////////////////////////////
//
//  Iterating through Persistent List
//
////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait ToPersistentListIter {
    fn iter(&self) -> PersistentListIter;
    fn nth(&self, ind: usize) -> Rc<Value> {
        match self.iter().nth(ind) {
            Some(rc_val) => rc_val,
            None => Rc::new(Value::Nil),
        }
    }
}
impl ToPersistentListIter for Rc<PersistentList> {
    fn iter(&self) -> PersistentListIter {
        PersistentListIter {
            list: Rc::clone(self),
        }
    }
}
impl ToPersistentListIter for &Rc<PersistentList> {
    fn iter(&self) -> PersistentListIter {
        PersistentListIter {
            list: Rc::clone(self),
        }
    }
}
impl PersistentList {
    /// Deprecated; use ToPersistentListIter trait to call iter on Rc<PersistentList> directly intead
    pub fn iter(rc_self: &Rc<PersistentList>) -> PersistentListIter {
        PersistentListIter {
            list: Rc::clone(rc_self),
        }
    }
}
pub struct PersistentListIter {
    list: Rc<PersistentList>,
}
impl Iterator for PersistentListIter {
    type Item = Rc<Value>;
    fn next(&mut self) -> Option<Self::Item> {
        match &*(self.list.clone()) {
            Cons(first, rest, _) => {
                self.list = Rc::clone(&rest);
                Some(Rc::clone(&first))
            }
            _ => None,
        }
    }
}
impl FromIterator<Rc<Value>> for PersistentList {
    fn from_iter<I: IntoIterator<Item = Rc<Value>>>(iter: I) -> Self {
        let mut retval = PersistentList::Empty;
        // @TODO see if we can directly loop through our original iter backwards, and avoid
        // dumping into this vector just to loop through again backwards
        let mut coll_as_vec = vec![];
        let mut count = 0;

        for i in iter {
            coll_as_vec.push(i);
        }
        for i in coll_as_vec.iter().rev() {
            count += 1;
            retval = Cons(Rc::clone(i), Rc::new(retval), count);
        }

        retval
    }
}
/////////////////////////////////////////////////////////////////////////////////////////////////////
// End Iteration
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::persistent_list::*;
    #[test]
    fn test_persistent_list_count() {
        let plist = cons(1_i32.to_value(), cons(2_i32.to_value(), Empty));
        let plist2 = cons(
            1_i32.to_value(),
            cons(2_i32.to_value(), cons(3_i32.to_value(), Empty)),
        );
        let plist3 = Empty;
        let plist4 = cons_rc(4_i32.to_rc_value(), Rc::new(plist2.clone()));
        let rc_plist4 = Rc::new(plist4.clone());
        let plist5 = cons_rc(5_i32.to_rc_value(), Rc::clone(&rc_plist4));

        let vec6 = vec![
            1_i32.to_rc_value(),
            2_i32.to_rc_value(),
            3_i32.to_rc_value(),
            4_i32.to_rc_value(),
            5_i32.to_rc_value(),
            6_i32.to_rc_value(),
        ];
        let plist6 = vec6.into_iter().collect::<PersistentList>();
        let plist6_2 = Rc::new(plist6.clone())
            .iter()
            .map(|rc_val| Rc::clone(&rc_val))
            .collect::<PersistentList>();

        assert_eq!(plist.len(), 2);
        assert_eq!(plist2.len(), 3);
        assert_eq!(plist3.len(), 0);
        assert_eq!(plist4.len(), 4);
        assert_eq!(plist5.len(), 5);
        assert_eq!(plist6.len(), 6);
        assert_eq!(plist6_2.len(), 6);
    }
}
