use super::{Op, Token, TokenPos};

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
        Lexer { src, src_vec, src_len }
    }

    fn lex(&self) -> Vec<Token> {
        let mut result = vec![];
        let mut pointer = 0;

        while pointer < self.src_len {
            match self.src_vec[pointer] {
                ' ' | '\n' | '\t' | '\r' => {}
                '/' => {
                    if self.src_vec[pointer + 1] == '/' {
                        pointer += 2;
                        while pointer < self.src_len &&
                              self.src_vec[pointer] != '\n'
                        {
                            pointer += 1;
                        }
                    } else {
                        result.push(Token(Op::Divide, TokenPos(pointer, 1)));
                    }
                }
                c => {
                    if let Some(operator) = Lexer::_get_operator(c) {
                        result.push(Token(operator, TokenPos(pointer, 1)));
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

    fn _get_operator(character: char) -> Option<Op<'a>> {
        match character {
            '+' => Some(Op::Plus),
            '-' => Some(Op::Minus),
            '*' => Some(Op::Multiply),
            '/' => Some(Op::Divide),
            '=' => Some(Op::Equal),
            '(' => Some(Op::LBracket),
            ')' => Some(Op::RBracket),
            _   => None
        }
    }

    fn _get_ident(&self, pointer: &mut usize) -> Token {
        let mut end_pos = *pointer + 1;
        while end_pos < self.src_len &&
              self.src_vec[end_pos].is_alphabetic()
        {
            end_pos += 1;
        }
        let token = Token(Op::Ident(&self.src[*pointer..end_pos]),
                          TokenPos(*pointer, end_pos - *pointer));
        *pointer = end_pos - 1;
        token
    }

    fn _get_integer(&self, pointer: &mut usize) -> Token {
        let mut end_pos = *pointer;
        while end_pos < self.src_len &&
              self.src_vec[end_pos].is_numeric()
        {
            end_pos += 1;
        }

        let token = Token(
            Op::Integer(self.src[*pointer..end_pos].parse::<i32>().unwrap()),
            TokenPos(*pointer, end_pos - *pointer));

        *pointer = end_pos - 1;
        token
    }

    /// Get strings delimited by qoutes
    fn _get_quote(&self, pointer: &mut usize) -> Token
    {
        *pointer += 1;
        let span_pos = *pointer;
        loop {
            match self.src_vec[*pointer] {
                '"' => return Token(Op::Quote(&self.src[span_pos..*pointer]),
                                    TokenPos(span_pos, *pointer - span_pos)),
                '\n' => panic!("Unclosed string"),
                _ => *pointer += 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_src() {
        let src = "person = (head + body) * \"mind\" + 42";
        let lexer = Lexer::new(src);

        assert_eq!(lexer.lex(), vec![
            Token(Op::Ident("person"), TokenPos(0, 6)),
            Token(Op::Equal,           TokenPos(7, 1)),
            Token(Op::LBracket,        TokenPos(9, 1)),
            Token(Op::Ident("head"),   TokenPos(10, 4)),
            Token(Op::Plus,            TokenPos(15, 1)),
            Token(Op::Ident("body"),   TokenPos(17, 4)),
            Token(Op::RBracket,        TokenPos(21, 1)),
            Token(Op::Multiply,        TokenPos(23, 1)),
            Token(Op::Quote("mind"),   TokenPos(26, 4)),
            Token(Op::Plus,            TokenPos(32, 1)),
            Token(Op::Integer(42),     TokenPos(34, 2)),
        ]);
    }
}
