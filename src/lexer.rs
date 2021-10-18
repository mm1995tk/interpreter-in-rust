use std::str::Bytes;

use crate::token::{Token, TokenType::*};

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
    pub fn ch_byte_to_str(&self) -> String {
        std::str::from_utf8(&vec![self.ch]).unwrap().to_string()
    }

    pub fn read_char(&mut self) {
        self.ch = if let Some(v) = self.input.next() {
            v
        } else {
            0
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.read_char();
        let literal = self.ch_byte_to_str();
        let token_type = match &literal as &str {
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
        };
        Token {
            token_type,
            literal,
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
