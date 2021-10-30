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
fn test_1() {
    let input = "=+(){},;";
    let mut l = Lexer::new(input);
    let mut tokens: Vec<Token> = vec![];
    for _n in 0..l.input.len() + 1 {
        let token = l.next_token();
        println!("{:?}", &token);
        tokens.push(token);
        // assert_eq!(l.ch_byte_to_str(), &input[n..n+1]);
    }
    let token_types: Vec<TokenType> = tokens.into_iter().map(|token| token.token_type).collect();
    assert_eq!(
        token_types,
        vec![ASSIGN, PLUS, LPAREN, RPAREN, LBRACE, RBRACE, COMMA, SEMICOLON, EOF]
    );
}

#[test]
fn test_2() {
    let input = std::fs::read_to_string("files/first.txt").unwrap();
    let mut l = Lexer::new(&input);
    let mut tokens: Vec<Token> = vec![];
    for _n in 0..l.input.len() + 1 {
        let token = l.next_token();
        println!("{:?}", &token);
        tokens.push(token);
    }
    let token_types: Vec<TokenType> = tokens.into_iter().map(|token| token.token_type).collect();
    assert_eq!(
        token_types,
        vec![
            LET, IDENT, ASSIGN, INT, SEMICOLON, LET, IDENT, ASSIGN, INT, SEMICOLON, LET, IDENT,
            ASSIGN, FUNCTION, LPAREN, IDENT, COMMA, IDENT, RPAREN, LBRACE, IDENT, PLUS, IDENT,
            SEMICOLON, RBRACE, SEMICOLON, LET, IDENT, ASSIGN, IDENT, LPAREN, IDENT, COMMA, IDENT,
            RPAREN, SEMICOLON, EOF
        ]
    );
}
