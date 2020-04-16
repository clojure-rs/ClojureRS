use std::fmt;

#[derive(Debug,Clone)]
pub enum TypeTag {
    I32,
    Symbol,
    IFn,
    Condition,
    PersistentList,
    PersistentVector,
    // Experimental; may make no sense at runtime, as we will likely be unable to take the value of a macro 
    Macro,
    String,
    Nil
}
use TypeTag::*;
impl fmt::Display for TypeTag {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	let str = match self {
	    I32 => std::string::String::from("rust.std.i32"),
	    Symbol => std::string::String::from("clojure.lang.Symbol"),
	    IFn => std::string::String::from("clojure.lang.Function"),
	    Condition => std::string::String::from("clojure.lang.Condition"),
	    PersistentList => std::string::String::from("clojure.lang.PersistentList"),
	    PersistentVector => std::string::String::from("clojure.lang.PersistentVector"),
	    Macro => std::string::String::from("clojure.lang.Macro"),
	    TypeTag::String => std::string::String::from("rust.std.string.String"),
	    Nil => std::string::String::from("clojure.lang.Nil")
	};
	write!(f, "{}",str)
    }
}
