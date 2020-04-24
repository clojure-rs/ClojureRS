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
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::multispace0,
    combinator::map_res,
    error::convert_error,
    map,
    sequence::{preceded, terminated},
    take_until, terminated, IResult,
};

use crate::maps::MapEntry;
use crate::persistent_list::ToPersistentList;
use crate::persistent_list_map::{PersistentListMap, ToPersistentListMap};
use crate::persistent_vector::ToPersistentVector;
use crate::symbol::Symbol;
use crate::value::{ToValue, Value};
use std::{iter::FromIterator, rc::Rc};

use std::fs::File;

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

    named!(identifier_ <&str, String>,
         do_parse!(
             head: identifier_head >>
             rest_input: identifier_tail >>
             (cons_str(head, rest_input))
         )
    );

    identifier_(input)
}

/// Parses valid Clojure symbols,  whose name is a valid identifier
pub fn symbol_parser(input: &str) -> IResult<&str, Symbol> {
    identifier_parser(input).map(|(rest_input, name)| (rest_input, Symbol::intern(&name)))
}

// @TODO add negatives
/// Parses valid integers
/// Example Successes: 1, 2, 4153,  -12421
pub fn integer(input: &str) -> IResult<&str, i32> {
    named!(integer_lexer<&str, &str>, take_while1!(|c: char| c.is_digit(10)));

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
/// Example Failures:
///    1.5,  7.1321 , 1423152621625226126431525
pub fn try_read_i32(input: &str) -> IResult<&str, Value> {
    to_value_parser(integer)(input)
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

    to_value_parser(string_parser)(input)
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
            try_read_symbol,
            try_read_i32,
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

/// Consumes one or more whitespaces from the input.
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
