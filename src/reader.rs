//! The reader.  The part that reads plain text and parses it into Clojure structures, which are
//! themselves code.
//!
//! Right now there's no sort of data kept track by the reader at any point, so there's no real
//! reader data structure here -- this is just a plain module, a bag of functions.  However,
//! I believe this will change -- especially as, for instance, we define the idea of reader conditionals,
//! or even reader macros,  although the latter will likely be reserved for our interpreter here (but perhaps
//! not;  since this is about being a 'free-er' Clojure, especially since it can't compete with it in raw
//! power, neither speed or ecosystem,  it might be worth it to leave in reader macros.

use nom::{
    branch::alt, bytes::complete::tag, map, sequence::preceded, take_until, terminated,
    Err::Incomplete, IResult, Needed,
};

use crate::maps::MapEntry;
use crate::persistent_list::ToPersistentList;
use crate::persistent_list_map::ToPersistentListMap;
use crate::persistent_vector::ToPersistentVector;
use crate::symbol::Symbol;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use std::io::BufRead;
//
// Note; the difference between ours 'parsers'
//   identifier_parser
//   symbol_parser
//   integer_parser
// And our 'try readers'
//   try_read_i32
//   try_read_string
//   try_read_map
//   try_read_list
//   try_read_vector
//
// Is our parsers are meant to be be nom parsers, and more primitive in that
// they can parse any information that we can later use to create a value::Value
//
// Our 'try readers' are a bit higher level, and are specifically supposed to be returning a valid // value::Value or some sort of failure.
//

/// Returns the first character of a string slice.
///
/// If `input` is not empty, then its first char will be returned. Otherwise,
/// `None` is returned.
///
/// # Panics
///
/// This function will panic if `input` is an empty string slice.
fn first_char(input: &str) -> char {
    input.chars().next().unwrap()
}

/// Same as Haskell cons operator, applied to rust strings.
///
/// Concatenates a `char` at the beginning of a `str`
fn cons_str(head: char, tail: &str) -> String {
    let cap = tail.len() + head.len_utf8();
    let mut ret = String::with_capacity(cap);

    ret.push(head);
    ret.push_str(tail);

    ret
}

/// Returns whether if a character can be in the tail of an identifier.
///
/// An identifier is composed of a head (its first char) and a tail (the other
/// chars).
///
/// A character is an identifier char if it is alphanumeric or if it is one of:
///   - `|`,
///   - `?`,
///   - `<`,
///   - `>`,
///   - `+`,
///   - `-`,
///   - `_`,
///   - `=`,
///   - `^`,
///   - `%`,
///   - `&`,
///   - `$`,
///   - `*`,
///   - `!`,
fn is_identifier_char(chr: char) -> bool {
    chr.is_alphanumeric() || "|?<>+-_=^%&$*!".contains(chr)
}

/// Returns whether if a character can be in the head of an identifier.
///
/// An identifier is composed of a head (its first char) and a tail (the other
/// chars).
///
/// A character is an identifier char if it is alphabetic or if it is one of:
///   - `|`,
///   - `?`,
///   - `<`,
///   - `>`,
///   - `+`,
///   - `-`,
///   - `_`,
///   - `=`,
///   - `^`,
///   - `%`,
///   - `&`,
///   - `$`,
///   - `*`,
///   - `!`,
fn is_non_numeric_identifier_char(chr: char) -> bool {
    chr.is_alphabetic() || "|?<>+-_=^%&$*!".contains(chr)
}

/// Returns true if given character is a minus character
///   - `-`,
fn is_minus_char(chr: char) -> bool {
    chr == '-'
}

/// Parses valid Clojure identifiers
/// Example Successes: ab,  cat,  -12+3, |blah|, <well>
/// Example Failures:  'a,  12b,   ,cat  
pub fn identifier_parser(input: &str) -> IResult<&str, String> {
    named!(identifier_head<&str, char>,
       map!(
           take_while_m_n!(1, 1, is_non_numeric_identifier_char),
           first_char
       )
    );

    named!(identifier_tail<&str, &str>, take_while!(is_identifier_char));

    named!(identifier <&str, String>,
         do_parse!(
             head: identifier_head >>
             rest_input: identifier_tail >>
             (cons_str(head, rest_input))
         )
    );

    identifier(input)
}

