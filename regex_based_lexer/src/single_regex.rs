use super::{Item, Token, Span};

use regex::Regex;

lazy_static! {
    // Notice the use of ^ at the beginning of all regex, this is
    // due to the Rust regex library treating every regex with an
    // implicit `.*?` at the beginning and end.
    static ref RULES: [(Item<'static>, Regex); 7] = [
        (Item::Plus,     Regex::new(r"^\+").unwrap()),
        (Item::Minus,    Regex::new(r"^\-").unwrap()),
        (Item::Multiply, Regex::new(r"^\*").unwrap()),
        (Item::Divide,   Regex::new(r"^/").unwrap()),
        (Item::Equal,    Regex::new(r"^=").unwrap()),
        (Item::LBracket, Regex::new(r"^\(").unwrap()),
        (Item::RBracket, Regex::new(r"^\)").unwrap())
    ];

    // (\s|\n|\r|\t) : token separaters
    // //.+$         : comment
    static ref NON_TOKEN_RE: Regex = Regex::new(r"^((\s|\n|\r|\t)+|//.+$)").unwrap();
    static ref INT_RE: Regex = Regex::new(r"^\d+").unwrap();
    static ref IDENT_RE: Regex = Regex::new(r"^\w+").unwrap();
    // Match everything but a newline character delimited by quotes
    // non greedily.
    static ref QUOTE_RE: Regex = Regex::new("^\"[^\n]*?\"").unwrap();
}

#[derive(Debug, PartialEq)]
struct Lexer<'a> {
    src: &'a str,
    src_len: usize,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Lexer {
        let src_len = src.chars().count();
        Lexer { src, src_len }
    }

    fn lex(&self) -> Vec<Token> {
        let mut pointer = 0;
        let mut tokens = vec![];

        while pointer < self.src_len {
            let mut found = false;
            let buf = &self.src[pointer..];

            if let Some(mat) = NON_TOKEN_RE.find(buf) {
                pointer += mat.end() - mat.start();
                continue;
            }

            // Test if the next character is an operator
            for &(ref rule, ref re) in RULES.iter() {
                if re.is_match(buf) {
                    tokens.push(Token(rule.clone(), Span(pointer, 1)));
                    found = true;
                    pointer += 1;
                    break;
                }
            }

            // Next token should either be a integer, identifier or string.
            if !found {
                let token_len;
                if let Some(mat) = INT_RE.find(buf) {
                    token_len = mat.end() - mat.start();
                    tokens.push(Token(Item::Integer((&self.src[pointer..pointer + token_len])
                                            .parse::<i32>()
                                            .unwrap()), Span(pointer, token_len)));

                } else if let Some(mat) = IDENT_RE.find(buf) {
                    token_len = mat.end() - mat.start();
                    tokens.push(Token(Item::Ident(&self.src[pointer..pointer + token_len]),
                                      Span(pointer, token_len)));
                } else if let Some(mat) = QUOTE_RE.find(buf) {
                    token_len = mat.end() - mat.start();
                    tokens.push(Token(Item::Quote(&self.src[pointer + 1..pointer + token_len - 1]),
                                      Span(pointer + 1, token_len - 2)));
                } else {
                    panic!("Unknown token");
                }
                pointer += token_len;
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn simple_src() {
        let src = "person = (head + body) * \"mind\" + 42";
        let lexer = Lexer::new(src);

        assert_eq!(lexer.lex(), vec![
            Token(Item::Ident("person"), Span(0, 6)),
            Token(Item::Equal,           Span(7, 1)),
            Token(Item::LBracket,        Span(9, 1)),
            Token(Item::Ident("head"),   Span(10, 4)),
            Token(Item::Plus,            Span(15, 1)),
            Token(Item::Ident("body"),   Span(17, 4)),
            Token(Item::RBracket,        Span(21, 1)),
            Token(Item::Multiply,        Span(23, 1)),
            Token(Item::Quote("mind"),   Span(26, 4)),
            Token(Item::Plus,            Span(32, 1)),
            Token(Item::Integer(42),     Span(34, 2)),
        ]);
    }

    #[bench]
    fn benchmark(b: &mut Bencher) {
        lazy_static! {
            static ref SRC: String = {
                let mut file = File::open("src_file").expect("");
                let mut content = String::new();
                file.read_to_string(&mut content).expect("");

                content
            };
        }
        let lexer = Lexer::new(&SRC);
        b.iter(|| {
            let _ = lexer.lex();
        });
    }
}
