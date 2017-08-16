use super::{Item, Token, Span};

use regex::Regex;

lazy_static! {
    // Notice the use of ^ at the beginning of all regex, this is
    // due to the Rust regex library treating every regex with an
    // implicit `.*?` at the beginning and end.
    static ref RULES: [(Item, Regex); 10] = [
        (Item::Plus,     Regex::new(r"^\+").unwrap()),
        (Item::Minus,    Regex::new(r"^\-").unwrap()),
        (Item::Multiply, Regex::new(r"^\*").unwrap()),
        (Item::Divide,   Regex::new(r"^/").unwrap()),
        (Item::Equal,    Regex::new(r"^=").unwrap()),
        (Item::LBracket, Regex::new(r"^\(").unwrap()),
        (Item::RBracket, Regex::new(r"^\)").unwrap()),
        (Item::Integer,  Regex::new(r"^\d+").unwrap()),
        (Item::Ident,    Regex::new(r"^\w+").unwrap()),
        // Match everything but a newline character delimited by quotes
        // non greedily.
        (Item::Quote,    Regex::new("^\"[^\n]*?\"").unwrap()),
    ];

    // (\s|\n|\r|\t) : token separaters
    // //.+$         : comment
    static ref NON_TOKEN_RE: Regex = Regex::new(r"^((\s|\n|\r|\t)+|//.+$)").unwrap();
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
            let buf = &self.src[pointer..];

            if let Some(mat) = NON_TOKEN_RE.find(buf) {
                pointer += mat.end() - mat.start();
                continue;
            }

            let token_len: usize;
            // Test if the next character is an operator
            for &(ref rule, ref re) in RULES.iter() {
                if let Some(mat) = re.find(buf) {
                    token_len = mat.end() - mat.start();
                    tokens.push(Token(rule.clone(),
                                      &self.src[pointer..pointer + token_len],
                                      Span(pointer, pointer + token_len)));
                    pointer += token_len;
                    break;
                }
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

        assert_eq!(lexer.lex(),
                   vec![Token(Item::Ident, "person", Span(0, 6)),
                        Token(Item::Equal, "=", Span(7, 8)),
                        Token(Item::LBracket, "(", Span(9, 10)),
                        Token(Item::Ident, "head", Span(10, 14)),
                        Token(Item::Plus, "+", Span(15, 16)),
                        Token(Item::Ident, "body", Span(17, 21)),
                        Token(Item::RBracket, ")", Span(21, 22)),
                        Token(Item::Multiply, "*", Span(23, 24)),
                        Token(Item::Quote, "\"mind\"", Span(25, 31)),
                        Token(Item::Plus, "+", Span(32, 33)),
                        Token(Item::Integer, "42", Span(34, 36))]);
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
        b.iter(|| { let _ = lexer.lex(); });
    }
}