/// Parses valid Clojure symbols,  whose name is a valid identifier
pub fn symbol_parser(input: &str) -> IResult<&str, Symbol> {
    identifier_parser(input).map(|(rest_input, name)| (rest_input, Symbol::intern(&name)))
}

/// Parses valid integers
/// Example Successes: 1, 2, 4153,  -12421
///
///
pub fn integer_parser(input: &str) -> IResult<&str, i32> {
    named!(integer_sign<&str, &str>,
       map!(
           opt!(take_while_m_n!(1, 1, is_minus_char)),
           |maybe_minus| maybe_minus.unwrap_or("")
       )
    );
    named!(integer_tail<&str, &str>, take_while1!(|c: char| c.is_digit(10)));
    named!(integer_lexer <&str, String>,
         do_parse!(
             sign: integer_sign >>
             rest_input: integer_tail >>
             (format!("{}{}",sign,rest_input))
         )
    );
    integer_lexer(input).map(|(rest, digits)| (rest, digits.parse().unwrap()))
}
// Currently used to create 'try_readers', which are readers (or
// reader functions, at least) that are basically composable InputType
// -> IResult<InputType,Value> parsers, that our normal read function
// / reader will wrap.
/// Takes a parser, such as one that reads a &str and returns an
/// i32, and creates a new parser that instead returns a valid
/// ClojureRS Value instead
pub fn to_value_parser<I, O: ToValue>(
    parser: impl Fn(I) -> IResult<I, O>,
) -> impl Fn(I) -> IResult<I, Value> {
    move |input: I| parser(input).map(|(rest_input, thing)| (rest_input, thing.to_value()))
}

// @TODO make sure whitespace or 'nothing' is at the end, fail for
// float like numbers
/// Tries to parse &str into Value::I32
/// Expects:
///   Integers
/// Example Successes:
///    1 => Value::I32(1),
///    5 => Value::I32(5),
///    1231415 => Value::I32(1231415)
///    -2 => Value::I32(-2)
/// Example Failures:
///    1.5,  7.1321 , 1423152621625226126431525
pub fn try_read_i32(input: &str) -> IResult<&str, Value> {
    to_value_parser(integer_parser)(input)
}

/// Tries to parse &str into Value::Symbol
/// Example Successes:
///    a                    => Value::Symbol(Symbol { name: "a" })
///    cat-dog              => Value::Symbol(Symbol { name: "cat-dog" })
///    +common-lisp-global+ => Value::Symbol(Symbol { name: "+common-lisp-global+" })
/// Example Failures:
///    12cat,  'quoted,  @at-is-for-references
pub fn try_read_symbol(input: &str) -> IResult<&str, Value> {
    to_value_parser(symbol_parser)(input)
}

// @TODO allow escaped strings
/// Tries to parse &str into Value::String
/// Example Successes:
///    "this is pretty straightforward" => Value::String("this is pretty straightforward")
pub fn try_read_string(input: &str) -> IResult<&str, Value> {
    named!(quotation<&str, &str>, preceded!(consume_clojure_whitespaces, tag!("\"")));

    let (rest_input, _) = quotation(input)?;

    named!(
        string_parser<&str, String>,
        map!(
            terminated!(take_until!("\""), tag("\"")),
            |v| String::from(v)
        )
    );

    to_value_parser(string_parser)(rest_input)
}

