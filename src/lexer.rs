use crate::token::{Token, TokenType::*};

#[derive(Debug)]
pub struct Lexer<'a> {
    pub input: &'a str,
    pub position: u32,
    pub read_position: u32,
    pub ch: u8,
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        }
    }
    pub fn ch_byte_to_str(&self) -> String {
        std::char::from_u32(self.ch as u32).unwrap().to_string()
    }

    pub fn read_char(&mut self) {
        let lexer_inpu_len = self.input.len() as u32;
        self.ch = if self.read_position >= lexer_inpu_len {
            0
        } else {
            self.input.as_bytes()[self.read_position as usize]
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.read_char();
        let literal = self.ch_byte_to_str();
        let ref_literal = &literal as &str;
        let token_type = match ref_literal {
            "=" => ASSIGN,
            ";" => SEMICOLON,
            "(" => LPAREN,
            ")" => RPAREN,
            "," => COMMA,
            "+" => PLUS,
            "{" => LBRACE,
            "}" => RBRACE,
            _ => {
                if self.ch == 0 {
                    EOF
                } else {
                    match ref_literal {
                        "p" => EOF,
                        _ => ILLEGAL
                    }
                }
            }
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
