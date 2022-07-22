use crate::token::{get_token_type, Token};
use std::io::Cursor;

#[derive(Debug,PartialEq,Clone)]
pub struct Lexer {
    pub input: Cursor<Vec<u8>>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: Cursor::new(input.as_bytes().to_vec()),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = self.get_token();
        self.read_char();

        token
    }

    fn read_char(&mut self) {
        self.input.set_position(self.input.position() + 1);
    }

    fn skip_whitespace(&mut self) {
        while self.get_cuurent_value().is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn get_cuurent_value(&self) -> u8 {
        if self.input.position() as usize >= self.input.get_ref().len() {
            0
        } else {
            self.input.get_ref()[self.input.position() as usize]
        }
    }

    fn peek_char(&self) -> u8 {
        if self.input.position() as usize + 1 >= self.input.get_ref().len() {
            0
        } else {
            self.input.get_ref()[self.input.position() as usize + 1]
        }
    }

    fn judge_two_cahr_token(&mut self) -> Token {
        let cur_ch = self.get_cuurent_value();
        let next_char = self.peek_char();

        if next_char == b'=' {
            self.read_char();
            if cur_ch == b'=' {
                Token::EQ
            } else {
                Token::NotEQ
            }
        } else if cur_ch == b'=' {
            Token::ASSIGN
        } else {
            Token::BANG
        }
    }

    fn get_token(&mut self) -> Token {
        let cur_ch = self.get_cuurent_value();

        let maybe_token_type = get_token_type(&[cur_ch]);

        if cur_ch == b'=' || cur_ch == b'!' {
            return self.judge_two_cahr_token();
        }

        if let Some(token_type) = maybe_token_type {
            token_type
        } else if cur_ch.is_ascii_alphanumeric() || cur_ch == b'_' {
            self.read_identifier()
        } else {
            Token::ILLEGAL
        }
    }

    fn cursor_identifier(&mut self) {
        loop {
            let item = self.get_cuurent_value();

            if !(item.is_ascii_alphabetic() || item == b'_' || item.is_ascii_digit()) || item == 0 {
                self.input.set_position(self.input.position() - 1);
                break;
            }
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> Token {
        let current_position = self.input.position() as usize;
        self.cursor_identifier();
        let bytes = &self.input.get_ref()[current_position..self.input.position() as usize + 1];
        let is_number = bytes.iter().filter(|x| !x.is_ascii_digit()).count() == 0;

        if is_number {
            Token::INT(bytes.to_vec())
        } else if let Some(keyword) = get_token_type(bytes) {
            keyword
        } else {
            Token::IDENT(bytes.to_vec())
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{
        lexer::Lexer,
        token::Token::{self, *},
    };

    fn str_to_vec(s: &str) -> Vec<u8> {
        s.as_bytes().to_vec()
    }

    fn exec(input: &str) -> Vec<Token> {
        let mut l = Lexer::new(input);
        let mut tokens: Vec<Token> = vec![];

        loop {
            let token = l.next_token();
            println!("{}", &token);
            if token == Token::EOF {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        tokens
    }

    #[test]
    fn test_0() {
        let input = "let";
        let tokens: Vec<Token> = exec(input);

        assert_eq!(tokens, vec![LET, EOF]);
    }
    #[test]
    fn test_1() {
        let input = "let=+(!=)test{},;let";
        let tokens: Vec<Token> = exec(input);

        assert_eq!(
            tokens,
            vec![
                LET,
                ASSIGN,
                PLUS,
                LPAREN,
                NotEQ,
                RPAREN,
                IDENT(str_to_vec("test")),
                LBRACE,
                RBRACE,
                COMMA,
                SEMICOLON,
                LET,
                EOF
            ]
        );
    }

    #[test]
    fn test_2() {
        use crate::token::Token::*;

        let input = std::fs::read_to_string("files/first.txt").unwrap();
        let tokens = exec(&input);

        assert_eq!(
            tokens,
            vec![
                LET,
                IDENT(str_to_vec("five")),
                ASSIGN,
                INT(str_to_vec("5")),
                SEMICOLON,
                LET,
                IDENT(str_to_vec("ten")),
                ASSIGN,
                INT(str_to_vec("10")),
                SEMICOLON,
                LET,
                IDENT(str_to_vec("add")),
                ASSIGN,
                FUNCTION,
                LPAREN,
                IDENT(str_to_vec("x")),
                COMMA,
                IDENT(str_to_vec("y")),
                RPAREN,
                LBRACE,
                IDENT(str_to_vec("x")),
                PLUS,
                IDENT(str_to_vec("y")),
                SEMICOLON,
                RBRACE,
                SEMICOLON,
                LET,
                IDENT(str_to_vec("result")),
                ASSIGN,
                IDENT(str_to_vec("add")),
                LPAREN,
                IDENT(str_to_vec("five")),
                COMMA,
                IDENT(str_to_vec("ten")),
                RPAREN,
                SEMICOLON,
                BANG,
                MINUS,
                SLASH,
                ASTERISK,
                INT(str_to_vec("5")),
                SEMICOLON,
                INT(str_to_vec("5")),
                LT,
                INT(str_to_vec("10")),
                GT,
                INT(str_to_vec("5")),
                SEMICOLON,
                IF,
                LPAREN,
                INT(str_to_vec("5")),
                LT,
                INT(str_to_vec("10")),
                RPAREN,
                LBRACE,
                RETURN,
                TRUE,
                SEMICOLON,
                RBRACE,
                ELSE,
                LBRACE,
                RETURN,
                FALSE,
                SEMICOLON,
                RBRACE,
                INT(str_to_vec("10")),
                EQ,
                INT(str_to_vec("10")),
                SEMICOLON,
                INT(str_to_vec("10")),
                NotEQ,
                INT(str_to_vec("9")),
                SEMICOLON,
                EOF
            ]
        );
    }
}