// @TODO Perhaps generalize this, or even generalize it as a reader macro
/// Tries to parse &str into Value::PersistentListMap, or some other Value::..Map
/// Example Successes:
///    {:a 1} => Value::PersistentListMap {PersistentListMap { MapEntry { :a, 1} .. ]})
pub fn try_read_map(input: &str) -> IResult<&str, Value> {
    named!(lbracep<&str, &str>, preceded!(consume_clojure_whitespaces, tag!("{")));
    named!(rbracep<&str, &str>, preceded!(consume_clojure_whitespaces, tag!("}")));
    let (map_inner_input, _) = lbracep(input)?;
    let mut map_as_vec: Vec<MapEntry> = Vec::new();
    let mut rest_input = map_inner_input;
    loop {
        let right_brace = rbracep(rest_input);
        if let Ok((after_map_input, _)) = right_brace {
            return Ok((after_map_input, map_as_vec.into_list_map().to_value()));
        }
        let (_rest_input, next_key) = try_read(rest_input)?;
        let (_rest_input, next_val) = try_read(_rest_input)?;
        map_as_vec.push(MapEntry {
            key: Rc::new(next_key),
            val: Rc::new(next_val),
        });
        rest_input = _rest_input;
    }
}

// @TODO use nom functions in place of macro
/// Tries to parse &str into Value::PersistentVector
/// Example Successes:
///    [1 2 3] => Value::PersistentVector(PersistentVector { vals: [Rc(Value::I32(1) ... ]})
///    [1 2 [5 10 15] 3]
///      => Value::PersistentVector(PersistentVector { vals: [Rc(Value::I32(1) .. Rc(Value::PersistentVector..)]})
pub fn try_read_vector(input: &str) -> IResult<&str, Value> {
    named!(lbracketp<&str, &str>, preceded!(consume_clojure_whitespaces, tag!("[")));
    named!(rbracketp<&str, &str>, preceded!(consume_clojure_whitespaces, tag!("]")));
    let (vector_inner_input, _) = lbracketp(input)?;
    let mut vector_as_vec = Vec::new();
    // What's left of our input as we read more of our PersistentVector
    let mut rest_input = vector_inner_input;
    loop {
        // Try parse end of vector
        // If we succeeded,  we can convert our vector of values into a PersistentVector and return our success
        if let Ok((after_vector_input, _)) = rbracketp(rest_input) {
            return Ok((after_vector_input, vector_as_vec.into_vector().to_value()));
        }

        // Otherwise, we need to keep reading until we get that closing bracket letting us know we're finished
        let (_rest_input, form) = try_read(rest_input)?;
        vector_as_vec.push(form.to_rc_value());
        rest_input = _rest_input;
    }
}

pub fn try_read_list(input: &str) -> IResult<&str, Value> {
    named!(lparenp<&str, &str>, preceded!(consume_clojure_whitespaces, tag!("(")));
    named!(rparenp<&str, &str>, preceded!(consume_clojure_whitespaces, tag!(")")));

    let (list_inner_input, _) = lparenp(input)?;
    let mut list_as_vec = Vec::new();
    let mut rest_input = list_inner_input;
    loop {
        if let Ok((after_list_input, _)) = rparenp(rest_input) {
            return Ok((after_list_input, list_as_vec.into_list().to_value()));
        }
        let (_rest_input, form) = try_read(rest_input)?;
        list_as_vec.push(form.to_rc_value());
        rest_input = _rest_input;
    }
}

pub fn try_read(input: &str) -> IResult<&str, Value> {
    preceded(
        consume_clojure_whitespaces,
        alt((
            try_read_map,
            try_read_string,
            try_read_i32,
            try_read_symbol,
            try_read_list,
            try_read_vector,
        )),
    )(input)
}

pub fn debug_try_read(input: &str) -> IResult<&str, Value> {
    let reading = try_read(input);
    match &reading {
        Ok((_, value)) => println!("Reading: {}", value),
        _ => println!("Reading: {:?}", reading),
    };
    reading
}

