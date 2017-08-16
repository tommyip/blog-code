use super::{Item, Token, Span};

use regex::Regex;

static RULES: [(&str, &str); 11] = [
    ("Plus", r"\+"),
    ("Minus", r"\-"),
    ("Multiply", r"\*"),
    ("Divide", r"/"),
    ("Equal", r"="),
    ("L_Bracket", r"\("),
    ("R_Bracket", r"\)"),
    ("Non_Token", r"((\s|\n|\r|\t)+|//.+$)"),
    ("Integer", r"\d+"),
    ("Ident", r"\w+"),
    ("Quote", "^\"[^\n]*?\"")
];

lazy_static! {
    static ref RE: Regex = {
        let mut re_str =
            RULES.iter()
                .fold(String::new(), |acc, &(rule, re)| {
                    format!("{}(?P<{}>{})|", acc, rule, re)
                });
        re_str.pop(); // Remove extra "|"
        Regex::new(&format!("^({})", re_str)).unwrap()
    };
}

#[derive(Debug, PartialEq)]
struct Lexer<'a> {
    src: &'a str,
    src_len: usize
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

            let cap = RE.captures(buf).unwrap();
            if let Some(mat) = cap.name("Non_Token") {
                pointer += mat.end() - mat.start();
            } else if cap.name("Plus").is_some() {
                Lexer::_process_operator(&mut tokens, Item::Plus, &mut pointer);
            } else if cap.name("Minus").is_some() {
                Lexer::_process_operator(&mut tokens, Item::Minus, &mut pointer);
            } else if cap.name("Multiply").is_some() {
                Lexer::_process_operator(&mut tokens, Item::Multiply, &mut pointer);
            } else if cap.name("Divide").is_some() {
                Lexer::_process_operator(&mut tokens, Item::Divide, &mut pointer);
            } else if cap.name("Equal").is_some() {
                Lexer::_process_operator(&mut tokens, Item::Equal, &mut pointer);
            } else if cap.name("L_Bracket").is_some() {
                Lexer::_process_operator(&mut tokens, Item::LBracket, &mut pointer);
            } else if cap.name("R_Bracket").is_some() {
                Lexer::_process_operator(&mut tokens, Item::RBracket, &mut pointer);
            } else {
                let token_len;
                if let Some(mat) = cap.name("Integer") {
                    token_len = mat.end() - mat.start();
                    tokens.push(Token(Item::Integer((&self.src[pointer..pointer + token_len])
                                            .parse::<i32>()
                                            .unwrap()), Span(pointer, token_len)));
                } else if let Some(mat) = cap.name("Ident") {
                    token_len = mat.end() - mat.start();
                    tokens.push(Token(Item::Ident(&self.src[pointer..pointer + token_len]),
                                      Span(pointer, token_len)));
                } else if let Some(mat) = cap.name("Quote") {
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

    fn _process_operator(tokens: &mut Vec<Token<'a>>, op: Item<'a>, pointer: &mut usize) {
        (*tokens).push(Token(op, Span(*pointer, 1)));
        *pointer += 1;
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
