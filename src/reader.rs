//! The reader.  The part that reads plain text and parses it into Clojure structures, which are
//! themselves code.
//!
//! Right now there's no sort of data kept track by the reader at any point, so there's no real
//! reader data structure here -- this is just a plain module, a bag of functions.  However,
//! I believe this will change -- especially as, for instance, we define the idea of reader conditionals,
//! or even reader macros,  although the latter will likely be reserved for our interpreter here (but perhaps
//! not;  since this is about being a 'free-er' Clojure, especially since it can't compete with it in raw
//! power, neither speed or ecosystem,  it might be worth it to leave in reader macros.

use nom::combinator::verify;
use nom::{
    branch::alt, bytes::complete::tag, combinator::opt, map, sequence::preceded, take_until,
    Err::Incomplete, IResult,
};

use crate::error_message;
use crate::keyword::Keyword;
use crate::maps::MapEntry;
use crate::persistent_list::ToPersistentList;
use crate::persistent_list_map::{PersistentListMap, ToPersistentListMap, ToPersistentListMapIter};
use crate::persistent_vector::ToPersistentVector;
use crate::protocol::Protocol;
use crate::protocol::ProtocolCastable;
use crate::protocols;
use crate::symbol::Symbol;
use crate::traits::IObj;
use crate::value::{ToValue, Value};
use crate::traits::IMeta;
use std::io::BufRead;
use std::rc::Rc;
//
// Note; the difference between ours 'parsers'
//   identifier_parser
//   symbol_parser
//   integer_parser
// And our 'try readers'
//   try_read_i32
//   try_read_char
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

////////////////////////////////////////////////////////////////////////////////////////////////////
//
//     Utils
//
////////////////////////////////////////////////////////////////////////////////////////////////////

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
////////////////////////////////////////////////////////////////////////////////////////////////////
//     End Utils
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
//
//     Predicates
//
////////////////////////////////////////////////////////////////////////////////////////////////////

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
fn is_identifier_char(ch: char) -> bool {
    ch.is_alphanumeric() || "|?<>+-_=^%&$*!.".contains(ch)
}

/// Returns true if a character is an acceptable (non numeric) identifier char
///
/// An identifier is either a non numeric identifier char, followed by any number
/// of identifier chars,  or is a '/' and nothing else.
///
/// A separate function will be used to detect if an identifier is possibly just '/'
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
fn is_non_numeric_identifier_char(ch: char) -> bool {
    ch.is_alphabetic() || "|?<>+-_=^%&$*!.".contains(ch)
}

/// Returns true if a character is an acceptable (non numeric) identifier char, or '/'
///
/// An identifier is either a non numeric identifier char, followed by any number
/// of identifier chars,  or is a '/' and nothing else.
///
/// The reason we check if this is *either* a non numeric identifier char, or a '/',
/// is because we will want to use it to parse either
///    1.a normal identifier
///    2.'/',
///    3. something like '/blah'
/// And then, if we have '/blah', we will proactively make the read fail
///
/// We need to explicitly look for this '/blah' case is otherwise, if we just check for 1 and 2,
/// then in the case where someone types in '/blah' it will count as two valid separate reads --
/// first the symbol '/' and then the symbol 'blah'.
///
/// This function passes if the char is alphabetic, a '/', or one of:
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
fn is_non_numeric_identifier_char_or_slash(ch: char) -> bool {
    ch == '/' || is_non_numeric_identifier_char(ch)
}

/// Returns true if given character is a minus character
///   - `-`,
fn is_minus_char(ch: char) -> bool {
    ch == '-'
}

/// Returns true if given character is a period character
///   - `-`,
fn is_period_char(ch: char) -> bool {
    ch == '.'
}

/// Returns whether if a given character is a whitespace.
///
/// Clojure defines a whitespace as either a comma or an unicode whitespace.
fn is_clojure_whitespace(c: char) -> bool {
    c.is_whitespace() || c == ','
}
////////////////////////////////////////////////////////////////////////////////////////////////////
//     End predicates
////////////////////////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////////////////////////
//
//     Parsers
//
//////////////////////////////////////////////////////////////////////////////////////////////////////

