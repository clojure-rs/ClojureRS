use std::fmt;

#[derive(Debug, Clone)]
pub enum TypeTag {
    I32,
    F64,
    Boolean,
    Symbol,
    Class,
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
            Boolean => std::string::String::from("rust.std.bool"),
            F64 => std::string::String::from("rust.std.f64"),
            Symbol => std::string::String::from("clojure.lang.Symbol"),
            Class => std::string::String::from("clojure.lang.Class"),
            Keyword => std::string::String::from("clojure.lang.Keyword"),
            IFn => std::string::String::from("clojure.lang.IFn"),
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

pub fn type_tag_for_name(type_tag_name: &str) -> TypeTag {
    return match type_tag_name {
        "rust.std.i32" => return TypeTag::I32,
        "rust.std.bool" => TypeTag::Boolean,
        "rust.std.f64" => TypeTag::F64,
        "clojure.lang.Symbol" => TypeTag::Symbol,
        "clojure.lang.Class" => TypeTag::Class,
        "clojure.lang.Keyword" => TypeTag::Keyword,
        "clojure.lang.Function" => TypeTag::IFn,
        "clojure.lang.Condition" => TypeTag::Condition,
        "clojure.lang.PersistentList" => TypeTag::PersistentList,
        "clojure.lang.PersistentVector" => TypeTag::PersistentVector,
        "clojure.lang.PersistentListMap" => TypeTag::PersistentListMap,
        "clojure.lang.Macro" => TypeTag::Macro,
        "rust.std.string.String" => TypeTag::String,
        "clojure.lang.Integer" => TypeTag::Integer,
        "clojure.lang.ISeq" => TypeTag::ISeq,
        "clojure.lang.Nil" => TypeTag::Nil,
        "rust.regex" => TypeTag::Pattern,
        _ => TypeTag::Nil,
    };
}
