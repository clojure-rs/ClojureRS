use crate::value::Value;
use std::rc::Rc;

use crate::environment::Environment;
use crate::error_message;
use crate::ifn::IFn;
use crate::keyword::Keyword;
use crate::persistent_list::{
    PersistentList,
    PersistentList::{Cons, Empty},
    ToPersistentList, ToPersistentListIter,
};
use crate::persistent_list_map::IPersistentMap;
use crate::persistent_vector::{PersistentVector, ToPersistentVectorIter};
use crate::repl::Repl;
use crate::symbol::Symbol;
use crate::type_tag::TypeTag;
use crate::util::IsEven;
use crate::value::{Evaluable, ToValue};

use itertools::Itertools;

use crate::iterable::Iterable;
use crate::protocol::Protocol;
use crate::protocol::ProtocolCastable;

// This module will hold core function and macro primitives that aren't special cases
// (like the quote macro, or let), and can't be implemented in clojure itself

// language core functions
pub(crate) mod eval;
pub use self::eval::*;

// macros
pub(crate) mod do_macro;
pub use self::do_macro::*;

pub(crate) mod ns;
pub use self::ns::*;

// arithmetics
pub(crate) mod _plus_;
pub use self::_plus_::*;

pub(crate) mod _subtract_;
pub use self::_subtract_::*;

pub(crate) mod _divide_;
pub use self::_divide_::*;

pub(crate) mod _multiply_;
pub use self::_multiply_::*;

pub(crate) mod rand;
pub use self::rand::*;

pub(crate) mod rand_int;
pub use self::rand_int::*;

// string
pub(crate) mod str;
pub use self::str::*;

// operations on collections
pub(crate) mod nth;
pub use self::nth::*;
pub(crate) mod concat;
pub use self::concat::*;
pub(crate) mod assoc;
pub use self::assoc::*;
pub(crate) mod get;
pub use self::get::*;
pub(crate) mod map;
pub use self::map::*;

// input and outputy
pub(crate) mod print_string;
pub use self::print_string::*;
pub(crate) mod string_print;
pub use self::string_print::*;

// other
pub(crate) mod slurp;
pub use self::slurp::*;

pub(crate) mod load_file;
pub use self::load_file::*;
