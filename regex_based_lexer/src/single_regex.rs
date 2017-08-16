use super::{Item, Token, Span};

use regex::Regex;

static RULES: [(Item, &str); 10] = [(Item::Plus, r"\+"),
                                    (Item::Minus, r"\-"),
                                    (Item::Multiply, r"\*"),
                                    (Item::Divide, r"/"),
                                    (Item::Equal, r"="),
                                    (Item::LBracket, r"\("),
                                    (Item::RBracket, r"\)"),
                                    (Item::Integer, r"\d+"),
                                    (Item::Ident, r"\w+"),
                                    (Item::Quote, "^\"[^\n]*?\"")];

lazy_static! {
    static ref NON_TOKEN: Regex = Regex::new(r"^(\s|\n)+").unwrap();
    static ref RE: Regex = {
        let re_str =
            RULES.iter()
                .fold(String::new(), |acc, &(ref rule, ref re)| {
                    format!("{}(?P<{:?}>{})|", acc, rule, re)
                });
        Regex::new(&format!("^({}(?P<Comment>//.+$))", re_str)).unwrap()
    };
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

            // Skip to token
            if let Some(mat) = NON_TOKEN.find(buf) {
                pointer += mat.end() - mat.start();
            } else {
                let cap = RE.captures(buf).unwrap();
                if let Some(mat) = cap.name("Comment") {
                    pointer += mat.end() - mat.start();
                } else {
                    for &(ref rule, _) in RULES.iter() {
                        if let Some(mat) = cap.name(&format!("{:?}", rule)) {
                            let token_len = mat.end() - mat.start();
                            tokens.push(Token(rule.clone(),
                                              &self.src[pointer..pointer + token_len],
                                              Span(pointer, pointer + token_len)));
                            pointer += token_len;
                            break;
                        }
                    }
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