// This is the high level read function that Clojure RS wraps
pub fn read<R: BufRead>(reader: &mut R) -> Value {
    // This is a buffer that will accumulate if a read requires more
    // text to make sense, such as trying to read (+ 1
    let mut input_buffer = String::new();

    // Ask for a line from the reader, try to read, and if unable (because we need more text),
    // loop over and ask for more lines, accumulating them in input_buffer until we can read
    loop {
        let maybe_line = reader.by_ref().lines().next();
        match maybe_line {
            Some(Err(e)) => return Value::Condition(format!("Reader error: {}", e)),
            Some(Ok(line)) => input_buffer.push_str(&line),
            None => return Value::Condition(format!("Tried to read empty stream; unexpected EOF")),
        }

        let line_read = try_read(&input_buffer);
        match line_read {
            Ok((_, value)) => return value,
            // Continue accumulating more input
            Err(Incomplete(_)) => continue,
            Err(err) => {
                return Value::Condition(format!(
                    "Reader Error: could not read next form; {:?}",
                    err
                ))
            }
        }
    }
}

/// Consumes any whitespace from input, if there is any.
/// Always succeeds.
///
/// A whitespace is either an ASCII whitespace or a comma.
fn consume_clojure_whitespaces(input: &str) -> IResult<&str, ()> {
    named!(parser<&str, &str>, take_while!(is_clojure_whitespace));
    parser(input).map(|(rest, _)| (rest, ()))
}

/// Returns whether if a given character is a whitespace.
///
/// Clojure defines a whitespace as either a comma or an unicode whitespace.
fn is_clojure_whitespace(c: char) -> bool {
    c.is_whitespace() || c == ','
}

#[cfg(test)]
mod tests {

    mod first_char_tests {
        use crate::reader::first_char;

        #[test]
        fn first_char_in_single_char_string() {
            assert_eq!('s', first_char("s"));
        }

        #[test]
        fn first_char_in_multi_char_string() {
            assert_eq!('a', first_char("ab"));
        }

        #[test]
        #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
        fn first_char_in_empty_string_panics() {
            first_char("");
        }
    }

    mod cons_str_tests {
        use crate::reader::cons_str;

        #[test]
        fn concatenates_char_to_str_beginning() {
            assert_eq!("str", cons_str('s', "tr"));
        }
    }

    mod identifier_parser_tests {
        use crate::reader::identifier_parser;

        #[test]
        fn identifier_parser_parses_valid_identifier() {
            assert_eq!(
                Some((" this", String::from("input->output?"))),
                identifier_parser("input->output? this").ok()
            );
        }

        #[test]
        fn identifier_parser_does_not_parse_valid_identifier() {
            assert_eq!(None, identifier_parser("1input->output? this").ok());
        }

        #[test]
        fn identifier_parser_does_not_parse_empty_input() {
            assert_eq!(None, identifier_parser("").ok());
        }
    }

    mod symbol_parser_tests {
        use crate::reader::symbol_parser;
        use crate::symbol::Symbol;

        #[test]
        fn identifier_parser_parses_valid_identifier() {
            assert_eq!(
                Some((
                    " this",
                    Symbol {
                        name: String::from("input->output?")
                    }
                )),
                symbol_parser("input->output? this").ok()
            );
        }

        #[test]
        fn identifier_parser_does_not_parse_valid_identifier() {
            assert_eq!(None, symbol_parser("1input->output? this").ok());
        }

        #[test]
        fn identifier_parser_does_not_parse_empty_input() {
            assert_eq!(None, symbol_parser("").ok());
        }
    }

    mod integer_parser_tests {
        use crate::reader::{debug_try_read, integer_parser};

        #[test]
        fn integer_parser_parses_integer_one() {
            let s = "1 ";
            assert_eq!(Some((" ", 1)), integer_parser(s).ok());
        }

        #[test]
        fn integer_parser_parses_integer_zero() {
            let s = "0 ";
            assert_eq!(Some((" ", 0)), integer_parser(s).ok());
        }

        #[test]
        fn integer_parser_parses_integer_negative_one() {
            let s = "-1 ";
            assert_eq!(Some((" ", -1)), integer_parser(s).ok());
        }

        #[test]
        //#[should_panic(expected = "called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }")]
        fn integer_parser_parses_and_fails() {
            let s = "-1-2 ";
            assert_eq!(Some(("-2 ", -1)), integer_parser(s).ok());
        }
    }

    mod try_read_symbol_tests {
        use crate::reader::try_read_symbol;
        use crate::symbol::Symbol;
        use crate::value::Value;