/// Consumes any whitespace from input, if there is any.
/// Always succeeds.
///
/// A whitespace is either an ASCII whitespace or a comma.
fn consume_clojure_whitespaces_parser(input: &str) -> IResult<&str, ()> {
    named!(comment_parser<&str,&str>, delimited!(tag(";"),take_until!("\n"),tag("\n")));

    named!(whitespace_parser<&str,()>,
           value!((),
               many0!(alt!(comment_parser |
                           take_while1!(is_clojure_whitespace))))
    );

    named!(no_whitespace_parser<&str,()>, value!((),tag!("")));

    // @TODO rename / check that all parsers are consistent?
    named!(parser<&str,()>,
           // Because 'whitespace_parser' loops, we cannot include the case where there's no whitespace at all in
           // its definition -- nom wouldn't allow it, as it would loop forever consuming no whitespace
           // So instead, we eat up all the whitespace first, and then use the no_whitespace_parser as our sort-of
           // base-case after
           alt!(whitespace_parser | no_whitespace_parser)
    );
    parser(input)
}

// This parser is made with nom's function combinator, rather than macros,
// both because nom is soft deprecating the macros (and we too should move away from them),
// but also because in doing so this is how we parse 'complete' text, rather than streamed text.
//
// The difference?  When streamed, typing 1234 and hitting enter will be feeding
// "1234/n" to our reader, and it will take this to mean *more* of this number may
// be coming. So to finish typing a number, you have to eventually go out of your way
// to hit " ", else you'll get
//
// user> 1234
// 456
// 1232 ...
//
// This function, unlike the other subparsers of a parser, is made external to the function
// just because type inference doesn't play nice with defining this inline

fn identifier_tail(input: &str) -> IResult<&str, &str> {
    nom::bytes::complete::take_while(is_identifier_char)(input)
}

/// Parses valid Clojure identifiers
/// Example Successes: ab,  cat,  -12+3, |blah|, <well>, / (edge case)
/// Example Failures:  'a,  12b,   ,cat  , /ab
pub fn identifier_parser(input: &str) -> IResult<&str, String> {
    // We will try to parse either a valid identifier, *or* the invalid identifier
    // '/slashwithmorecharacters'
    // Because if we do get the '/blah', we want to know and actively fail, otherwise '/blah'
    // will just count as two valid reads; one for '/' and one for 'blah'
    // So, we call these parsers 'maybe_valid_identifier_..',  as they are also trying to catch
    // this one invalid case 
    named!(maybe_invalid_identifier_head_parser<&str, char>,
       map!(
           take_while_m_n!(1, 1, is_non_numeric_identifier_char_or_slash),
           first_char
       )
    );

    // identifier_tail<&str,&str> defined above so it can be a 'completion' parser instead of a
    // 'streaming' parser -- look into nom's documentation for more info 

    named!(maybe_invalid_identifier_parser <&str, String>,
         do_parse!(
             head: maybe_invalid_identifier_head_parser >>
             rest_input: identifier_tail >>
             (cons_str(head, &rest_input))
         )
    );

    named!(valid_identifier_parser <&str,String>,
           verify!(maybe_invalid_identifier_parser,|identifier| {
               first_char(&identifier) != '/' ||
               identifier == "/"
           }));

    valid_identifier_parser(input)
}

/// Parses valid Clojure symbol
/// Example Successes: a , b , |ab123|
///                    namespace.subnamespace/a    cat/b   a.b.c/|ab123|
pub fn symbol_parser(input: &str) -> IResult<&str, Symbol> {
    named!(namespace_parser <&str,String>,
           do_parse!(
               ns: identifier_parser >>
               complete!(tag!("/")) >>
               (ns)));

    let (rest_input, ns) = opt(namespace_parser)(input)?;
    let (rest_input, name) = identifier_parser(rest_input)?;
    match ns {
        Some(ns) => Ok((rest_input, Symbol::intern_with_ns(&ns, &name))),
        None => Ok((rest_input, Symbol::intern(&name))),
    }
}

