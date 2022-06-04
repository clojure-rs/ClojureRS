use crate::environment::Environment;
use crate::ifn::IFn;
use crate::keyword::Keyword;
use crate::lambda;
use crate::maps::MapEntry;
use crate::persistent_list::PersistentList::Cons;
use crate::persistent_list::{PersistentList, ToPersistentList, ToPersistentListIter};
use crate::persistent_list_map::{PersistentListMap, ToPersistentListMapIter};
use crate::persistent_vector::PersistentVector;
use crate::symbol::Symbol;
use crate::type_tag::TypeTag;
use crate::var::Var;
use core::fmt::Display;

extern crate rand;
use rand::Rng;

use std::cmp::{Ord, Ordering};
use std::fmt;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

// @TODO Change IFn's name -- IFn is a function, not an IFn.
//       The body it executes just happens to be an the IFn.
/// Represents any Value known to ClojureRS, by wrapping any Value known to ClojureRS;
/// an int, a symbol, a fn, and so on.  Some Values here are more specific than others;
/// I32 wraps any I32, but QuoteMacro specifically wraps the value for the quote macro, which
/// is a special case macro that has hardcoded behavior.
#[derive(Debug, Clone)]
pub enum Value {
    I32(i32),
    F64(f64),
    Boolean(bool),
    Symbol(Symbol),
    Var(Var),
    Keyword(Keyword),
    IFn(Rc<dyn IFn>),
    //
    // Special case functions
    //

    // I don't know if this exists in any particular Lisp,
    // but it allows me to reach into our local environment through an invoke
    LexicalEvalFn,

    PersistentList(PersistentList),
    PersistentVector(PersistentVector),
    PersistentListMap(PersistentListMap),

    Condition(std::string::String),
    // Macro body is still a function, that will be applied to our unevaled arguments
    Macro(Rc<dyn IFn>),
    //
    // Special case macros
    //
    QuoteMacro,
    DefmacroMacro,
    DefMacro,
    FnMacro,
    LetMacro,
    IfMacro,

    String(std::string::String),
    Nil,
    Pattern(regex::Regex),
}
use crate::value::Value::*;

impl PartialEq for Value {
    // @TODO implement our generic IFns some other way? After all, again, this isn't Java
    // @TODO improve this? This is a hack
    fn eq(&self, other: &Value) -> bool {
        //
        match (self, other) {
            (Value::I32(i), Value::I32(i2)) => i == i2,
            (Value::F64(d), Value::F64(d2)) => d == d2,
            (Value::Boolean(b), Value::Boolean(b2)) => b == b2,
            (Value::Symbol(sym), Value::Symbol(sym2)) => sym == sym2,
            (Value::Var(var), Value::Var(var2)) => var == var2,
            (Value::Keyword(kw), Value::Keyword(kw2)) => kw == kw2,
            // Equality not defined on functions, similar to Clojure
            // Change this perhaps? Diverge?
            (Value::IFn(_), Value::IFn(_)) => false,
            // Is it misleading for equality to sometimes work?
            (Value::LexicalEvalFn, Value::LexicalEvalFn) => true,
            (Value::PersistentList(plist), Value::PersistentList(plist2)) => plist == plist2,
            (Value::PersistentVector(pvector), Value::PersistentVector(pvector2)) => {
                *pvector == *pvector2
            }
            (Value::PersistentListMap(plistmap), Value::PersistentListMap(plistmap2)) => {
                *plistmap == *plistmap2
            }
            (Value::Condition(msg), Value::Condition(msg2)) => msg == msg2,
            (Value::QuoteMacro, Value::QuoteMacro) => true,
            (Value::DefmacroMacro, Value::DefmacroMacro) => true,
            (Value::DefMacro, Value::DefMacro) => true,
            (Value::LetMacro, Value::LetMacro) => true,
            (Value::String(string), Value::String(string2)) => string == string2,
            (Value::Nil, Value::Nil) => true,
            (Value::Pattern(p1), Value::Pattern(p2)) => p1.as_str() == p2.as_str(),
            _ => false,
        }
    }
}

