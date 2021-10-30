use std::io::Cursor;

use regex::Regex;

use crate::token::{Token, TokenType::*};

#[derive(Debug)]
pub struct Lexer {
    pub input: Cursor<Vec<u8>>,
    // pub ch: u8,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: Cursor::new(input.as_bytes().to_vec()),
            // ch: 0,
            // // position: 0
        }
    }

    pub fn next_token(&mut self) -> Token {
        match get_cuurent_value(&self.input) {
            None => Token {
                token_type: EOF,
                literal: ch_byte_to_str(0),
            },
            Some(current_value) => {
                let token = self.get_token(&ch_byte_to_str(current_value));
                self.read_char();
                token
            }
        }
    }

    fn read_char(&mut self) {
        self.input.set_position(self.input.position() + 1);
    }

    fn get_token(&mut self, literal: &str) -> Token {
        // println!("{}", self.input.position());
        match literal {
            "=" => Token {
                token_type: ASSIGN,
                literal: literal.to_string(),
            },
            ";" => Token {
                token_type: SEMICOLON,
                literal: literal.to_string(),
            },
            "(" => Token {
                token_type: LPAREN,
                literal: literal.to_string(),
            },
            ")" => Token {
                token_type: RPAREN,
                literal: literal.to_string(),
            },
            "," => Token {
                token_type: COMMA,
                literal: literal.to_string(),
            },
            "+" => Token {
                token_type: PLUS,
                literal: literal.to_string(),
            },
            "{" => Token {
                token_type: LBRACE,
                literal: literal.to_string(),
            },
            "}" => Token {
                token_type: RBRACE,
                literal: literal.to_string(),
            },
            _ => {
                if self.input.position() as usize == self.input.get_ref().len() {
                    Token {
                        token_type: EOF,
                        literal: literal.to_string(),
                    };
                }

                if is_letter(literal) {
                    self.read_identifier()
                } else {
                    Token {
                        token_type: ILLEGAL,
                        literal: literal.to_string(),
                    }
                }
            }
        }
    }

    fn read_identifier(&mut self) -> Token {
        let current_position = self.input.position() as usize;

        loop {
            match get_cuurent_value(&self.input) {
                None => {
                    return Token {
                        token_type: IDENT,
                        literal: std::str::from_utf8(&self.input.get_ref()[current_position..])
                            .unwrap()
                            .to_string(),
                    };
                }
                Some(current_value) => {
                    if !is_letter(&ch_byte_to_str(current_value)) {
                        self.input.set_position(self.input.position() - 1);
                        break;
                    }
                    self.read_char();
                }
            }
        }

        Token {
            token_type: IDENT,
            literal: std::str::from_utf8(
                &self.input.get_ref()[current_position..self.input.position() as usize + 1],
            )
            .unwrap()
            .to_string(),
        }
    }
}

fn get_cuurent_value(item: &Cursor<Vec<u8>>) -> Option<u8> {
    if item.position() as usize >= item.get_ref().len() {
        None
    } else {
        Some(item.get_ref()[item.position() as usize])
    }
}

fn ch_byte_to_str(ch: u8) -> String {
    std::str::from_utf8(&[ch]).unwrap().to_string()
}

fn is_letter(ch: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z_]{1}$");
    match re {
        Ok(regex) => regex.is_match(ch),
        Err(_) => false,
    }
}

#[test]
fn sandbox() {
    let x = vec![1, 2, 3];
    println!("{:?}", &x[2..3]);

    // println!("{:?}", 1)
}

#[test]
fn test_1() {
    use crate::token::{
        Token,
        TokenType::{self, *},
    };
    let input = "str=+()test{},;let";
    let mut l = Lexer::new(input);
    let mut tokens: Vec<Token> = vec![];
    for _n in 0..l.input.get_ref().len() + 1 {
        let token = l.next_token();
        println!("{:?}", &token);
        if token.token_type == EOF {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }
    let token_types: Vec<TokenType> = tokens.into_iter().map(|token| token.token_type).collect();
    assert_eq!(
        token_types,
        vec![
            IDENT, ASSIGN, PLUS, LPAREN, RPAREN, IDENT, LBRACE, RBRACE, COMMA, SEMICOLON, IDENT,
            EOF
        ]
    );
}

#[test]
fn test_2() {
    use crate::token::{
        Token,
        TokenType::{self, *},
    };
    let input = std::fs::read_to_string("files/first.txt").unwrap();
    let mut l = Lexer::new(&input);
    let mut tokens: Vec<Token> = vec![];
    for _n in 0..l.input.get_ref().len() + 1 {
        let token = l.next_token();
        if token.token_type == EOF {
            break;
        }
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