pub fn string_parser(input: &str) -> IResult<&str, String> {
    // Convert escaped characters like \n to their actual counterparts -- like an actual newline
    named!(escaped_string_parser<&str, String>, escaped_transform!(take_till1!(|ch| { ch == '\\' || ch == '\"'}), '\\', alt!(
        tag!("t")   => { |_| "\t"   } |
        tag!("b")   => { |_| "\x08" } |
        tag!("n")   => { |_| "\n"   } |
        tag!("r")   => { |_| "\r"   } |
        tag!("f")   => { |_| "\x0C" } |
        tag!("'")   => { |_| "'"    } |
        tag!("\"")  => { |_| "\""   } |
        tag!("\\")  => { |_| "\\"   }
    )));

    named!(empty_string_parser <&str, String>, map!(tag!("\"\""),|_| String::from("")));

    named!(
        string_parser<&str, String>,
        alt!(
            delimited!(tag("\""),escaped_string_parser, tag("\"")) |
            // Base case; empty string
            empty_string_parser)
    );

    string_parser(input)
}

// Helper function to integer_parser for same reason as
// identifier_tail. See comment above said function for explanation

fn integer_tail(input: &str) -> IResult<&str, &str> {
    nom::bytes::complete::take_while1(|c: char| c.is_digit(10))(input)
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
    // integer_tail<&str,&str> above function

    named!(integer_parser <&str, String>,
         do_parse!(
             sign: integer_sign >>
             rest_input: integer_tail >>
             (format!("{}{}",sign,rest_input))
         )
    );
    integer_parser(input).map(|(rest, digits)| (rest, digits.parse().unwrap()))
}

