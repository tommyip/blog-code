//! Blog post: Hand-written lexer compared to regex-based ones with DFA engine
//!
//! A comparison between hand-written, multi-regex and single-regex lexer.
//! We try to parse a simple made-up language which looks like:
//! ```
//! a = 12
//! b = 352.54
//! result = a + (b * 2)
//! ```

#![feature(test)]

extern crate test;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod hand_written;
mod multi_regex;
mod single_regex;

#[derive(Debug, PartialEq, Clone)]
pub enum Item<'a> {
    Ident(&'a str),
    Integer(i32),
    Quote(&'a str),
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    LBracket,
    RBracket,
}

/// The location and length of the token
/// Example:
///     "apple = banana + car"
/// The identifier "banana" would have a span of
/// Span(8, 6)
#[derive(Debug, PartialEq)]
pub struct Span(usize, usize);

#[derive(Debug, PartialEq)]
pub struct Token<'a>(Item<'a>, Span);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
