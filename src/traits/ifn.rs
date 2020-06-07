//! Trait for function-like things
//!
//! An IFn is something that can be invoked on arguments, like:
//! a function,
//!    (+ 1 2)
//! a macro
//!    (-> 1 (+ 5) (* 10)))
//! a keyword
//!    (:name {:name "Blah" :age 20})
//! a map
//!    ({:name "Blah" :age 20} :name)
//! As well as a few more types.
use crate::value::Value;

use dyn_clone::DynClone;

use std::fmt::Debug;
use std::rc::Rc;

//
// Based on: clojure.lang.IFn
//
// Naming this is a bit difficult, as
// 1. Rust already has a Fn trait
// 2. Naming this IFn or not naming this IFn introduces consistency and inconsistency;
//
//    Keeping 'IFn' introduces consistency because not just does the
//    original ClojureJVM have an IFn in the Java underneath, but
//    Clojure itself carries on this idea even as we leave the JVM (as,
//    for instance, ifn is implemented in Clojurescript via)
//
//    Inconsistency, because Rust surely does not name traits 'ITrait'.
//
// I've regardless kept it as IFn, and you might say IFn here is
// referring to the Clojure idea of an IFn, implemented in Rust with a
// trait, rather than saying 'this is an interface called Fn'
//

pub trait IFn: Debug + DynClone {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value;
}
dyn_clone::clone_trait_object!(IFn);