/// Parses valid doubles
/// Example Successes: -1.0, 0.023, 1234.3223423
///
///
pub fn double_parser(input: &str) -> IResult<&str, f64> {
    named!(decimal_point<&str, &str>, take_while_m_n!(1, 1, is_period_char));

    named!(double_parser <&str, String>,
         do_parse!(
             integer: integer_parser >> //integer_part >>
             point: complete!(decimal_point) >>
             decimal: integer_tail >> //decimal_part >>
             (format!("{}{}{}",integer, point, decimal))
         )
    );
    double_parser(input).map(|(rest, digits)| (rest, digits.parse().unwrap()))
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
////////////////////////////////////////////////////////////////////////////////////////////////////
//    End Parsers
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
//
//    Try-Readers
//
////////////////////////////////////////////////////////////////////////////////////////////////////

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

/// Tries to parse &str into Value::Boolean
/// Expects:
///     Booleans
/// Example success:
///     true => Value::Boolean(true)
///     false => Value::Boolean(false)
pub fn try_read_bool(input: &str) -> IResult<&str, Value> {
    named!(bool_parser<&str,&str>, alt!( tag!("true") | tag!("false")));
    let (rest_input, bool) = bool_parser(input)?;
    Ok((rest_input, Value::Boolean(bool.parse().unwrap())))
}

/// Tries to parse &str into Value::double
///
pub fn try_read_f64(input: &str) -> IResult<&str, Value> {
    to_value_parser(double_parser)(input)
}

// Perhaps generalize this into reader macros
/// Tries to parse &str into Value::Keyword
/// Example Successes:
///    :a                    => Value::Keyword(Keyword { sym: Symbol { name: "a" })
///    :cat-dog              => Value::Keyword(Keyword { sym: Symbol { name: "cat-dog" })
/// Example Failures:
///    :12 :'a
pub fn try_read_keyword(input: &str) -> IResult<&str, Value> {
    named!(keyword_colon<&str, &str>, preceded!(consume_clojure_whitespaces_parser, tag!(":")));

    let (rest_input, _) = keyword_colon(input)?;
    let (rest_input, symbol) = symbol_parser(rest_input)?;

    let keyword_value = Keyword { sym: symbol }.to_value();
    Ok((rest_input, keyword_value))
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

/// Tries to parse a &str that says 'nil' into Value::Nil
/// Example Successes:
///    nil => Value::Nil
pub fn try_read_nil(input: &str) -> IResult<&str, Value> {
    let (rest_input, _) = verify(identifier_parser, |ident: &str| ident == "nil")(input)?;
    Ok((rest_input, Value::Nil))
}

/// Tries to parse &str into Value::Char
/// Example Successes:
///    "\newline" => Value::Char("\n")
/// Example Failures:
///
pub fn try_read_char(input: &str) -> IResult<&str, Value> {
    named!(backslash<&str, &str>, preceded!(consume_clojure_whitespaces_parser, tag!("\\")));

    fn str_to_unicode(s: &str) -> char {
        u32::from_str_radix(s, 16)
            .ok()
            .and_then(std::char::from_u32)
            .unwrap()
    }

    named!(unicode < &str, char>,  alt!(
        preceded!(
            tag!("u"),
            alt!(
                map!(take_while_m_n!(4,4, |c :char| c.is_digit(16)), str_to_unicode)
            )
        )
    ));

    named!(special_escapes < &str, char>,  complete!( alt!(
        tag!("newline")   => { |_|  '\n'} |
        tag!("space")     => { |_|  ' ' } |
        tag!("tab")       => { |_|  '\t'} |
        //tag!("formfeed")  => { |_|  '\f'} |
        //tag!("backspace") => { |_|  '\b'} |
        tag!("return")    => { |_|  '\r' } )));

    named!(normal_char < &str, char>,
           // accept anything after \
           map!(take_while_m_n!(1,1,|_| true), first_char));

    named!(char_parser<&str,char>,
           alt!(unicode | special_escapes | normal_char));

    let (rest_input, _) = backslash(input)?;

    let (rest_input, char_value) = char_parser(rest_input)?;

    Ok((rest_input, Value::Char(char_value)))
}

// @TODO allow escaped strings
/// Tries to parse &str into Value::String
/// Example Successes:
///    "this is pretty straightforward" => Value::String("this is pretty straightforward")
pub fn try_read_string(input: &str) -> IResult<&str, Value> {
    to_value_parser(string_parser)(input)
}

pub fn try_read_pattern(input: &str) -> IResult<&str, Value> {
    named!(hash_parser<&str, &str>, preceded!(consume_clojure_whitespaces_parser, tag!("#")));

    let (rest_input, _) = hash_parser(input)?;
    let (rest_input, regex_string) = string_parser(rest_input)?;

    // If an error is thrown,  this will be coerced into a condition
    let regex = regex::Regex::new(regex_string.as_str()).to_value();
    Ok((rest_input, regex))
}
// Reads the #
pub fn try_read_var(input: &str) -> IResult<&str, Value> {
    named!(var_parser<&str, &str>, preceded!(consume_clojure_whitespaces_parser, tag!("#'")));

    let (rest_input, _) = var_parser(input)?;
    let (rest_input, val) = try_read(rest_input)?;
    // #'x just expands to (var x), just like 'x is just a shorthand for (quote x)
    // So here we return (var val)
    Ok((rest_input, list_val!(sym!("var") val)))
}

// @TODO Perhaps generalize this, or even generalize it as a reader macro
/// Tries to parse &str into Value::PersistentListMap, or some other Value::..Map
/// Example Successes:
///    {:a 1} => Value::PersistentListMap {PersistentListMap { MapEntry { :a, 1} .. ]})
pub fn try_read_map(input: &str) -> IResult<&str, Value> {
    named!(lbracep<&str, &str>, preceded!(consume_clojure_whitespaces_parser, tag!("{")));
    named!(rbracep<&str, &str>, preceded!(consume_clojure_whitespaces_parser, tag!("}")));
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

pub fn try_read_meta(input: &str) -> IResult<&str, Value> {
    named!(meta_start<&str, &str>, preceded!(consume_clojure_whitespaces_parser, tag!("^")));
    let (rest_input, _) = meta_start(input)?;

    let (rest_input, meta_value) = try_read(rest_input)?;
    let mut meta = PersistentListMap::Empty;
    match &meta_value {
        Value::Symbol(symbol) => {
            // @TODO Note; do NOT hardcode this, make some global for TAG_KEY, like Clojure does
            meta = persistent_list_map! {"tag" => symbol};
        }
        Value::Keyword(keyword) => {
            meta = persistent_list_map!(MapEntry {
                key: meta_value.to_rc_value(),
                val: true.to_rc_value()
            });
        }
        Value::String(string) => {
            // @TODO Note; do NOT hardcode this, make some global for TAG_KEY, like Clojure does
            meta = persistent_list_map! {"tag" => string};
        }
        Value::PersistentListMap(plist_map) => {
            meta = plist_map.clone();
            // Then we're already set
        }
        _ => {
            // @TODO check instanceof IPersistentMap here instead
            // @TODO Clojure has basically this one off error here, but another thing we wish to do
            //       is write clear errors
            return Ok((
                rest_input,
                error_message::custom(
                    "When trying to read meta: metadata must be Symbol, Keyword, String, or Map",
                ),
            ));
        }
    }
    let (rest_input, iobj_value) = try_read(rest_input)?;

    // Extra clone, implement these functions for plain Values
    if let Some(iobj_value) = iobj_value
        .to_rc_value()
        .try_as_protocol::<protocols::IObj>()
    {
        // @TODO get actual line and column info
        let line = 1;
        let column = 1;
        // @TODO merge the meta iobj_value *already* has
        // @TODO define some better macros and / or functions for map handling 
        meta = conj!(
            meta,
            map_entry!("line",line),
            map_entry!("column",column)
        );
        meta = merge!(meta,iobj_value.meta());
        Ok((rest_input,iobj_value.with_meta(meta).unwrap().to_value()))
    }
    else {
        Ok((rest_input,error_message::custom("In meta reader: metadata can only be applied to types who are an instance of IMeta")))
    }
}
// @TODO use nom functions in place of macro
/// Tries to parse &str into Value::PersistentVector
/// Example Successes:
///    [1 2 3] => Value::PersistentVector(PersistentVector { vals: [Rc(Value::I32(1) ... ]})
///    [1 2 [5 10 15] 3]
///      => Value::PersistentVector(PersistentVector { vals: [Rc(Value::I32(1) .. Rc(Value::PersistentVector..)]})
pub fn try_read_vector(input: &str) -> IResult<&str, Value> {
    named!(lbracketp<&str, &str>, preceded!(consume_clojure_whitespaces_parser, tag!("[")));
    named!(rbracketp<&str, &str>, preceded!(consume_clojure_whitespaces_parser, tag!("]")));
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
    named!(lparenp<&str, &str>, preceded!(consume_clojure_whitespaces_parser, tag!("(")));
    named!(rparenp<&str, &str>, preceded!(consume_clojure_whitespaces_parser, tag!(")")));

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

pub fn try_read_quoted(input: &str) -> IResult<&str, Value> {
    named!(quote<&str, &str>, preceded!(consume_clojure_whitespaces_parser, tag!("'")));

    let (form, _) = quote(input)?;

    let (rest_input, quoted_form_value) = try_read(form)?;

    // (quote value)
    Ok((rest_input, list_val!(sym!("quote") quoted_form_value)))
}

pub fn try_read(input: &str) -> IResult<&str, Value> {
    preceded(
        consume_clojure_whitespaces_parser,
        alt((
            try_read_meta,
            try_read_quoted,
            try_read_nil,
            try_read_map,
            try_read_char,
            try_read_string,
            try_read_f64,
            try_read_i32,
            try_read_bool,
            try_read_nil,
            try_read_symbol,
            try_read_keyword,
            try_read_list,
            try_read_vector,
            try_read_pattern,
            try_read_var,
        )),
    )(input)
}
////////////////////////////////////////////////////////////////////////////////////////////////////
//      End Try-Readers
////////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
//
//      Readers
//
///////////////////////////////////////////////////////////////////////////////////////////////////

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
            // `lines` does not include \n,  but \n is part of the whitespace given to the reader
            // (and is important for reading comments) so we will push a newline as well
            Some(Ok(line)) => {
                input_buffer.push_str(&line);
                input_buffer.push_str("\n");
            }
            None => {
                return Value::Condition(String::from("Tried to read empty stream; unexpected EOF"))
            }
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
                Some((" this", Symbol::intern("input->output?"))),
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

        #[test]
        fn symbol_parser_normal_symbol_test() {
            assert_eq!(Symbol::intern("a"), symbol_parser("a ").ok().unwrap().1);
        }
        #[test]
        fn symbol_parser_namespace_qualified_symbol_test() {
            assert_eq!(
                Symbol::intern_with_ns("clojure.core", "a"),
                symbol_parser("clojure.core/a ").ok().unwrap().1
            );
        }
    }

    mod double_parser_tests {
        use crate::reader::double_parser;

        #[test]
        fn double_parser_parses_negative_one() {
            let s = "-1.2 ";
            assert_eq!(Some((" ", -1.2)), double_parser(s).ok());
        }

        #[test]
        fn double_parser_parses_one() {
            let s = "1.12 ";
            assert_eq!(Some((" ", 1.12)), double_parser(s).ok());
        }

        #[test]
        fn double_parser_parses_integer_zero() {
            let s = "0.0001 ";
            assert_eq!(Some((" ", 0.0001)), double_parser(s).ok());
        }
    }

    mod integer_parser_tests {
        use crate::reader::integer_parser;

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

    mod try_read_bool_tests {
        use crate::reader::try_read_bool;
        use crate::value::Value;

        #[test]
        fn try_read_boolean_true_test() {
            assert_eq!(Value::Boolean(true), try_read_bool("true ").ok().unwrap().1);
        }

        #[test]
        fn try_read_boolean_false_test() {
            assert_eq!(
                Value::Boolean(false),
                try_read_bool("false ").ok().unwrap().1
            );
        }
    }

    mod try_read_nil_tests {
        use crate::reader::try_read_nil;
        use crate::value::Value;

        #[test]
        fn try_read_nil_test() {
            assert_eq!(Value::Nil, try_read_nil("nil ").ok().unwrap().1);
        }
    }

    mod try_read_char_tests {
        use crate::reader::try_read_char;
        use crate::value::Value;

        //    #[test]
        //    fn try_read_char_test() {
        //        assert_eq!(Value::Char("\\f"), try_read_char("\\formfeed"))
        //    }

        #[test]
        fn try_read_char_space() {
            assert_eq!(Value::Char(' '), try_read_char("\\space").ok().unwrap().1);
        }

        #[test]
        fn try_read_char_return() {
            assert_eq!(Value::Char('\r'), try_read_char("\\return").ok().unwrap().1);
        }

        #[test]
        fn try_read_char_hashtag() {
            assert_eq!(Value::Char('#'), try_read_char("\\#").ok().unwrap().1);
        }
        #[test]
        fn try_read_char_n() {
            assert_eq!(Value::Char('n'), try_read_char("\\n").ok().unwrap().1);
        }
        #[test]
        fn try_read_char_f() {
            assert_eq!(Value::Char('r'), try_read_char("\\r").ok().unwrap().1);
        }
        #[test]
        fn try_read_unicode() {
            assert_eq!(Value::Char('张'), try_read_char("\\u5F20").ok().unwrap().1);
        }
        #[test]
        fn try_read_char_fail() {
            assert!(try_read_char("d").is_err());
        }
    }

    mod try_read_symbol_tests {
        use crate::reader::try_read_symbol;
        use crate::symbol::Symbol;
        use crate::value::Value;

        #[test]
        fn try_read_minus_as_valid_symbol_test() {
            assert_eq!(
                Value::Symbol(Symbol::intern("-")),
                try_read_symbol("- ").unwrap().1
            );
        }
    }

    //    mod try_read_char_tests {
    //        use crate::reader::try_read_char;
    //        use crate::value::Value;
    //
    //        #[test]
    //        fn try_read_char_test() {
    //            assert_eq!(Value::Char('f'), try_read_character("\\f"))
    //        }
    //
    //        #[test]
    //        fn try_read_newline_test() {
    //            assert_eq!(Value::Char('\n'), try_read_character("\newline"))
    //        }
    //    }

    mod try_read_tests {
        use crate::keyword::Keyword;
        use crate::persistent_list;
        use crate::persistent_list_map;
        use crate::persistent_list_map::IPersistentMap;
        use crate::persistent_vector;
        use crate::reader::try_read;
        use crate::symbol::Symbol;
        use crate::value::Value::{PersistentList, PersistentListMap, PersistentVector};
        use crate::value::{ToValue, Value};

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
        fn try_read_string_empty() {
            assert_eq!(
                Value::String(String::from("")),
                try_read("\"\"").ok().unwrap().1
            );
        }

        #[test]
        fn try_read_string_escaped_quotes() {
            assert_eq!(
                Value::String(String::from("\" \" c c caf \" fadsg")),
                try_read(r#""\" \" c c caf \" fadsg""#).ok().unwrap().1
            );
        }

        #[test]
        fn try_read_string_newlines() {
            assert_eq!(
                Value::String(String::from("\n fadsg \n")),
                try_read(r#""\n fadsg \n""#).ok().unwrap().1
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
                Value::Symbol(Symbol::intern("my-symbol")),
                try_read("my-symbol ").ok().unwrap().1
            );
        }

        #[test]
        fn try_read_minus_as_valid_symbol_test() {
            assert_eq!(
                Value::Symbol(Symbol::intern("-")),
                try_read("- ").ok().unwrap().1
            );
        }

        #[test]
        fn try_read_minus_prefixed_as_valid_symbol_test() {
            assert_eq!(
                Value::Symbol(Symbol::intern("-prefixed")),
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

        #[test]
        fn try_read_bool_true_test() {
            assert_eq!(Value::Boolean(true), try_read("true ").ok().unwrap().1)
        }

        #[test]
        fn try_read_bool_false_test() {
            assert_eq!(Value::Boolean(false), try_read("false ").ok().unwrap().1)
        }
        #[test]
        fn try_read_meta_symbol() {
            let with_meta = "^cat a";
            match try_read(with_meta).ok().unwrap().1 {
                Value::Symbol(symbol) => {
                    assert!(symbol
                        .meta()
                        .contains_key(&Keyword::intern("tag").to_rc_value()));
                    assert_eq!(
                        Symbol::intern("cat").to_value(),
                        *symbol.meta().get(&Keyword::intern("tag").to_rc_value())
                    );
                }
                _ => panic!("try_read_meta \"^cat a\" should return a symbol"),
            }
        }
        #[test]
        fn try_read_meta_string() {
            let with_meta = "^\"cat\" a";
            match try_read(with_meta).ok().unwrap().1 {
                Value::Symbol(symbol) => {
                    assert_eq!(String::from("a"), symbol.name);
                    assert!(symbol
                        .meta()
                        .contains_key(&Keyword::intern("tag").to_rc_value()));
                    assert_eq!(
                        "cat".to_value(),
                        *symbol.meta().get(&Keyword::intern("tag").to_rc_value())
                    );
                }
                _ => panic!("try_read_meta '^\"cat\" a' should return a symbol"),
            }
        }
        #[test]
        fn try_read_meta_persistent_list_map() {
            let with_meta = "^{:cat 1 :dog 2} a";
            match try_read(with_meta).ok().unwrap().1 {
                Value::Symbol(symbol) => {
                    assert!(symbol
                        .meta()
                        .contains_key(&Keyword::intern("cat").to_rc_value()));
                    assert_eq!(
                        Value::I32(1),
                        *symbol.meta().get(&Keyword::intern("cat").to_rc_value())
                    );
                    assert!(symbol
                        .meta()
                        .contains_key(&Keyword::intern("dog").to_rc_value()));
                    assert_eq!(
                        Value::I32(2),
                        *symbol.meta().get(&Keyword::intern("dog").to_rc_value())
                    );
                    assert!(!symbol
                        .meta()
                        .contains_key(&Keyword::intern("chicken").to_rc_value()));
                }
                _ => panic!("try_read_meta \"^{:cat 1 :dog 2} a\" should return a symbol"),
            }
        }
        #[test]
        fn try_read_multiple_meta_keyword() {
            let with_meta = "^:cat ^:dog a";
            match try_read(with_meta).ok().unwrap().1 {
                Value::Symbol(symbol) => {
                    assert!(symbol.meta().contains_key(&Keyword::intern("cat").to_rc_value()));
                    assert!(symbol.meta().contains_key(&Keyword::intern("dog").to_rc_value()));
                },
                _ => panic!("try_read_meta \"^:cat a\" should return a symbol")
            }
        }
        #[test]
        fn try_read_meta_keyword() {
            let with_meta = "^:cat a";
            match try_read(with_meta).ok().unwrap().1 {
                Value::Symbol(symbol) => {
                    assert!(symbol
                        .meta()
                        .contains_key(&Keyword::intern("cat").to_rc_value()));
                }
                _ => panic!("try_read_meta \"^:cat a\" should return a symbol"),
            }
        }
        #[test]
        fn try_read_forward_slash_test() {
            assert_eq!(
                Value::Symbol(Symbol::intern(&"/")),
                try_read("/ ").ok().unwrap().1
            );
        }
        #[test]
        fn try_read_forward_slash_with_letters_and_fails_test() {
            assert!(try_read("/ab ").ok().is_none());
        }

        #[test]
        fn try_read_forward_slash_keyword_test() {
            assert_eq!(
                Value::Keyword(Keyword::intern(&"/")),
                try_read(":/ ").ok().unwrap().1
            );
        }
        
        #[test]
        fn try_read_forward_slash_keyword_with_letters_and_fails_test() {
            assert!(try_read(":/ab ").ok().is_none());
        }

        #[test]
        fn try_read_forward_slash_keyword_with_ns_test() {
            assert_eq!(
                Value::Keyword(Keyword::intern_with_ns("core", "/")),
                try_read(":core// ").ok().unwrap().1
            );
        }
        
        #[test]
        fn try_read_forward_slash_keyword_with_ns_with_letters_and_fails_test() {
            assert!(try_read(":core//ab ").ok().is_none());
        }
    }

    mod regex_tests {
        use crate::reader::try_read;
        use crate::value::Value;

        #[test]
        fn try_read_simple_regex_pattern_test() {
            assert_eq!(
                Value::Pattern(regex::Regex::new("a").unwrap()),
                try_read(r###"#"a" "###).ok().unwrap().1
            );
        }

        #[test]
        fn try_read_regex_pattern_test() {
            assert_eq!(
                Value::Pattern(regex::Regex::new("hello").unwrap()),
                try_read("#\"hello\" ").ok().unwrap().1
            );
        }

        #[test]
        fn try_read_regex_pattern_escaped_quote_test() {
            assert_eq!(
                Value::Pattern(regex::Regex::new("h\"e\"l\"l\"o\"").unwrap()),
                try_read(r#"#"h\"e\"l\"l\"o\"" something"#).ok().unwrap().1
            );
        }

        #[test]
        fn try_read_regex_pattern_escaped_quote_prefixed_by_whitespace_test() {
            assert_eq!(
                Value::Pattern(regex::Regex::new("h\"e\"l\"l \"o").unwrap()),
                try_read(r#"#"h\"e\"l\"l \"o""#).ok().unwrap().1
            );
        }

        #[test]
        fn try_read_regex_pattern_escaped_quote_suffixed_by_whitespace_test() {
            assert_eq!(
                Value::Pattern(regex::Regex::new("h\"e\"l\" l \"o").unwrap()),
                try_read(r#"#"h\"e\"l\" l \"o" something"#).ok().unwrap().1
            );
        }
    }

    mod consume_clojure_whitespaces_tests {
        use crate::reader::consume_clojure_whitespaces_parser;
        #[test]
        fn consume_whitespaces_from_input() {
            let s = ", ,,  ,1, 2, 3, 4 5,,6 ";
            assert_eq!(
                Some(("1, 2, 3, 4 5,,6 ", ())),
                consume_clojure_whitespaces_parser(&s).ok()
            );
        }
        #[test]
        fn consume_whitespaces_from_empty_input() {
            let s = "";
            assert_eq!(None, consume_clojure_whitespaces_parser(&s).ok());
        }
        #[test]
        fn consume_whitespaces_from_input_no_whitespace() {
            let s = "1, 2, 3";
            assert_eq!(
                Some(("1, 2, 3", ())),
                consume_clojure_whitespaces_parser(&s).ok()
            );
        }

        #[test]
        fn consume_whitespaces_with_comments_then_no_whitespace() {
            let s = ", ,,  \n; Line starts as comment\n  ; Line does not start as comment\n1, 2, 3, 4 5,,6 ";
            assert_eq!(
                Some(("1, 2, 3, 4 5,,6 ", ())),
                consume_clojure_whitespaces_parser(&s).ok()
            );
        }

        #[test]
        fn consume_whitespaces_with_comments_then_whitespace() {
            let s = ", ,,  \n; Line starts as comment\n  ; Line does not start as comment\n,   1, 2, 3, 4 5,,6 ";
            assert_eq!(
                Some(("1, 2, 3, 4 5,,6 ", ())),
                consume_clojure_whitespaces_parser(&s).ok()
            );
        }
        #[test]
        fn consume_whitespaces_with_comments() {
            let mut s = "; Line starts as comment\n\n,   1, 2, 3, 4 5,,6 ";
            assert_eq!(
                Some(("1, 2, 3, 4 5,,6 ", ())),
                consume_clojure_whitespaces_parser(&s).ok()
            );

            s = "; Line starts as comment\n\n1, 2, 3, 4 5,,6 ";
            assert_eq!(
                Some(("1, 2, 3, 4 5,,6 ", ())),
                consume_clojure_whitespaces_parser(&s).ok()
            );
        }

        #[test]
        fn consume_whitespaces_multiline() {
            let s = " , , ,\n    \n\n\n,   1, 2, 3, 4 5,,6 ";
            assert_eq!(
                Some(("1, 2, 3, 4 5,,6 ", ())),
                consume_clojure_whitespaces_parser(&s).ok()
            );
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
