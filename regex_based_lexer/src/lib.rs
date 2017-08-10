// Blog post: WIP title (Hand-written lexer compared to regex-based ones with DFA engine)
// Comparison between hand-written, single-regex and multi-regex lexer.
// We try to parse a simple made-up language which looks like:
// ```
// a = 12
// b = 352.54
// result = a + ( b * 2 )
// ```

mod hand_written;
mod single_regex;
mod multi_regex;

#[derive(Debug, PartialEq)]
pub enum Op<'a> {
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

// The location and length of the token
// Example:
//     "apple = banana + car"
// The identifier "banana" would have a TokenPos of
// TokenPos(8, 6)
#[derive(Debug, PartialEq)]
pub struct TokenPos(usize, usize);

#[derive(Debug, PartialEq)]
pub struct Token<'a>(Op<'a>, TokenPos);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
