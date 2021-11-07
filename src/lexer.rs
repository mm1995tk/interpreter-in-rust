use crate::token::{get_token_type, Token};
use std::io::Cursor;

#[derive(Debug)]
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
        loop {
            if !self.get_cuurent_value().is_ascii_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn get_cuurent_value(&self) -> u8 {
        if (self.input.position() + 1) as usize >= self.input.get_ref().len() {
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

            if item == 0 {
                break;
            }

            if !(item.is_ascii_alphabetic() || item == b'_' || item.is_ascii_digit()) {
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
            Token::INT
        } else if let Some(keyword) = get_token_type(bytes) {
            keyword
        } else {
            Token::IDENT
        }
    }
}

#[test]
fn test_1() {
    use crate::token::Token::*;
    let input = "let=+(!=)test{},;let";
    let mut l = Lexer::new(input);
    let mut tokens: Vec<Token> = vec![];

    loop {
        let token = l.next_token();
        println!("{:?}", &token);
        if token == Token::EOF {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }

    assert_eq!(
        tokens,
        vec![
            LET, ASSIGN, PLUS, LPAREN, NotEQ, RPAREN, IDENT, LBRACE, RBRACE, COMMA, SEMICOLON, LET,
            EOF
        ]
    );
}

#[test]
fn test_2() {
    use crate::token::Token::*;

    let input = std::fs::read_to_string("files/first.txt").unwrap();
    let mut l = Lexer::new(&input);
    let mut tokens: Vec<Token> = vec![];
    loop {
        let token = l.next_token();
        println!("{:?}", &token);
        if token == Token::EOF {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }

    assert_eq!(
        tokens,
        vec![
            LET, IDENT, ASSIGN, INT, SEMICOLON, LET, IDENT, ASSIGN, INT, SEMICOLON, LET, IDENT,
            ASSIGN, FUNCTION, LPAREN, IDENT, COMMA, IDENT, RPAREN, LBRACE, IDENT, PLUS, IDENT,
            SEMICOLON, RBRACE, SEMICOLON, LET, IDENT, ASSIGN, IDENT, LPAREN, IDENT, COMMA, IDENT,
            RPAREN, SEMICOLON, BANG, MINUS, SLASH, ASTERISK, INT, SEMICOLON, INT, LT, INT, GT, INT,
            SEMICOLON, IF, LPAREN, INT, LT, INT, RPAREN, LBRACE, RETURN, TRUE, SEMICOLON, RBRACE,
            ELSE, LBRACE, RETURN, FALSE, SEMICOLON, RBRACE, INT, EQ, INT, SEMICOLON, INT, NotEQ,
            INT, SEMICOLON, EOF
        ]
    );
}
