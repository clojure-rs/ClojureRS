pub(crate) mod slurp;

use crate::value::Value;
use std::rc::Rc;

use crate::environment::Environment;
use crate::ifn::IFn;
use crate::persistent_list::{
    PersistentList,
    PersistentList::{Cons, Empty},
    ToPersistentList, ToPersistentListIter,
};
use crate::persistent_list_map::IPersistentListMap;
use crate::persistent_vector::{PersistentVector, ToPersistentVectorIter};
use crate::symbol::Symbol;
use crate::type_tag::TypeTag;
use crate::value::{Evaluable, ToValue};

use itertools::Itertools;
use crate::error_message;

use crate::util::IsEven;

pub(crate) mod _subtract_;
pub use self::_subtract_::*;

pub(crate) mod _divide_;
pub use self::_divide_::*;

pub(crate) mod _multiply_;
pub use self::_multiply_::*;
//
// This module will hold core function and macro primitives that aren't special cases
// (like the quote macro, or let), and can't be implemented in clojure itself

// language core functions
pub(crate) mod eval;

// macros
pub(crate) mod do_macro;

// arithmetics
pub(crate) mod _plus_;
pub use self::_plus_::*;

// string
pub(crate) mod str;
pub use self::str::*;

// operations on collections
pub(crate) mod nth;
pub(crate) mod concat;
pub(crate) mod assoc;

// input and output
pub(crate) mod print_string;
pub(crate) mod string_pring;

// other
pub(crate) mod slurp;
