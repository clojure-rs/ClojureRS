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

// input and output
pub(crate) mod system_newline;
pub use self::system_newline::*;
pub(crate) mod flush_stdout;
pub use self::flush_stdout::*;
pub(crate) mod print_string;
pub use self::print_string::*;
pub(crate) mod string_print;
pub use self::string_print::*;
pub(crate) mod read_line;
pub use self::read_line::*;


// other
pub(crate) mod slurp;
pub use self::slurp::*;

pub(crate) mod load_file;
pub use self::load_file::*;
