use std::fmt;
use std::hash::Hash;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Symbol {
    pub name: String,
    // @TODO Should this be an optional string?
    //       on one hand, playing with this is closer to the original,
    //       and slightly easier to read and understand (for me).
    //       But you might say it doesn't force you to cover the None
    //       route, the sort of invariants ADTs are good at.
    //       Most likely, we will reimplement this as Option<String>
    pub ns: String,
}
impl Symbol {
    pub fn intern(name: &str) -> Symbol {
        let mut ns = "";
        let mut name = name;
        // @TODO See if we will have any problems with manipulating
        //       text in other languages
        //       I think we will be ok here though
        if let Some(ind) = name.chars().position(|c| c == '/') {
            // @TODO Make sure that the index given by ^
            //       has the same meaning as the index
            //       we are giving to this range
            //       Ie,  if the 6 in a[..6] refers to the 6th byte
            //       and  if the 6 in Some(6) means the 6th character,
            //       we need to make sure each 'character' in this case
            //       is 1 byte, and not some other grapheme abstraction
            //       else,these are two different indexes

            // support interning of the symbol '/' for division
            if ind > 0 || name.len() > 1 {
                ns = &name[..ind];
                name = &name[ind + 1..];
            }
        }
        Symbol::intern_with_ns(ns, name)
    }
    pub fn intern_with_ns(ns: &str, name: &str) -> Symbol {
        Symbol {
            name: String::from(name),
            ns: String::from(ns),
        }
    }
    pub fn unqualified(&self) -> Symbol {
        Symbol::intern(&self.name)
    }
    pub fn has_ns(&self) -> bool {
        self.ns != ""
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.has_ns() {
            write!(f, "{}/{}", self.ns, self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}
mod tests {

    mod symbol_tests {
        use crate::symbol::Symbol;
        use std::collections::HashMap;

        #[test]
        fn test_intern() {
            assert_eq!(
                Symbol::intern("a"),
                Symbol {
                    ns: String::from(""),
                    name: String::from("a")
                }
            );
        }

        #[test]
        fn test_intern_with_ns() {
            assert_eq!(
                Symbol::intern_with_ns("clojure.core", "a"),
                Symbol {
                    ns: String::from("clojure.core"),
                    name: String::from("a")
                }
            );
            assert_eq!(
                Symbol::intern_with_ns("", "a"),
                Symbol {
                    ns: String::from(""),
                    name: String::from("a")
                }
            );
            assert_eq!(
                Symbol::intern("a"),
                Symbol {
                    ns: String::from(""),
                    name: String::from("a")
                }
            );
            assert_eq!(
                Symbol::intern("clojure.core/a"),
                Symbol {
                    ns: String::from("clojure.core"),
                    name: String::from("a")
                }
            );
            assert_eq!(
                Symbol::intern("clojure/a"),
                Symbol {
                    ns: String::from("clojure"),
                    name: String::from("a")
                }
            );
            assert_eq!(
                Symbol::intern("/a"),
                Symbol {
                    ns: String::from(""),
                    name: String::from("a")
                }
            );
        }
        #[test]
        fn test_work_with_hashmap() {
            let mut hashmap = HashMap::new();
            hashmap.insert(Symbol::intern("+"), 1_i32);
            hashmap.insert(Symbol::intern("-"), 2_i32);

            assert_eq!(1_i32, *hashmap.get(&Symbol::intern("+")).unwrap());
            assert_eq!(2_i32, *hashmap.get(&Symbol::intern("-")).unwrap());
            assert_eq!(None, hashmap.get(&Symbol::intern("*")));
        }
    }
}
