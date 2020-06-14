use std::fmt;

#[derive(Debug, Clone)]
pub enum TypeTag {
    I32,
    F64,
    Boolean,
    Symbol,
    Var,
    Char,
    Keyword,
    IFn,
    Condition,
    PersistentList,
    PersistentVector,
    PersistentListMap,
    // Experimental; may make no sense at runtime, as we will likely be unable to take the value of a macro
    Macro,
    String,
    Integer,
    ISeq,
    Nil,
    Pattern,
}

use TypeTag::*;
impl fmt::Display for TypeTag {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            I32 => std::string::String::from("rust.std.i32"),
            F64 => std::string::String::from("rust.std.f64"),
            Boolean => std::string::String::from("rust.std.bool"),
            Symbol => std::string::String::from("clojure.lang.Symbol"),
            Var => std::string::String::from("clojure.lang.Var"),
            Char => std::string::String::from("clojure.lang.Char"),
            Keyword => std::string::String::from("clojure.lang.Keyword"),
            IFn => std::string::String::from("clojure.lang.Function"),
            Condition => std::string::String::from("clojure.lang.Condition"),
            PersistentList => std::string::String::from("clojure.lang.PersistentList"),
            PersistentVector => std::string::String::from("clojure.lang.PersistentVector"),
            PersistentListMap => std::string::String::from("clojure.lang.PersistentListMap"),
            Macro => std::string::String::from("clojure.lang.Macro"),
            TypeTag::String => std::string::String::from("rust.std.string.String"),
            TypeTag::Integer => std::string::String::from("clojure.lang.Integer"),
            ISeq => std::string::String::from("clojure.lang.ISeq"),
            Nil => std::string::String::from("clojure.lang.Nil"),
            Pattern => std::string::String::from("rust.regex"),
        };
        write!(f, "{}", str)
    }
}