// Again, this is certainly not the right away to do this
// @FIXME remove this entire monstrocity
#[derive(Debug, Clone, Hash)]
enum ValueHash {
    LexicalEvalFn,
    QuoteMacro,
    DefmacroMacro,
    DefMacro,
    FnMacro,
    IfMacro,
    LetMacro,
    Nil,
}
impl Eq for Value {}
impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::I32(i) => i.hash(state),
            Value::F64(d) => d.to_value().hash(state),
            Value::Boolean(b) => b.hash(state),
            Value::Symbol(sym) => sym.hash(state),
            Value::Var(var) => var.hash(state),
            Value::Keyword(kw) => kw.hash(state),
            Value::IFn(_) => {
                let mut rng = rand::thread_rng();
                let n2: u16 = rng.gen();
                n2.hash(state)
            }
            Value::LexicalEvalFn => (ValueHash::LexicalEvalFn).hash(state),
            Value::PersistentList(plist) => plist.hash(state),
            Value::PersistentVector(pvector) => pvector.hash(state),
            Value::PersistentListMap(plistmap) => plistmap.hash(state),
            Value::Condition(msg) => msg.hash(state),
            // Random hash is temporary;
            // @TODO implement hashing for functions / macros
            Value::Macro(_) => {
                let mut rng = rand::thread_rng();
                let n2: u16 = rng.gen();
                n2.hash(state)
            }
            Value::QuoteMacro => ValueHash::QuoteMacro.hash(state),
            Value::DefmacroMacro => ValueHash::DefmacroMacro.hash(state),
            Value::DefMacro => ValueHash::DefMacro.hash(state),
            Value::FnMacro => ValueHash::FnMacro.hash(state),
            Value::LetMacro => ValueHash::LetMacro.hash(state),
            Value::IfMacro => ValueHash::IfMacro.hash(state),

            Value::String(string) => string.hash(state),
            Value::Pattern(p) => p.as_str().hash(state),
            Value::Nil => ValueHash::Nil.hash(state),
        }
        // self.id.hash(state);
        // self.phone.hash(state);
    }
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Value::I32(val) => val.to_string(),
            Value::F64(val) => val.to_string(),
            Value::Boolean(val) => val.to_string(),
            Value::Symbol(sym) => sym.to_string(),
            Value::Var(var) => var.to_string(),
            Value::Keyword(kw) => kw.to_string(),
            Value::IFn(_) => std::string::String::from("#function[]"),
            Value::LexicalEvalFn => std::string::String::from("#function[lexical-eval*]"),
            Value::PersistentList(plist) => plist.to_string(),
            Value::PersistentVector(pvector) => pvector.to_string(),
            Value::PersistentListMap(plistmap) => plistmap.to_string(),
            Value::Condition(msg) => format!("#Condition[\"{}\"]", msg),
            Value::Macro(_) => std::string::String::from("#macro[]"),
            Value::QuoteMacro => std::string::String::from("#macro[quote*]"),
            Value::DefMacro => std::string::String::from("#macro[def*]"),
            Value::DefmacroMacro => std::string::String::from("#macro[defmacro*]"),
            Value::FnMacro => std::string::String::from("#macro[fn*]"),
            Value::IfMacro => std::string::String::from("#macro[if*]"),
            Value::LetMacro => std::string::String::from("#macro[let*]"),
            Value::String(string) => string.clone(),
            Value::Pattern(pattern) => std::string::String::from(
                "#\"".to_owned() + &pattern.as_str().escape_default().to_string().clone() + "\"",
            ),
            Value::Nil => std::string::String::from("nil"),
        };
        write!(f, "{}", str)
    }
}
impl Value {
    //
    // Likely temporary
    // I cannot remember for the life of me whether or not there's a function like this normally
    // and what its called
    // Regardless, when we have, say, a string inside a list, we want to print the string explicitly
    // with a \"\" and all.
    // Everything else we print as is.
    //
    pub fn to_string_explicit(&self) -> std::string::String {
        match self {
            Value::String(string) => format!("\"{}\"", string),
            _ => self.to_string(),
        }
    }
    pub fn type_tag(&self) -> TypeTag {
        match self {
            Value::I32(_) => TypeTag::I32,
            Value::F64(_) => TypeTag::F64,
            Value::Boolean(_) => TypeTag::Boolean,
            Value::Symbol(_) => TypeTag::Symbol,
            Value::Var(_) => TypeTag::Var,
            Value::Keyword(_) => TypeTag::Keyword,
            Value::IFn(_) => TypeTag::IFn,
            Value::LexicalEvalFn => TypeTag::IFn,
            Value::PersistentList(_) => TypeTag::PersistentList,
            Value::PersistentVector(_) => TypeTag::PersistentVector,
            Value::PersistentListMap(_) => TypeTag::PersistentListMap,
            Value::Condition(_) => TypeTag::Condition,
            // Note; normal Clojure cannot take the value of a macro, so I don't imagine this
            // having significance in the long run, but we will see
            Value::Macro(_) => TypeTag::Macro,
            Value::QuoteMacro => TypeTag::Macro,
            Value::DefMacro => TypeTag::Macro,
            Value::DefmacroMacro => TypeTag::Macro,
            Value::LetMacro => TypeTag::Macro,
            Value::FnMacro => TypeTag::Macro,
            Value::IfMacro => TypeTag::Macro,
            Value::String(_) => TypeTag::String,
            Value::Nil => TypeTag::Nil,
            Value::Pattern(_) => TypeTag::Pattern,
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////
    //
    // Eval Helper function
    //
    //////////////////////////////////////////////////////////////////////////////////////////////////////

    //
    // This function is inherently long, as it is dispatching on all valid function-like (IFn) Value types
    // We could further separate each dispatch case into individual functions, but I don't think that's necessary;
    // its not code that will be reused, and it doesn't make the code inherently shorter, it just moves it around
    // In this case, though, I don't find its movement to be one that would increase clarity;
    // this used to be a part of the overall eval function, and I think that DID
    // obscure a bit of clarity, because it added this extra level of nesting as a huge block in the middle
    // of the function,  and you could no longer just grok the functions shape at a glance,
    // nor could you know right away just by looking at the function what nested level of logic you were in.
    //
    // But now that this is separate, its only one level -- its only a list of implementations for each
    // IFn application, it might as well be a list of functions itself.  It in fact means you don't have to
    // hunt around for each individual implementation.
    //
    /// Applies any valid function-like Value to a PersistentList, or returns None if our Value can't be applied
    fn apply_to_persistent_list(
        &self,
        environment: &Rc<Environment>,
        args: &Rc<PersistentList>,
    ) -> Option<Rc<Value>> {
        match self {
            Value::IFn(ifn) => {
                // Eval arguments
                let evaled_arg_refs = PersistentList::iter(args)
                    .map(|rc_arg| rc_arg.eval_to_rc(Rc::clone(environment)))
                    .collect::<Vec<Rc<Value>>>();

                // Invoke fn on arguments
                Some(Rc::new(ifn.invoke(evaled_arg_refs)))
            }
            Value::LexicalEvalFn => {
                if args.len() != 1 {
                    return Some(Rc::new(Value::Condition(format!(
                        "Wrong number of arguments (Given: {}, Expected: 1)",
                        args.len()
                    ))));
                }
                // This should only be one value
                let evaled_arg_values = PersistentList::iter(args)
                    .map(|rc_arg| rc_arg.eval_to_rc(Rc::clone(environment)))
                    .collect::<Vec<Rc<Value>>>();

                let evaled_arg = evaled_arg_values.get(0).unwrap();

                Some(evaled_arg.eval_to_rc(Rc::clone(environment)))
            }
            //
            // Unless I'm mistaken, this is incorrect; instead of having a phase where
            // the macro expands, and then another phase where the whole expanded form
            // is evaluated, it all happens at once.  I will have to look further into
            // whether or not this will cause any problems; you'd think I'd know more
            // about this particular step by now, but this is an implementation detail
            // that's never interested me all that much
            //
            Value::Macro(ifn) => {
                let arg_refs = PersistentList::iter(args).collect::<Vec<Rc<Value>>>();

                let macroexpansion = Rc::new(ifn.invoke(arg_refs));

                Some(macroexpansion.eval_to_rc(Rc::clone(environment)))
            }
            //
            // Special case macros
            //
            // How these are implemented may change when we redesign macros
            // That being said,  each of these macros introduce a various constraint
            // that makes it easier to hardcode them into the evaluation step
            // (or, for some, surely impossible not to do otherwise)

            //
            // def is a primitive for modifying the environment itself,
            // and it is easier to access the environment during this step,
            // rather than owning some sort of reference to it in our def macro
            // Edit:
            //   The environment has now been modified to make it easy to close
            //   around :D. Originally needed for our lambdas,  we can probably now,
            //   should we choose,  define def without a 'special case macro', an extra
            //   value type -- although we still need to hardcode its definition in Rust,
            //   as an implementation of the generic Value::Macro(Rc<IFn>)
            //
            // (def symbol doc-string? init?)
            Value::DefMacro => {
                let arg_rc_values = PersistentList::iter(args)
                    .map(|rc_arg| rc_arg)
                    .collect::<Vec<Rc<Value>>>();

                if arg_rc_values.len() > 3 || arg_rc_values.is_empty() {
                    return Some(Rc::new(Value::Condition(format!(
                        "Wrong number of arguments (Given: {}, Expected: 1-3)",
                        arg_rc_values.len()
                    ))));
                }

                let defname = arg_rc_values.get(0).unwrap();

                let defval = arg_rc_values
                    .get(if arg_rc_values.len() == 2 { 1 } else { 2 })
                    .or(Some(&Rc::new(Value::Nil)))
                    .unwrap()
                    .eval_to_rc(Rc::clone(&environment));

                let doc_string = if arg_rc_values.len() == 3 {
                    match arg_rc_values.get(1).unwrap().to_value() {
                        Value::String(s) => Value::String(s.to_string()),
                        _ => Value::Nil,
                    }
                } else {
                    Value::Nil
                };

                match &**defname {
                    Value::Symbol(sym) => {
                        let mut meta = sym.meta();

                        if doc_string != Value::Nil {
                            meta = conj!(meta, map_entry!("doc", doc_string));
                        }

                        let sym = sym.with_meta(meta);
                        environment.insert(sym.clone(), defval);
                        // @TODO return var
                        Some(sym.to_rc_value())
                    }
                    _ => Some(Rc::new(Value::Condition(std::string::String::from(
                        "First argument to def must be a symbol",
                    )))),
                }
            }
            Value::DefmacroMacro => {
                let arg_rc_values = PersistentList::iter(args)
                    .map(|rc_arg| rc_arg)
                    .collect::<Vec<Rc<Value>>>();

                if arg_rc_values.len() < 2 || arg_rc_values.is_empty() {
                    return Some(Rc::new(Value::Condition(format!(
                        "Wrong number of arguments (Given: {}, Expected: >=2)",
                        args.len()
                    ))));
                }
                let macro_name = arg_rc_values.get(0).unwrap();
                let macro_args = arg_rc_values.get(1).unwrap();

                let macro_body_exprs = if arg_rc_values.len() <= 2 {
                    &[]
                } else {
                    arg_rc_values.get(2..).unwrap()
                };
                let mut macro_invokable_body_vec =
                    vec![Symbol::intern("fn").to_rc_value(), Rc::clone(macro_args)];
                // vec![do expr1 expr2 expr3]
                macro_invokable_body_vec.extend_from_slice(macro_body_exprs);
                let macro_invokable_body = macro_invokable_body_vec
                    .into_list()
                    .eval(Rc::clone(&environment));
                let macro_value = match &macro_invokable_body {
		    Value::IFn(ifn) => Rc::new(Value::Macro(Rc::clone(&ifn))),
		    _ => Rc::new(Value::Condition(std::string::String::from("Compiler Error: your macro_value somehow compiled into something else entirely.  I don't even know how that happened,  this behavior is hardcoded, that's impressive")))
		};
                Some(
                    vec![
                        Symbol::intern("def").to_rc_value(),
                        Rc::clone(macro_name),
                        macro_value,
                    ]
                    .into_list()
                    .eval_to_rc(Rc::clone(&environment)),
                )
            }
            //
            // (fn [x y z] (+ x y z))
            //
            // @TODO Rename for* everywhere, define for in terms of for* in
            //       ClojureRS
            Value::FnMacro => {
                let arg_rc_values = PersistentList::iter(args)
                    .map(|rc_arg| rc_arg)
                    .collect::<Vec<Rc<Value>>>();

                if arg_rc_values.is_empty() {
                    return Some(Rc::new(Value::Condition(format!(
                        "Wrong number of arguments (Given: {}, Expect: >=1",
                        arg_rc_values.len()
                    ))));
                }
                // Let's not do fn names yet
                // let fnname = arg_rc_values.get(0).unwrap();
                let fn_args = arg_rc_values.get(0).unwrap();
                // Let's not do docstrings yet
                // let docstring = ...
                match &**fn_args {
                    Value::PersistentVector(PersistentVector { vals }) => {
                        let mut arg_syms_vec = vec![];
                        let enclosing_environment =
                            Rc::new(Environment::new_local_environment(Rc::clone(&environment)));
                        for val in vals.iter() {
                            if let Value::Symbol(sym) = &**val {
                                arg_syms_vec.push(sym.clone());
                            }
                        }

                        let fn_body =
			// (fn [x y] ) -> nil 
			    if arg_rc_values.len() <= 1 {
				Rc::new(Value::Nil)
				// (fn [x y] expr) -> expr 
			    } else if arg_rc_values.len() == 2 {
				Rc::clone(arg_rc_values.get(1).unwrap())
				// (fn [x y] expr1 expr2 expr3) -> (do expr1 expr2 expr3) 
			    } else {
				// (&[expr1 expr2 expr3] 
				let body_exprs = arg_rc_values.get(1..).unwrap();
				// vec![do]
				let mut do_body = vec![Symbol::intern("do").to_rc_value()];
				// vec![do expr1 expr2 expr3]
				do_body.extend_from_slice(body_exprs);
				// (do expr1 expr2 expr3) 
				do_body.into_list().to_rc_value()
			    };

                        Some(Rc::new(
                            lambda::Fn {
                                body: fn_body,
                                enclosing_environment,
                                arg_syms: arg_syms_vec,
                            }
                            .to_value(),
                        ))
                    }
                    _ => Some(Rc::new(Value::Condition(std::string::String::from(
                        "First argument to def must be a symbol",
                    )))),
                }
            }
            Value::LetMacro => {
                let arg_rc_values = PersistentList::iter(args)
                    .map(|rc_arg| rc_arg)
                    .collect::<Vec<Rc<Value>>>();
                if arg_rc_values.is_empty() || arg_rc_values.len() > 2 {
                    // @TODO: we give 0 but it may be 3, 4, 5...
                    return Some(Rc::new(Value::Condition(std::string::String::from(
                        "Wrong number of arguments given to let (Given: 0, Expecting: 1 or 2)",
                    ))));
                }
                // Already guaranteed to exist by earlier checks
                let local_bindings = arg_rc_values.get(0).unwrap();
                match &**local_bindings {
                    Value::PersistentVector(vector) => {
                        //let mut local_environment_map : HashMap<Symbol,Rc<Value>> = HashMap::new();
                        let local_environment =
                            Rc::new(Environment::new_local_environment(Rc::clone(environment)));
                        // let chunk_test2 =
                        for pair in vector.vals.chunks(2) {
                            if let Some(rc_sym) = (&*pair).get(0)
                            //(*pair[0]).clone()
                            {
                                let val = (&*pair)
                                    .get(1)
                                    .unwrap()
                                    .eval_to_rc(Rc::clone(&local_environment));
                                if let Value::Symbol(sym) = &(**rc_sym) {
                                    local_environment.insert(sym.clone(), val);
                                    //println!("Sym found: {:?}: {:?}",sym,val)
                                }
                            } else {
                                //println!("Nope; pair: {:?}",pair)
                            }
                        }
                        let body = arg_rc_values.get(1);
                        if let Some(body_) = body {
                            Some(body_.eval_to_rc(local_environment))
                        } else {
                            Some(Rc::new(Value::Nil))
                        }
                    }
                    _ => Some(Rc::new(Value::Condition(std::string::String::from(
                        "Bindings to let should be a vector",
                    )))),
                }
            }
            //
            // Quote is simply a primitive, a macro base case; trying to define quote without
            // quote just involves an infinite loop of macroexpansion. Or so it seems
            //
            Value::QuoteMacro => {
                match args.len().cmp(&1) {
                    Ordering::Greater => Some(Rc::new(Value::Condition(format!(
                        "Wrong number of arguments (Given: {}, Expected: 1)",
                        args.len()
                    )))),
                    // @TODO define is_empty()
                    Ordering::Less => Some(Rc::new(Value::Condition(std::string::String::from(
                        "Wrong number of arguments (Given: 0, Expected: 1)",
                    )))),
                    Ordering::Equal => Some(args.nth(0)),
                }
            }
            Value::IfMacro => {
                if args.len() != 2 && args.len() != 3 {
                    return Some(Rc::new(Value::Condition(format!(
                        "Wrong number of arguments (Given: {}, Expected: 2 or 3)",
                        args.len()
                    ))));
                }
                let arg_refs = PersistentList::iter(args).collect::<Vec<Rc<Value>>>();
                let condition = arg_refs.get(0).unwrap().eval(Rc::clone(environment));

                if condition.is_truthy() {
                    Some(arg_refs.get(1).unwrap().eval_to_rc(Rc::clone(environment)))
                } else {
                    Some(
                        arg_refs
                            .get(2)
                            .unwrap_or(&Rc::new(Value::Nil))
                            .eval_to_rc(Rc::clone(environment)),
                    )
                }
            }
            //
            // If we're not a valid IFn
            //
            _ => None,
        }
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////////
    // Eval Helper
    ////////////////////////////////////////////////////////////////////////////////////////////////////
    pub fn is_truthy(&self) -> bool {
        if let Value::Boolean(false) = self {
            return false;
        }
        if let Value::Nil = self {
            return false;
        }
        true
    }
}

pub trait ToValue {
    fn to_value(&self) -> Value;
    fn to_rc_value(&self) -> Rc<Value> {
        Rc::new(self.to_value())
    }
}

impl ToValue for Value {
    fn to_value(&self) -> Value {
        self.clone()
    }
}

impl ToValue for Rc<Value> {
    fn to_value(&self) -> Value {
        (**self).clone()
    }
}

impl ToValue for i32 {
    fn to_value(&self) -> Value {
        Value::I32(*self)
    }
}

impl ToValue for f64 {
    fn to_value(&self) -> Value {
        Value::F64(*self)
    }
}

impl ToValue for bool {
    fn to_value(&self) -> Value {
        Value::Boolean(*self)
    }
}

impl ToValue for std::string::String {
    fn to_value(&self) -> Value {
        Value::String(self.clone())
    }
}

// Not sure why this has to be done separately from the `str` implementation
impl ToValue for &str {
    fn to_value(&self) -> Value {
        Value::String(std::string::String::from(*self))
    }
}

impl ToValue for str {
    fn to_value(&self) -> Value {
        Value::String(std::string::String::from(self))
    }
}

impl ToValue for regex::Regex {
    fn to_value(&self) -> Value {
        Value::Pattern(self.clone())
    }
}

impl ToValue for Symbol {
    fn to_value(&self) -> Value {
        Value::Symbol(self.clone())
    }
}

impl ToValue for Keyword {
    fn to_value(&self) -> Value {
        Value::Keyword(self.clone())
    }
}

impl ToValue for Rc<dyn IFn> {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::clone(self))
    }
}

impl ToValue for PersistentList {
    fn to_value(&self) -> Value {
        Value::PersistentList(self.clone())
    }
}

impl ToValue for PersistentVector {
    fn to_value(&self) -> Value {
        Value::PersistentVector(self.clone())
    }
}

impl ToValue for PersistentListMap {
    fn to_value(&self) -> Value {
        Value::PersistentListMap(self.clone())
    }
}

impl<T: Display, V: ToValue> ToValue for Result<V, T> {
    fn to_value(&self) -> Value {
        match self {
            Ok(val) => val.to_value(),
            Err(err) => Value::Condition(err.to_string()),
        }
    }
}

/// Allows a type to be evaluated, abstracts evaluation
///
/// Our 'Value' type currently wraps and unites all types that exist within ClojureRS,
/// and therefore all values that are evaluated within ClojureRS,  so when not called on a Value,
/// this mostly acts as a shortcut for evaluating outside types not yet converted into a Value,
/// so you can write something like "1.eval(env)" instead of "1.to_value().eval(env)"
pub trait Evaluable {
    /// Evaluates a value and returns a Rc pointer to the evaluated ClojureRS Value
    /// The program primarily
    fn eval_to_rc(&self, environment: Rc<Environment>) -> Rc<Value>;
    /// Evaluates a value and returns a new ClojureRS Value altogether, by cloning what
    /// eval_to_rc points to
    fn eval(&self, environment: Rc<Environment>) -> Value {
        self.eval_to_rc(environment).to_value()
    }
}

impl Evaluable for Rc<Value> {
    fn eval_to_rc(&self, environment: Rc<Environment>) -> Rc<Value> {
        match &**self {
            // Evaluating a symbol means grabbing the value its been bound to in our environment
            Value::Symbol(symbol) => environment.get(symbol),
            // Evaluating a vector [a b c] just means [(eval a) (eval b) (eval c)]
            Value::PersistentVector(pvector) => {
                // Evaluate each Rc<Value> our PersistentVector wraps
                // and return a new PersistentVector wrapping the new evaluated Values
                let evaled_vals = pvector
                    .vals
                    .iter()
                    .map(|rc_val| rc_val.eval_to_rc(Rc::clone(&environment)))
                    .collect::<PersistentVector>();
                Rc::new(Value::PersistentVector(evaled_vals))
            }
            Value::PersistentListMap(plistmap) => {
                // Evaluate each Rc<Value> our PersistentVector wraps
                // and return a new PersistentVector wrapping the new evaluated Values
                let evaled_vals = plistmap
                    .iter()
                    .map(|map_entry| MapEntry {
                        key: map_entry.key.eval_to_rc(Rc::clone(&environment)),
                        val: map_entry.val.eval_to_rc(Rc::clone(&environment)),
                    })
                    .collect::<PersistentListMap>();
                Rc::new(Value::PersistentListMap(evaled_vals))
            }
            // Evaluating a list (a b c) means calling a as a function or macro on arguments b and c
            Value::PersistentList(plist) => match plist {
                Cons(head, tail, __count) => {
                    // First we have to evaluate the head of our list and make sure it is function-like
                    // and can be invoked on our arguments
                    // (ie, a fn, a macro, a keyword ..)
                    // @TODO remove clone if possible
                    let ifn = Rc::clone(head).eval_to_rc(Rc::clone(&environment));

                    let try_apply_ifn =
                        ifn.apply_to_persistent_list(&Rc::clone(&environment), tail);

                    // Right now we're using the normal error message, however maybe later we will try
                    //
                    // You tried to call value of type {} like a function, but only types of the
                    // interface clojure.lang.IFn can be called this way
                    //
                    // Sounds less correct but also seems clearer; the current error message relies on
                    // you pretty much already knowing when this error message is called
                    try_apply_ifn.unwrap_or_else(|| {
                        Rc::new(Value::Condition(format!(
                            "Execution Error: {} cannot be cast to clojure.lang.IFn",
                            ifn.type_tag()
                        )))
                    })
                }
                // () evals to ()
                PersistentList::Empty => Rc::new(Value::PersistentList(PersistentList::Empty)),
            },
            // Other types eval to self; (5 => 5,  "cat" => "cat",  #function[+] => #function[+]
            _ => Rc::clone(&self),
        }
    }
}
impl Evaluable for PersistentList {
    fn eval_to_rc(&self, environment: Rc<Environment>) -> Rc<Value> {
        self.to_rc_value().eval_to_rc(environment)
    }
}
impl Evaluable for Value {
    fn eval_to_rc(&self, environment: Rc<Environment>) -> Rc<Value> {
        self.to_rc_value().eval_to_rc(environment)
    }
}
mod tests {
    use crate::environment::Environment;
    use crate::keyword::Keyword;
    use crate::maps::MapEntry;
    use crate::persistent_list_map::IPersistentMap;
    use crate::persistent_list_map::PersistentListMap;
    use crate::protocol::ProtocolCastable;
    use crate::protocols;
    use crate::symbol::Symbol;
    use crate::traits::IMeta;
    use crate::value::ToValue;
    use crate::value::Value;
    use std::rc::Rc;
    // (def ^{:cat 1 :dog 2} a "Docstring" 1)
    // ==>
    // a with meta of {:cat 1 :dog 2 :doc "Docstring"} ?
    #[test]
    fn def_with_docstring() {
        let sym_meta = persistent_list_map! {
            "cat" => 1,
            "dog" => 2
        };
        let a = sym!("a").with_meta(sym_meta);
        let result = Value::DefMacro.apply_to_persistent_list(
            &Rc::new(Environment::new_main_environment()),
            &Rc::new(list!(a "Docstring" 1)),
        );

        let final_sym_meta = result.unwrap().as_protocol::<protocols::IMeta>().meta();

        assert_eq!(
            Value::I32(1),
            *final_sym_meta.get(&Keyword::intern("cat").to_rc_value())
        );
        assert_eq!(
            Value::I32(2),
            *final_sym_meta.get(&Keyword::intern("dog").to_rc_value())
        );
        assert_eq!(
            Value::String("Docstring".to_string()),
            *final_sym_meta.get(&Keyword::intern("doc").to_rc_value())
        );
    }
}
