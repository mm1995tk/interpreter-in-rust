use std::{io::Cursor, str::Utf8Error};

use regex::Regex;

use crate::token::{get_token_type, Token, TokenType::*};

#[derive(Debug)]
pub struct Lexer {
    pub input: Cursor<Vec<u8>>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: Cursor::new(input.as_bytes().to_vec()),
        }
    }

    pub fn next_token(&mut self) -> Result<Token, Utf8Error> {
        self.skip_whitespace();

        match get_cuurent_value(&self.input).map(|i| ch_byte_to_str(i)) {
            Some(Err(e)) => Err(e),
            Some(Ok(v)) => {
                let token = self.get_token(&v);
                self.read_char();
                token
            }
            None => ch_byte_to_str(0).map(|literal| Token {
                token_type: EOF,
                literal,
            }),
        }
    }

    fn read_char(&mut self) {
        self.input.set_position(self.input.position() + 1);
    }

    fn skip_whitespace(&mut self) {
        loop {
            match get_cuurent_value(&self.input).map(|i| ch_byte_to_str(i)) {
                Some(Ok(ref w)) if w == " " || w == "\n" || w == "\t" || w == "\r" => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn peek_char(&self) -> Result<Option<String>, Utf8Error> {
        if self.input.position() as usize >= self.input.get_ref().len() {
            Ok(None)
        } else {
            let byte = self.input.get_ref()[self.input.position() as usize + 1];
            std::str::from_utf8(&[byte]).map(|i| Some(i.to_string()))
        }
    }

    fn judge_two_cahr_token(&mut self, literal: &str) -> Result<Token, Utf8Error> {
        let next_char = self.peek_char();

        next_char.map(|next| match next.as_deref() {
            None => Token {
                token_type: if literal == "=" { ASSIGN } else { BANG },
                literal: literal.to_string(),
            },
            Some(value) => match value {
                "=" => {
                    self.read_char();
                    Token {
                        token_type: if literal == "=" { EQ } else { NotEQ },
                        literal: if literal == "=" {
                            format!("==")
                        } else {
                            format!("!=")
                        },
                    }
                }
                _ => Token {
                    token_type: if literal == "=" { ASSIGN } else { BANG },
                    literal: literal.to_string(),
                },
            },
        })
    }

    fn get_token(&mut self, literal_ref: &str) -> Result<Token, Utf8Error> {
        let literal = literal_ref.to_string();
        let maybe_token_type = get_token_type(&literal);

        if literal_ref == "=" || literal_ref == "!" {
            return self.judge_two_cahr_token(literal_ref);
        }

        if let Some(token_type) = maybe_token_type {
            Ok(Token {
                token_type,
                literal,
            })
        } else {
            if is_letter(literal_ref) || is_number(literal_ref) {
                self.read_identifier()
            } else {
                Ok(Token {
                    token_type: ILLEGAL,
                    literal,
                })
            }
        }
    }

    fn cursor_identifier(&mut self) {
        loop {
            match get_cuurent_value(&self.input).map(|current_value| ch_byte_to_str(current_value))
            {
                Some(Ok(item)) => {
                    if !is_letter(&item) && !is_number(&item) {
                        self.input.set_position(self.input.position() - 1);
                        break;
                    }
                    self.read_char();
                }
                _ => {
                    self.input.set_position(self.input.position() - 1);
                    break;
                }
            }
        }
    }

    fn read_identifier(&mut self) -> Result<Token, Utf8Error> {
        let current_position = self.input.position() as usize;
        self.cursor_identifier();
        let bytes = &self.input.get_ref()[current_position..self.input.position() as usize + 1];

        std::str::from_utf8(bytes).map(|s| {
            let token_type = if is_number(s) {
                INT
            } else {
                if let Some(keyword) = get_token_type(s) {
                    keyword
                } else {
                    IDENT
                }
            };

            Token {
                token_type,
                literal: s.to_string(),
            }
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
    assert_eq!(true, true);
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
