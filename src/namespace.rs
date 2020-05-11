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
    pub fn from_sym(name: Symbol) -> Namespace {
        Namespace::new(name, RefCell::new(HashMap::new()))
    }
    pub fn insert(&self, sym: Symbol, val: Rc<Value>) {
        self.mappings.borrow_mut().insert(sym, val);
    }
    pub fn get(&self, sym: &Symbol) -> Rc<Value> {
        match self.mappings.borrow_mut().get(sym) {
            Some(val) => Rc::clone(val),
            None => Rc::new(Value::Condition(format!("1 Undefined symbol {}", sym.name))),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Namespaces(pub RefCell<HashMap<Symbol, Namespace>>);

impl Namespaces {
    pub fn new() -> Namespaces {
        Namespaces(RefCell::new(HashMap::new()))
    }
    /// Insert a new namespace of name (sym)
    pub fn insert(&self, sym: Symbol, namespace: Namespace) {
        // When storing / retrieving from namespaces, we want
        // namespace unqualified keys
        let sym = sym.unqualified();
        self.0.borrow_mut().insert(sym, namespace);
    }
    pub fn has_namespace(&self, namespace_sym: &Symbol) -> bool {
        let namespace_sym = namespace_sym.unqualified();

        let namespaces = self.0.borrow();
        let namespace = namespaces.get(&namespace_sym);
        match namespace {
            Some(_) => true,
            None => false,
        }
    }
    // @TODO Consider writing `sym` as reference here, because we clone it anyways
    //       Only reason to keep it is `inserts` are pass-by-value here normally,
    //       since the idea normally is you are literally inserting the keys too
    //       I'd prefer that consistency, unless we find it has a noticeable
    //       performance impact
    /// Insert a binding (sym = val) *into* namespace (namespace)
    pub fn insert_into_namespace(&self, namespace_sym: &Symbol, sym: Symbol, val: Rc<Value>) {
        let mut namespace_sym = &namespace_sym.unqualified();
        // We will only use this if ns isn't ""
        let symbol_namespace_sym = Symbol::intern(&sym.ns);

        if sym.ns != "" {
            namespace_sym = &symbol_namespace_sym;
        }

        let namespaces = self.0.borrow();
        let namespace = namespaces.get(namespace_sym);
        match namespace {
            Some(namespace) => {
                namespace.insert(sym.unqualified(), val);
            }
            None => {
                drop(namespaces);
                let namespace = Namespace::from_sym(namespace_sym.clone());
                namespace.insert(sym.unqualified(), val);
                self.insert(namespace_sym.unqualified(), namespace);
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
        if sym.ns != "" {
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

    mod namespaces_tests {
        use crate::namespace::Namespaces;
        use crate::symbol::Symbol;
        use crate::value::Value;
        use std::rc::Rc;

        ////////////////////////////////////////////////////////////////////////////////////////////////////
        //
        //  pub fn get(&self,namespace_sym: &Symbol,sym: &Symbol) -> Rc<Value>
        //
        ////////////////////////////////////////////////////////////////////////////////////////////////////
        #[test]
        fn get_namespace_get_empty_and_fail() {
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
        fn get_namespace_qualified_symbol_overriding_namespace() {
            let namespaces = Namespaces::new();

            let clojure_core1_plus_1 = Symbol::intern("clojure.core1/+1");
            namespaces.insert_into_namespace(
                &Symbol::intern("clojure.core1"),
                Symbol::intern("+1"),
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
        fn get_namespace_overwritten_namespace_again() {
            let namespaces = Namespaces::new();

            let clojure_core_plus = Symbol::intern("clojure.core/+");
            namespaces.insert_into_namespace(
                &Symbol::intern("clojure.core"),
                Symbol::intern("+"),
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
        fn get_namespace_namespace_symbol_and_symbol_separate() {
            let namespaces = Namespaces::new();

            // add namespace core2/+2
            let plus_2 = Symbol::intern("+2");
            namespaces.insert_into_namespace(
                &Symbol::intern("core2"),
                Symbol::intern("+2"),
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
        fn get_namespace_wrong_ns_right_name() {
            let namespaces = Namespaces::new();
            namespaces.insert_into_namespace(
                &Symbol::intern("core2"),
                Symbol::intern("+2"),
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
