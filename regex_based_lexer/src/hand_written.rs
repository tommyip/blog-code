use super::{Item, Token, Span};

#[derive(Debug, PartialEq)]
struct Lexer<'a> {
    src: &'a str,
    src_vec: Vec<char>,
    src_len: usize,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Lexer {
        let src_vec: Vec<char> = src.chars().collect();
        let src_len = src_vec.len();
        Lexer {
            src,
            src_vec,
            src_len,
        }
    }

    fn lex(&self) -> Vec<Token> {
        let mut result = vec![];
        let mut pointer = 0;

        while pointer < self.src_len {
            match self.src_vec[pointer] {
                ' ' | '\n' | '\t' | '\r' => {}
                '/' => {
                    // Item is a line comment, so we skip over everything
                    // and stop after a \n token.
                    if self.src_vec[pointer + 1] == '/' {
                        pointer += 2;
                        while pointer < self.src_len && self.src_vec[pointer] != '\n' {
                            pointer += 1;
                        }
                    } else {
                        result.push(Token(Item::Divide,
                                          &self.src[pointer..pointer + 1],
                                          Span(pointer, pointer + 1)));
                    }
                }
                c => {
                    if let Some(operator) = Lexer::_get_operator(c) {
                        result.push(Token(operator,
                                          &self.src[pointer..pointer + 1],
                                          Span(pointer, pointer + 1)));
                    } else if c.is_alphabetic() {
                        result.push(self._get_ident(&mut pointer));
                    } else if c.is_numeric() {
                        result.push(self._get_integer(&mut pointer));
                    } else if c == '"' {
                        result.push(self._get_quote(&mut pointer));
                    }
                }
            }
            pointer += 1;
        }
        result
    }

    fn _get_operator(character: char) -> Option<Item> {
        match character {
            '+' => Some(Item::Plus),
            '-' => Some(Item::Minus),
            '*' => Some(Item::Multiply),
            '/' => Some(Item::Divide),
            '=' => Some(Item::Equal),
            '(' => Some(Item::LBracket),
            ')' => Some(Item::RBracket),
            _ => None,
        }
    }

    /// A valid identifier only contains alphabets
    fn _get_ident(&self, pointer: &mut usize) -> Token {
        let mut end_pos = *pointer;
        while end_pos < self.src_len && self.src_vec[end_pos].is_alphabetic() {
            end_pos += 1;
        }
        let token = Token(Item::Ident,
                          &self.src[*pointer..end_pos],
                          Span(*pointer, end_pos));
        *pointer = end_pos - 1;
        token
    }

    fn _get_integer(&self, pointer: &mut usize) -> Token {
        let mut end_pos = *pointer;
        while end_pos < self.src_len && self.src_vec[end_pos].is_numeric() {
            end_pos += 1;
        }
        let token = Token(Item::Integer,
                          &self.src[*pointer..end_pos],
                          Span(*pointer, end_pos));
        *pointer = end_pos - 1;
        token
    }

    /// Get strings delimited by quotes
    // "abc"
    fn _get_quote(&self, pointer: &mut usize) -> Token {
        let lo = *pointer;
        *pointer += 1;
        loop {
            match self.src_vec[*pointer] {
                '"' => {
                    *pointer += 1;
                    return Token(Item::Quote,
                                 &self.src[lo..*pointer],
                                 Span(lo, *pointer));
                }
                '\n' => panic!("Unclosed string"),
                _ => *pointer += 1,
            }
        }
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
