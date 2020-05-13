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
    pub fn new(name: &Symbol, mappings: RefCell<HashMap<Symbol, Rc<Value>>>) -> Namespace {
        Namespace {
            name: name.unqualified(),
            mappings,
        }
    }
    pub fn from_sym(name: &Symbol) -> Namespace {
        Namespace::new(name, RefCell::new(HashMap::new()))
    }
    pub fn insert(&self, sym: &Symbol, val: Rc<Value>) {
        self.mappings.borrow_mut().insert(sym.unqualified(), val);
    }
    pub fn get(&self, sym: &Symbol) -> Rc<Value> {
        match self.mappings.borrow_mut().get(&sym.unqualified()) {
            Some(val) => Rc::clone(val),
            None => Rc::new(Value::Condition(format!("1 Undefined symbol {}", sym.name))),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Namespaces(RefCell<HashMap<Symbol, Namespace>>);

impl Namespaces {
    pub fn new() -> Namespaces {
        Namespaces(RefCell::new(HashMap::new()))
    }
    fn insert(&self, namespace: Namespace) {
        // When storing / retrieving from namespaces, we want
        // namespace unqualified keys
        self.0
            .borrow_mut()
            .insert(namespace.name.unqualified(), namespace);
    }
    /// Adds a new namespace to internal HashMap (but does
    /// *not* return a Namespace or reference to one)
    pub fn create_namespace(&self, sym: &Symbol) {
        self.insert(Namespace::from_sym(sym));
    }
    /// Insert a new namespace of name (sym)
    pub fn has_namespace(&self, namespace_sym: &Symbol) -> bool {
        let namespace_sym = namespace_sym.unqualified();

        let namespaces = self.0.borrow();
        let namespace = namespaces.get(&namespace_sym);
        match namespace {
            Some(_) => true,
            None => false,
        }
    }
    /// Insert a binding (sym = val) *into* namespace (namespace)
    /// If namespace doesn't exist, create it
    pub fn insert_into_namespace(&self, namespace_sym: &Symbol, sym: &Symbol, val: Rc<Value>) {
        let mut namespace_sym = &namespace_sym.unqualified();
        // We will only use this if ns isn't ""
        let symbol_namespace_sym = Symbol::intern(&sym.ns);

        if sym.has_ns() {
            namespace_sym = &symbol_namespace_sym;
        }

        let namespaces = self.0.borrow();
        let namespace = namespaces.get(namespace_sym);
        match namespace {
            Some(namespace) => {
                namespace.insert(sym, val);
            }
            None => {
                drop(namespaces);
                let namespace = Namespace::from_sym(namespace_sym);
                namespace.insert(sym, val);
                self.insert(namespace);
            }
        }
    }
    /// Get value of sym at namespace
    pub fn get(&self, namespace_sym: &Symbol, sym: &Symbol) -> Rc<Value> {
        // When storing / retrieving from namespaces, we want
        // namespace_sym unqualified keys
        let mut namespace_sym = namespace_sym.unqualified();

        // @TODO just make it an Optional<String>
        // If our sym is namespace qualified,  use that as our namespace
        if sym.has_ns() {
            namespace_sym = Symbol::intern(&sym.ns);
        }

        let sym = sym.unqualified();
        let namespaces = self.0.borrow();
        let namespace = namespaces.get(&namespace_sym);

        match namespace {
            Some(namespace) => Rc::clone(&namespace.get(&sym)),
            // @TODO should this be a condition or nil?
            _ => Rc::new(Value::Condition(format!("Undefined symbol {}", sym.name))),
        }
    }
}

#[cfg(test)]
mod tests {
    // 'Struct' here because its not immediately clear why, when testing this,
    // why the word 'namespace' is repeated and that this is actually specifically
    // a struct
    mod namespace_struct {
        use crate::namespace::Namespace;
        use crate::symbol::Symbol;
        use crate::value::Value;
        use std::cell::RefCell;
        use std::collections::HashMap;
        use std::rc::Rc;

        #[test]
        fn new() {
            let namespace = Namespace::new(&Symbol::intern("a"), RefCell::new(HashMap::new()));
            assert_eq!(namespace.name, Symbol::intern("a"));
            assert!(namespace.mappings.borrow().is_empty());
        }

        #[test]
        fn new_removes_namespace_from_qualified_symbol() {
            let namespace = Namespace::new(
                &Symbol::intern_with_ns("ns", "a"),
                RefCell::new(HashMap::new()),
            );
            assert_eq!(namespace.name, Symbol::intern("a"));
            assert!(namespace.name != Symbol::intern_with_ns("ns", "a"));
            assert!(namespace.mappings.borrow().is_empty());
        }
        #[test]
        fn new_namespace_starts_empty() {
            let namespace = Namespace::new(&Symbol::intern("a"), RefCell::new(HashMap::new()));
            let namespace2 = Namespace::new(
                &Symbol::intern_with_ns("ns", "b"),
                RefCell::new(HashMap::new()),
            );
            assert!(namespace.mappings.borrow().is_empty());
            assert!(namespace2.mappings.borrow().is_empty());
        }

        #[test]
        fn from_sym() {
            let namespace = Namespace::from_sym(&Symbol::intern_with_ns("ns", "name"));
            assert_eq!(namespace.name, Symbol::intern("name"));
            assert!(namespace.name != Symbol::intern_with_ns("ns", "name"));
            assert!(namespace.mappings.borrow().is_empty());
        }
        #[test]
        fn insert() {
            let namespace = Namespace::from_sym(&Symbol::intern("name"));
            namespace.insert(&Symbol::intern("a"), Rc::new(Value::Nil));
            namespace.insert(&Symbol::intern_with_ns("ns", "b"), Rc::new(Value::Nil));
            assert_eq!(namespace.name, Symbol::intern("name"));
            assert!(namespace.name != Symbol::intern("ns"));
            assert!(namespace.name != Symbol::intern_with_ns("ns", "name"));

            namespace.insert(&Symbol::intern("c"), Rc::new(Value::Nil));
            match &*namespace.get(&Symbol::intern("c")) {
                Value::Condition(_) => panic!("We are unable to get a symbol we've put into our namespace created with from_sym()"),
                _ => {}
            }
        }

        #[test]
        fn get() {
            let namespace = Namespace::from_sym(&Symbol::intern("name"));
            namespace.insert(&Symbol::intern("a"), Rc::new(Value::Nil));
            namespace.insert(&Symbol::intern_with_ns("ns", "b"), Rc::new(Value::Nil));
            match &*namespace.get(&Symbol::intern("a")) {
                Value::Condition(_) => panic!("We are unable to get a symbol we've put into our namespace created with from_sym()"),
                _ => {}
            }

            match &*namespace.get(&Symbol::intern("b")) {
                Value::Condition(_) => panic!("We are unable to get a symbol we've put into our namespace created with from_sym()"),
                _ => {}
            }

            match &*namespace.get(&Symbol::intern("ns")) {
                Value::Condition(_) => {}
                _ => panic!("We are able to get a symbol whose name is the namespace of another symbol we inserted (and note, that namesapce should be dropped altogether on insert)"),
            }

            match &*namespace.get(&Symbol::intern("sassafrass")) {
                Value::Condition(_) => {}
                _ => panic!(
                    "We are able to get a symbol we didn't insert without a Condition being thrown"
                ),
            }

            match &*namespace.get(&Symbol::intern_with_ns("ns","b")) {
                Value::Condition(_) => panic!("We are unable to get a symbol by trying to get a namespace qualified version of it (the namespace normally should be irrelevant and automatically drop)"),
                _ => {}
            }

            match &*namespace.get(&Symbol::intern_with_ns("chicken","a")) {
                Value::Condition(_) => panic!("We are unable to get a symbol by trying to get a namespace qualified (with a random namespace) version of it (the namespace normally should be irrelevant and automatically drop)"),
                _ => {}
            }
        }
    }
    mod namespaces_newtype {
        use crate::namespace::Namespace;
        use crate::namespace::Namespaces;
        use crate::symbol::Symbol;
        use crate::value::Value;
        use std::rc::Rc;
        fn new() {
            let namespaces = Namespaces::new();
            assert!(namespaces.0.borrow().is_empty());
        }
        fn insert() {
            let namespaces = Namespaces::new();
            let namespace = Namespace::from_sym(&Symbol::intern("clojure.core"));
            namespace.insert(&Symbol::intern("+"), Rc::new(Value::Nil));
            // Namespace should be dropped; doesn't matter when inserting into
            // a namespace
            namespace.insert(
                &Symbol::intern_with_ns("clojure.math", "+"),
                Rc::new(Value::Nil),
            );
            /////////////////////////////////////////////////////////////
            namespaces.insert(namespace);

            assert_eq!(
                Value::Nil,
                *namespaces.get(&Symbol::intern("clojure.core"), &Symbol::intern("+"))
            );
            assert_eq!(
                Value::Nil,
                *namespaces.get(
                    &Symbol::intern_with_ns("ns-doesn't-matter", "clojure.core"),
                    &Symbol::intern("+")
                )
            );
            assert_eq!(
                Value::Nil,
                *namespaces.get(
                    &Symbol::intern_with_ns("ns-doesn't-matter", "clojure.core"),
                    &Symbol::intern_with_ns("ns-still-doesn't-matter", "+")
                )
            );
            //Ie, we should get a Condition, because there is no clojure.math/+
            assert!(
                Value::Nil
                    != *namespaces.get(&Symbol::intern("clojure.math"), &Symbol::intern("+"))
            );
        }
        fn has_namespace() {
            let namespaces = Namespaces::new();
            let namespace = Namespace::from_sym(&Symbol::intern("clojure.core"));
            namespace.insert(&Symbol::intern("+"), Rc::new(Value::Nil));

            assert!(namespaces.has_namespace(&Symbol::intern("clojure.core")));
            assert!(namespaces
                .has_namespace(&Symbol::intern_with_ns("ns-doesn't-matter", "clojure.core")));
            assert!(!namespaces.has_namespace(&Symbol::intern("+")));
            // Note; ns-doesn't-matter *isn't* the namespace this time
            assert!(!namespaces
                .has_namespace(&Symbol::intern_with_ns("clojure.core", "ns-doesn't-matter")));
        }
        fn insert_into_namespace() {
            let namespaces = Namespaces::new();
            namespaces.insert_into_namespace(
                &Symbol::intern("clojure.core"),
                &Symbol::intern("+"),
                Rc::new(Value::Nil),
            );
            assert!(!namespaces.has_namespace(&Symbol::intern("random_ns")));
            assert!(namespaces.has_namespace(&Symbol::intern("clojure.core")));

            assert_eq!(
                Value::Nil,
                *namespaces.get(&Symbol::intern("clojure.core"), &Symbol::intern("+"))
            );
            assert!(
                Value::Nil
                    != *namespaces.get(
                        &Symbol::intern("clojure.core"),
                        &Symbol::intern("other-sym")
                    )
            );
            assert!(
                Value::Nil != *namespaces.get(&Symbol::intern("other-ns"), &Symbol::intern("+"))
            );
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////
        //
        //  pub fn get(&self,namespace_sym: &Symbol,sym: &Symbol) -> Rc<Value>
        //
        ////////////////////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn get_get_empty_and_fail() {
            let namespaces = Namespaces::new();
            let clojure_core_plus = Symbol::intern("clojure.core/+");
            match &*namespaces.get(&Symbol::intern("clojure.your/+"), &clojure_core_plus) {
                Value::Condition(_) => {}
                _ => {
                    panic!(
                        "Symbol {} somehow succeeded in {:#?}",
                        clojure_core_plus, namespaces
                    );
                }
            }
        }

        #[test]
        fn get_qualified_symbol_overriding_namespace() {
            let namespaces = Namespaces::new();

            let clojure_core1_plus_1 = Symbol::intern("clojure.core1/+1");
            namespaces.insert_into_namespace(
                &Symbol::intern("clojure.core1"),
                &Symbol::intern("+1"),
                Rc::new(Value::Nil),
            );
            match &*namespaces.get(&Symbol::intern("clojure.your"), &clojure_core1_plus_1) {
                Value::Condition(_) => {
                    panic!(
                        "Symbol {} somehow failed in {:#?}",
                        clojure_core1_plus_1, namespaces
                    );
                }
                _ => {
                    assert!(true);
                }
            }
        }

        #[test]
        fn get_overwritten_namespace_again() {
            let namespaces = Namespaces::new();

            let clojure_core_plus = Symbol::intern("clojure.core/+");
            namespaces.insert_into_namespace(
                &Symbol::intern("clojure.core"),
                &Symbol::intern("+"),
                Rc::new(Value::Nil),
            );
            // Really means get +/+,  but is overwritten to mean get clojure.core/+
            match &*namespaces.get(&Symbol::intern("clojure.core/+"), &clojure_core_plus) {
                Value::Condition(_) => {
                    panic!(
                        "Symbol {} somehow failed in {:#?}",
                        clojure_core_plus, namespaces
                    );
                }
                _ => {}
            }
        }

        #[test]
        fn get_namespace_symbol_and_symbol_separate() {
            let namespaces = Namespaces::new();

            // add namespace core2/+2
            let plus_2 = Symbol::intern("+2");
            namespaces.insert_into_namespace(
                &Symbol::intern("core2"),
                &Symbol::intern("+2"),
                Rc::new(Value::Nil),
            );
            // Get intern("core2/+2")
            // ----------------------
            // Here is the part where namespace symbol and symbol are separate;
            // rather than having &plus_2 qualified fully as 'core2/+2'
            // ---------------------
            // Should succeed
            match &*namespaces.get(&Symbol::intern("core2"), &plus_2) {
                Value::Condition(_) => {
                    panic!("Symbol {} somehow failed in {:#?}", &plus_2, namespaces);
                }
                _ => {
                    assert!(true);
                }
            }
        }
        #[test]
        fn get_wrong_ns_right_name() {
            let namespaces = Namespaces::new();
            namespaces.insert_into_namespace(
                &Symbol::intern("core2"),
                &Symbol::intern("+2"),
                Rc::new(Value::Nil),
            );

            let plus_2 = Symbol::intern("+2");
            // get intern("core1/+2")
            // Should fail
            match &*namespaces.get(&Symbol::intern("clojure.core1"), &plus_2) {
                Value::Condition(_) => {
                    assert!(true);
                }
                _ => {
                    panic!("Symbol {} somehow failed in {:#?}", &plus_2, namespaces);
                }
            }

            // Make sure it normally works
            // get intern("core2/+2")
            // Should succeed
            match &*namespaces.get(&Symbol::intern("core2"), &plus_2) {
                Value::Condition(_) => {
                    panic!("Symbol {} somehow failed in {:#?}", &plus_2, namespaces);
                }
                _ => {
                    assert!(true);
                }
            }
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////
        //
        ////////////////////////////////////////////////////////////////////////////////////////////////////

        //let namespaces.insert_into_namespace(&Symbol::intern("clojure.core/+"), , ${3:val: Rc<Value>})
    }
}
