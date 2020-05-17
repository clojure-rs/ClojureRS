use crate::symbol::Symbol;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
/// This is a list of namespaces your namespace has completely referred, and the symbols
/// that your namespace has individually referred 
/// Example:
/// ```clojure
/// (ns cats (:require [dogs :refer :all] [chickens :refer [a b c]))`
/// ```
/// =>
/// ```
/// Refers {
///    refers: [
///        Symbol::intern("clojure.core"),
///        Symbol::intern("dogs")
///    },
///    syms:  {
///        Symbol::intern("chickens") : [ Symbol::intern("a"),..,Symbol::intern("c")]
///    }
/// }
/// ``
#[derive(Debug, Clone)]
pub struct Refers {
    /// Namespaces that you have completely referred into your namespace;
    /// Basically, `[blah :refer :all]`
    pub namespaces: Vec<Symbol>,
    /// Symbols that you have individually referred into your namespace from another;
    /// Basically, `[blah :refer [a b c]]`
    pub syms: HashMap<Symbol,Vec<Symbol>>
}

//@TODO see if can hide default constructor? Perhaps look at Vector implementation while offline 
impl Refers {
    pub fn new(namespaces: Vec<Symbol>, syms: HashMap<Symbol,Vec<Symbol>>) -> Refers {
        if namespaces.contains(&Symbol::intern("clojure.core")) {
            Refers { namespaces, syms }
        } else
        {
            let mut namespaces_with_default = vec![Symbol::intern("clojure.core")];
            namespaces_with_default.extend_from_slice(&namespaces);
            Refers { namespaces: namespaces_with_default, syms }
        }
        
    }
    // @TODO does this really need to be a vec ? 
    pub fn from_namespace_names(namespaces: Vec<&str>) -> Refers {
        Refers::new(
            namespaces.into_iter().map(Symbol::intern).collect::<Vec<Symbol>>(),
            HashMap::new()
        )
    }
}

impl Default for Refers {
    fn default() -> Self {
        Refers::new(vec![Symbol::intern("clojure.core")],HashMap::new()) 
    }
}

#[derive(Debug, Clone)]
pub struct Namespace {
    pub name: Symbol,
    mappings: RefCell<HashMap<Symbol, Rc<Value>>>,
    pub refers: RefCell<Refers>
}

impl Namespace {
    fn new(name: &Symbol, mappings: HashMap<Symbol, Rc<Value>>,refers: Refers) -> Namespace {
        Namespace {
            name: name.unqualified(),
            mappings: RefCell::new(mappings),
            refers: RefCell::new(refers)
        }
    }
    pub fn from_sym(name: &Symbol) -> Namespace {
        Namespace::new(name, HashMap::new(), Refers::default())
    }

    pub fn from_sym_with_refers(name: &Symbol, refers: Refers) -> Namespace {
        Namespace::new(name,HashMap::new(),refers)
    }

    pub fn insert(&self, sym: &Symbol, val: Rc<Value>) {
        self.mappings.borrow_mut().insert(sym.unqualified(), val);
    }

