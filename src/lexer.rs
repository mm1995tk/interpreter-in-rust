use std::str::Bytes;

use crate::token::{
    Token,
    TokenType::{self, *},
};

#[derive(Debug)]
pub struct Lexer<'a> {
    pub input: Bytes<'a>,
    pub ch: u8,
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.bytes(),
            ch: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.read_char();
        let literal = self.ch_byte_to_str();
        let token_type = self.to_token_type(&literal);
        Token {
            token_type,
            literal,
        }
    }

    fn ch_byte_to_str(&self) -> String {
        std::str::from_utf8(&[self.ch]).unwrap().to_string()
    }

    fn read_char(&mut self) {
        self.ch = if let Some(v) = self.input.next() {
            v
        } else {
            0
        }
    }

    fn to_token_type(&self, literal: &str) -> TokenType {
        match literal {
            "=" => ASSIGN,
            ";" => SEMICOLON,
            "(" => LPAREN,
            ")" => RPAREN,
            "," => COMMA,
            "+" => PLUS,
            "{" => LBRACE,
            "}" => RBRACE,
            _ => match self.ch {
                0 => EOF,
                _ => ILLEGAL,
            },
        }
    }
}

#[test]
fn test() {
    let input = "=+(){},;";
    let mut l = Lexer::new(input);
    for _ in 0..l.input.len() {
        l.read_char();
    }
    assert_eq!(l.ch_byte_to_str(), ";");
    l.read_char();
    assert_eq!(l.ch, 0);
}