        #[test]
        fn try_read_minus_as_valid_symbol_test() {
            assert_eq!(
                Value::Symbol(Symbol {
                    name: String::from("-")
                }),
                try_read_symbol("- ").unwrap().1
            );
        }
    }

    mod try_read_tests {
        use crate::maps::MapEntry;
        use crate::persistent_list;
        use crate::persistent_list_map;
        use crate::persistent_vector;
        use crate::reader::try_read;
        use crate::symbol::Symbol;
        use crate::value::Value;
        use crate::value::Value::{PersistentList, PersistentListMap, PersistentVector};
        use std::rc::Rc;

        #[test]
        fn try_read_empty_map_test() {
            assert_eq!(
                PersistentListMap(persistent_list_map::PersistentListMap::Empty),
                try_read("{} ").ok().unwrap().1
            );
        }

        #[test]
        fn try_read_string_test() {
            assert_eq!(
                Value::String(String::from("a string")),
                try_read("\"a string\" ").ok().unwrap().1
            );
        }

        #[test]
        fn try_read_int_test() {
            assert_eq!(Value::I32(1), try_read("1 ").ok().unwrap().1);
        }

        #[test]
        fn try_read_negative_int_test() {
            assert_eq!(Value::I32(-1), try_read("-1 ").ok().unwrap().1);
        }

        #[test]
        fn try_read_negative_int_with_second_dash_test() {
            assert_eq!(Value::I32(-1), try_read("-1-2 ").ok().unwrap().1);
        }

        #[test]
        fn try_read_valid_symbol_test() {
            assert_eq!(
                Value::Symbol(Symbol {
                    name: String::from("my-symbol")
                }),
                try_read("my-symbol ").ok().unwrap().1
            );
        }

        #[test]
        fn try_read_minus_as_valid_symbol_test() {
            assert_eq!(
                Value::Symbol(Symbol {
                    name: String::from("-")
                }),
                try_read("- ").ok().unwrap().1
            );
        }

        #[test]
        fn try_read_minus_prefixed_as_valid_symbol_test() {
            assert_eq!(
                Value::Symbol(Symbol {
                    name: String::from("-prefixed")
                }),
                try_read("-prefixed ").ok().unwrap().1
            );
        }

        #[test]
        fn try_read_empty_list_test() {
            assert_eq!(
                PersistentList(persistent_list::PersistentList::Empty),
                try_read("() ").ok().unwrap().1
            );
        }

        #[test]
        fn try_read_empty_vector_test() {
            assert_eq!(
                PersistentVector(persistent_vector::PersistentVector { vals: [].to_vec() }),
                try_read("[] ").ok().unwrap().1
            );
        }
    }

    mod consume_clojure_whitespaces_tests {
        use crate::reader::consume_clojure_whitespaces;
        #[test]
        fn consume_whitespaces_from_input() {
            let s = ", ,,  ,1, 2, 3, 4 5,,6 ";
            assert_eq!(
                Some(("1, 2, 3, 4 5,,6 ", ())),
                consume_clojure_whitespaces(&s).ok()
            );
        }
        #[test]
        fn consume_whitespaces_from_empty_input() {
            let s = "";
            assert_eq!(None, consume_clojure_whitespaces(&s).ok());
        }
        #[test]
        fn consume_whitespaces_from_input_no_whitespace() {
            let s = "1, 2, 3";
            assert_eq!(Some(("1, 2, 3", ())), consume_clojure_whitespaces(&s).ok());
        }
    }

    mod is_clojure_whitespace_tests {
        use crate::reader::is_clojure_whitespace;
        #[test]
        fn comma_is_clojure_whitespace() {
            assert_eq!(true, is_clojure_whitespace(','));
        }

        #[test]
        fn unicode_whitespace_is_clojure_whitespace() {
            assert_eq!(true, is_clojure_whitespace(' '));
        }

        #[test]
        fn character_is_not_clojure_whitespace() {
            assert_eq!(false, is_clojure_whitespace('a'));
        }
    }
}
