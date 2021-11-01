use core::panic;
use std::{io::Cursor, str::Utf8Error};

use regex::Regex;

use crate::token::{judge_token, Token, TokenType::*};

#[derive(Debug)]
pub struct Lexer {
    pub input: Cursor<Vec<u8>>,
    // pub ch: u8,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: Cursor::new(input.as_bytes().to_vec()),
        }
    }

    pub fn next_token(&mut self) -> Result<Token, Utf8Error> {
        loop {
            match get_cuurent_value(&self.input).map(|i| ch_byte_to_str(i)) {
                Some(Ok(w)) if &w == " " || &w == "\n" => {
                    self.read_char();
                }
                Some(Ok(v)) => {
                    let token = self.get_token(&v);
                    self.read_char();
                    return token;
                }
                Some(Err(e)) => {
                    return Err(e);
                }
                None => {
                    return ch_byte_to_str(0).map(|literal| Token {
                        token_type: EOF,
                        literal,
                    })
                }
            }
        }
    }

    fn read_char(&mut self) {
        self.input.set_position(self.input.position() + 1);
    }

    fn get_token(&mut self, literal_ref: &str) -> Result<Token, Utf8Error> {
        let literal = literal_ref.to_string();
        match literal_ref {
            "=" => Ok(Token {
                token_type: ASSIGN,
                literal,
            }),
            ";" => Ok(Token {
                token_type: SEMICOLON,
                literal,
            }),
            "(" => Ok(Token {
                token_type: LPAREN,
                literal,
            }),
            ")" => Ok(Token {
                token_type: RPAREN,
                literal,
            }),
            "," => Ok(Token {
                token_type: COMMA,
                literal,
            }),
            "+" => Ok(Token {
                token_type: PLUS,
                literal,
            }),
            "{" => Ok(Token {
                token_type: LBRACE,
                literal,
            }),
            "}" => Ok(Token {
                token_type: RBRACE,
                literal,
            }),
            _ => {
                if is_letter(literal_ref) || is_number(literal_ref) {
                    self.read_identifier()
                } else {
                    if literal_ref == " " || literal_ref == "\n" {
                        panic!("white spaceはスキップしてからこの関数を呼び出してください。");
                    } else {
                        Ok(Token {
                            token_type: ILLEGAL,
                            literal,
                        })
                    }
                }
            }
        }
    }

    fn read_identifier(&mut self) -> Result<Token, Utf8Error> {
        let current_position = self.input.position() as usize;

        loop {
            if let Some(current_value) = get_cuurent_value(&self.input) {
                match ch_byte_to_str(current_value) {
                    Err(err) => return Err(err),
                    Ok(item) => {
                        if !is_letter(&item) && !is_number(&item) {
                            self.input.set_position(self.input.position() - 1);
                            break;
                        }
                        self.read_char();
                    }
                }
            } else {
                return std::str::from_utf8(&self.input.get_ref()[current_position..]).map(|s| {
                    Token {
                        token_type: if is_number(s) {
                            INT
                        } else {
                            if let Some(keyword) = judge_token(s) {
                                KeyWord(keyword)
                            } else {
                                IDENT
                            }
                        },
                        literal: s.to_string(),
                    }
                });
            }
        }

        std::str::from_utf8(
            &self.input.get_ref()[current_position..self.input.position() as usize + 1],
        )
        .map(|s| Token {
            token_type: if is_number(s) {
                INT
            } else {
                if let Some(keyword) = judge_token(s) {
                    KeyWord(keyword)
                } else {
                    IDENT
                }
            },
            literal: s.to_string(),
        })
    }
}

fn get_cuurent_value(item: &Cursor<Vec<u8>>) -> Option<u8> {
    if item.position() as usize >= item.get_ref().len() {
        None
    } else {
        Some(item.get_ref()[item.position() as usize])
    }
}

fn ch_byte_to_str(ch: u8) -> Result<String, Utf8Error> {
    std::str::from_utf8(&[ch]).map(|i| i.to_string())
}

fn is_letter(ch: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z_]{1}$");
    match re {
        Ok(regex) => regex.is_match(ch),
        Err(_) => false,
    }
}

fn is_number(ch: &str) -> bool {
    let re = Regex::new(r"\d");
    match re {
        Ok(regex) => regex.is_match(ch),
        Err(_) => false,
    }
}

#[test]
fn sandbox() {
    println!("{:?}", is_number("1"));
    println!("{:?}", is_number("12"));
    println!("{:?}", is_number("a"));

    // println!("{:?}", 1)
}

#[test]
fn test_1() {
    use crate::token::{
        KeyWord::*,
        Token,
        TokenType::{self, *},
    };
    let input = "str=+()test{},;let";
    let mut l = Lexer::new(input);
    let mut tokens: Vec<Token> = vec![];
    for _n in 0..l.input.get_ref().len() + 1 {
        let token = l.next_token().unwrap();
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
            IDENT,
            ASSIGN,
            PLUS,
            LPAREN,
            RPAREN,
            IDENT,
            LBRACE,
            RBRACE,
            COMMA,
            SEMICOLON,
            KeyWord(LET),
            EOF
        ]
    );
}

#[test]
fn test_2() {
    use crate::token::{
        KeyWord::*,
        Token,
        TokenType::{self, *},
    };
    let input = std::fs::read_to_string("files/first.txt").unwrap();
    let mut l = Lexer::new(&input);
    let mut tokens: Vec<Token> = vec![];
    for _n in 0..l.input.get_ref().len() + 1 {
        let token = l.next_token().unwrap();
        println!("{:?}", token);
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
            KeyWord(LET),
            IDENT,
            ASSIGN,
            INT,
            SEMICOLON,
            KeyWord(LET),
            IDENT,
            ASSIGN,
            INT,
            SEMICOLON,
            KeyWord(LET),
            IDENT,
            ASSIGN,
            KeyWord(FUNCTION),
            LPAREN,
            IDENT,
            COMMA,
            IDENT,
            RPAREN,
            LBRACE,
            IDENT,
            PLUS,
            IDENT,
            SEMICOLON,
            RBRACE,
            SEMICOLON,
            KeyWord(LET),
            IDENT,
            ASSIGN,
            IDENT,
            LPAREN,
            IDENT,
            COMMA,
            IDENT,
            RPAREN,
            SEMICOLON,
            EOF
        ]
    );
}