    pub fn try_get(&self, sym: &Symbol) -> Option<Rc<Value>> {
        match self.mappings.borrow_mut().get(&sym.unqualified()) {
            Some(val) => Some(Rc::clone(val)),
            None => None 
        }
    }
    pub fn get(&self, sym: &Symbol) -> Rc<Value> {
        match self.try_get(sym) {
            Some(val) => val,
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

    pub fn create_namespace_with_refers(&self, sym: &Symbol, refers: Refers) {
        self.insert(Namespace::from_sym_with_refers(sym, refers));
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

    /// Like get, but slightly lower level; returns a None on failure rather than a
    /// Value::Condition. See docs for get 
    pub fn try_get(&self, namespace_sym: &Symbol, sym: &Symbol) -> Option<Rc<Value>> {
        // When storing / retrieving from namespaces, we want
        // namespace_sym unqualified keys
        let mut namespace_sym = namespace_sym.unqualified();
        // Ie, a scenario like get(.. , 'clojure.core/+) or get(.., 'shortcut/+)
        let mut grabbing_from_namespace_directly = false;

        // If our sym is namespace qualified,  use that as our namespace
        if sym.has_ns() {
            grabbing_from_namespace_directly = true;
            namespace_sym = Symbol::intern(&sym.ns);
        }

        let sym = sym.unqualified();
        let namespaces = self.0.borrow();
        let namespace = namespaces.get(&namespace_sym)?;

        
        // If we cannot find the symbol, and its not a direct grab from a specific namespace,
        // we should see if we can find it in one of our referred namespaces or symbols
        let val = namespace.try_get(&sym);
        match val {
            Some(_) => val,
            None => {
                if grabbing_from_namespace_directly {
                    return None;
                }
                let refers = namespace.refers.borrow();
                let referred_namespaces = &refers.namespaces;
                // Ex: looping through 
                //     vec![Symbol::intern("clojure.core"), Symbol::intern("clojure.string")]
                for referred_namespace_sym in referred_namespaces.into_iter() {
                    // clojure.core, for instance, refers itself technically, so we don't want an infinite loop 
                    if *referred_namespace_sym == namespace_sym {
                        continue;
                    }
                    // Ex: let's try to get, then, from "clojure.core or "clojure.string" 
                    let try_get_sym_from_other_ns = self.try_get(&referred_namespace_sym,&sym);
                    // And if we get a value, return it 
                    if let Some(_) = &try_get_sym_from_other_ns {
                        return try_get_sym_from_other_ns;
                    }
                }
                //
                // @TODO swap the order of these clauses to put this first perhaps 
                //
                let referred_syms_map = &refers.syms;
                // Ex:
                //  { 'clojure.core ['a 'b 'c] ,
                //    'clojure.string ['x 'y 'z]}
                //  referred_namespace_sym: Symbol::intern("clojure.core"),
                //  referred_syms:          vec![Symbol::intern("x"), .. Symbol::intern("z")] 
                for (referred_namespace_sym, referred_syms) in referred_syms_map.iter() {
                    // Ex: (if we're trying to get, say,  '+)
                    //     Do we even refer a '+ from this namespace?
                    //     'clojure.string ['x 'y 'z] <-- no
                    //     Continue then 
                    if !referred_syms.contains(&sym) {
                        continue;
                    }
                    // Again, let's just avoid any infinite loops 
                    if *referred_namespace_sym == namespace_sym {
                        continue;
                    }
                    // If we *have* referred the sym we're looking for from this ns
                    // let's try to get it
                    // Ex:  try_get('clojure.string, '+)
                    let try_get_sym_from_other_ns = self.try_get(&referred_namespace_sym,&sym);
                    // And if we get a value, return it 
                    if let Some(_) = &try_get_sym_from_other_ns {
                        return try_get_sym_from_other_ns;
                    }
                }
                None
            }
        }

    }

    /// Get value of sym in namespace
    /// Note;
    /// ```
    ///  get('clojure.core,'+)
    /// ```
    /// Will be asking what '+ means in 'clojure.core, so
    /// this will only return a value if there is a 'clojure.core/+
    /// But
    /// ```
    /// get('clojure.core, 'clojure.other/+)
    /// ```
    /// Will always mean the same thing, no matter what namespace we're in; it will mean
    /// the value '+ belonging to clojure.other,  the namespace you're in is irrelevant
    ///
    /// Finally,
    /// ```
    /// get('clojure.core, 'shortcut/+)
    /// ```
    /// Will depend on what shortcut expands to in clojure.core (assuming shortcut is not an actual namespace here)
    /// 
    /// As we can see, this is a relatively high level function meant to be getting _whatever_
    /// a user has typed in for a symbol while inside a namespace 
    pub fn get(&self, namespace_sym: &Symbol, sym: &Symbol) -> Rc<Value> {
        match self.try_get(namespace_sym,sym) {
            Some(val) => val,
            // @TODO should this be a condition or nil?
            None => Rc::new(Value::Condition(format!("Undefined symbol {}", sym.name)))
        }
    }
}

#[cfg(test)]
mod tests {
    mod refers_struct {
        use crate::symbol::Symbol;
        use std::collections::HashMap;
        use crate::namespace::Refers;

        #[test]
        fn new_with_empty_refers() {
            let refers_everything_empty = Refers::new(vec![], HashMap::new());
            assert!(refers_everything_empty.namespaces.contains(&Symbol::intern("clojure.core")));
        }
        #[test]
        fn new() {
            let mut syms = HashMap::new();
            syms.insert(Symbol::intern("clojure.weird"), vec![Symbol::intern("a"),Symbol::intern("b"),Symbol::intern("c")]);

            let refers_without_core = Refers::new(vec![Symbol::intern("clojure.string")], syms);
            assert!(refers_without_core.namespaces.contains(&Symbol::intern("clojure.core")));
            assert!(refers_without_core.namespaces.contains(&Symbol::intern("clojure.string")));
            
            let mut syms2 = HashMap::new();
            syms2.insert(Symbol::intern("clojure.weird"), vec![Symbol::intern("a"),Symbol::intern("b"),Symbol::intern("c")]);

            let refers_with_core = Refers::new(vec![Symbol::intern("clojure.core"),Symbol::intern("clojure.not-core")], syms2);
            assert!(refers_with_core.namespaces.contains(&Symbol::intern("clojure.core")));
            assert!(refers_with_core.namespaces.contains(&Symbol::intern("clojure.not-core")));
            //assert!(refers_without_core.mappings.borrow().is_empty());
        }
    }
    // 'Struct' here because its not immediately clear why, when testing this,
    // why the word 'namespace' is repeated and that this is actually specifically
    // a struct
    mod namespace_struct {
        use crate::namespace::Namespace;
        use crate::namespace::Refers;
        use crate::symbol::Symbol;
        use crate::value::Value;
        use std::cell::RefCell;
        use std::collections::HashMap;
        use std::rc::Rc;
        
        #[test]
        fn new() {
            let namespace = Namespace::new(&Symbol::intern("a"), HashMap::new(), Refers::default());
            assert_eq!(namespace.name, Symbol::intern("a"));
            assert!(namespace.mappings.borrow().is_empty());
        }

        #[test]
        fn new_removes_namespace_from_qualified_symbol() {
            let namespace = Namespace::new(
                &Symbol::intern_with_ns("ns", "a"),
                HashMap::new(),
                Refers::default()
            );
            assert_eq!(namespace.name, Symbol::intern("a"));
            assert!(namespace.name != Symbol::intern_with_ns("ns", "a"));
            assert!(namespace.mappings.borrow().is_empty());
        }
        #[test]
        fn new_namespace_starts_empty() {
            let namespace = Namespace::new(&Symbol::intern("a"), HashMap::new(),Refers::default());
            let namespace2 = Namespace::new(
                &Symbol::intern_with_ns("ns", "b"),
                HashMap::new(),
                Refers::default()
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
        fn from_sym_with_refers() {
            let namespace = Namespace::from_sym_with_refers(
                &Symbol::intern_with_ns("ns", "name"),
                Refers::new(vec![Symbol::intern("referred-ns")],HashMap::new())
            );
            assert_eq!(namespace.name, Symbol::intern("name"));
            assert!(namespace.name != Symbol::intern_with_ns("ns", "name"));
            assert!(namespace.mappings.borrow().is_empty());

            assert!(namespace.refers.borrow().namespaces.contains(&Symbol::intern("clojure.core")));
            assert!(namespace.refers.borrow().namespaces.contains(&Symbol::intern("referred-ns")));
            
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
        use std::collections::HashMap;
        use crate::symbol::Symbol;
        use crate::value::Value;
        use crate::namespace::Refers;
        use std::rc::Rc;
        
        #[test]
        fn new() {
            let namespaces = Namespaces::new();
            assert!(namespaces.0.borrow().is_empty());
        }
        
        #[test]
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
            //Ie, we should get a Condition, because there is no clojure.math/+
            assert!(
                Value::Nil
                    != *namespaces.get(&Symbol::intern("clojure.math"), &Symbol::intern("+"))
            );
        }

        #[test]
        fn create_namespace_with_refers(){
            let namespaces = Namespaces::new();
            namespaces.create_namespace_with_refers(&Symbol::intern("user"), Refers::new(vec![Symbol::intern("referred-ns")], HashMap::new()));

            namespaces.insert_into_namespace(&Symbol::intern("user"), &Symbol::intern("user-fn"), Rc::new(Value::Nil));
            namespaces.insert_into_namespace(&Symbol::intern("clojure.core"), &Symbol::intern("core-fn"), Rc::new(Value::Nil));
            namespaces.insert_into_namespace(&Symbol::intern("referred-ns"), &Symbol::intern("referred-fn"), Rc::new(Value::Nil));
            namespaces.insert_into_namespace(&Symbol::intern("mystery-ns"), &Symbol::intern("mystery-fn"), Rc::new(Value::Nil));

            assert_eq!(
                Value::Nil,
                *namespaces.get(&Symbol::intern("user"), &Symbol::intern("user-fn"))
            );
            assert_eq!(
                Value::Nil,
                *namespaces.get(&Symbol::intern("user"), &Symbol::intern("core-fn"))
            );
            assert_eq!(
                Value::Nil,
                *namespaces.get(&Symbol::intern("referred-ns"), &Symbol::intern("referred-fn"))
            );
            assert_eq!(
                Value::Nil,
                *namespaces.get(&Symbol::intern("user"), &Symbol::intern("referred-fn"))
            );
            match &*namespaces.get(&Symbol::intern("user"), &Symbol::intern("mystery-fn")) {
                Value::Condition(_) => {},
                _ => panic!("user shouldn't know about mystery-fn")
            }
        }

        #[test]
        fn has_namespace() {
            let namespaces = Namespaces::new();
            let namespace = Namespace::from_sym(&Symbol::intern("clojure.core"));
            namespace.insert(&Symbol::intern("+"), Rc::new(Value::Nil));
            namespaces.insert(namespace);

            assert!(namespaces.has_namespace(&Symbol::intern("clojure.core")));
            assert!(namespaces
                .has_namespace(&Symbol::intern_with_ns("ns-doesn't-matter", "clojure.core")));
            assert!(!namespaces.has_namespace(&Symbol::intern("+")));
            // Note; ns-doesn't-matter *isn't* the namespace this time
            assert!(!namespaces
                .has_namespace(&Symbol::intern_with_ns("clojure.core", "ns-doesn't-matter")));
        }

        #[test]
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
        #[test]
        fn get_referred_symbol_from_namespace() {
            let namespaces = Namespaces::new();
            namespaces.create_namespace_with_refers(
                &Symbol::intern("user"),
                Refers::from_namespace_names(vec!["clojure.weird-ns"])
            );
            namespaces.insert_into_namespace(
                &Symbol::intern("clojure.weird-ns"),
                &Symbol::intern("+"),
                Rc::new(Value::Nil),
            );

            namespaces.insert_into_namespace(
                &Symbol::intern("user"),
                &Symbol::intern("-"),
                Rc::new(Value::Nil),
            );

            match &*namespaces.get(&Symbol::intern("user"),&Symbol::intern("+")) {
                Value::Condition(_) => {
                    panic!("Namespace user failed to grab clojure.weird-ns/+ as a referred symbol");
                }
                _ => {}
            }
        }
        // Default referred namespace = clojure.core 
        #[test]
        fn get_from_default_referred_namespace() {
            let namespaces = Namespaces::new();
            namespaces.insert_into_namespace(
                &Symbol::intern("clojure.core"),
                &Symbol::intern("+"),
                Rc::new(Value::Nil),
            );

            namespaces.insert_into_namespace(
                &Symbol::intern("user"),
                &Symbol::intern("-"),
                Rc::new(Value::Nil),
            );

            match &*namespaces.get(&Symbol::intern("user"),&Symbol::intern("+")) {
                Value::Condition(_) => {
                    panic!("Namespace user failed to grab clojure.core/+ as a referred symbol");
                }
                _ => {}
            }

        }

        #[test]
        fn get_individual_referred_syms() {
            let namespaces = Namespaces::new();

            let mut refers_map = HashMap::new();
            refers_map.insert(Symbol::intern("referred-syms-ns"),vec![Symbol::intern("a"), Symbol::intern("b")]);

            namespaces.create_namespace_with_refers(
                &Symbol::intern("user"),
                Refers::new(vec![Symbol::intern("fully-referred-ns")], refers_map)
            );

            namespaces.insert_into_namespace(
                &Symbol::intern("referred-syms-ns"),
                &Symbol::intern("a"),
                Rc::new(Value::Nil),
            );
            namespaces.insert_into_namespace(
                &Symbol::intern("referred-syms-ns"),
                &Symbol::intern("b"),
                Rc::new(Value::Nil),
            );
            namespaces.insert_into_namespace(
                &Symbol::intern("referred-syms-ns"),
                &Symbol::intern("c"),
                Rc::new(Value::Nil),
            );
            // Note; this is the symbol we *shouldn't* be able to get (from either referred ns)
            namespaces.insert_into_namespace(
                &Symbol::intern("referred-syms-ns"),
                &Symbol::intern("d"),
                Rc::new(Value::Nil),
            );

            namespaces.insert_into_namespace(
                &Symbol::intern("fully-referred-ns"),
                &Symbol::intern("c"),
                Rc::new(Value::Nil),
            );
            
            assert_eq!(
                Value::Nil,
                *namespaces.get(&Symbol::intern("user"),&Symbol::intern("a"))
            );
            assert_eq!(
                Value::Nil,
                *namespaces.get(&Symbol::intern("user"),&Symbol::intern("b"))
            );
            assert_eq!(
                Value::Nil,
                *namespaces.get(&Symbol::intern("user"),&Symbol::intern("c"))
            );
            // Ie, this returns a Condition 
            assert!(
                Value::Nil != 
                *namespaces.get(&Symbol::intern("user"),&Symbol::intern("d"))
            );

        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////
        //
        ////////////////////////////////////////////////////////////////////////////////////////////////////

        //let namespaces.insert_into_namespace(&Symbol::intern("clojure.core/+"), , ${3:val: Rc<Value>})
    }
}
