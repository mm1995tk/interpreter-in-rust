use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,
    IDENT(Vec<u8>),
    INT(Vec<u8>),
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NotEQ,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::ILLEGAL => write!(f, "Token: ILLEGAL"),
            Token::EOF => write!(f, "Token: EOF, Literal: '\\u{{0}}'"),
            Token::IDENT(literal) => {
                write!(f, "Token: IDENT, {}", std::str::from_utf8(literal).unwrap())
            }
            Token::INT(literal) => write!(
                f,
                "Token: INT, Literal: {}",
                std::str::from_utf8(literal).unwrap()
            ),
            Token::ASSIGN => write!(f, "Token: ASSIGN, Literal: ="),
            Token::PLUS => write!(f, "Token: PLUS, Literal: +"),
            Token::MINUS => write!(f, "Token: MINUS, Literal: -"),
            Token::BANG => write!(f, "Token: BANG, Literal: !"),
            Token::ASTERISK => write!(f, "Token: ASTERISK, Literal: *"),
            Token::SLASH => write!(f, "Token: SLASH, Literal: /"),
            Token::LT => write!(f, "Token: LT, Literal: <"),
            Token::GT => write!(f, "Token: GT, Literal: >"),
            Token::EQ => write!(f, "Token: EQ, Literal: =="),
            Token::NotEQ => write!(f, "Token: NotEQ, Literal: !="),
            Token::COMMA => write!(f, "Token: COMMA, Literal: ,"),
            Token::SEMICOLON => write!(f, "Token: SEMICOLON, Literal: ;"),
            Token::LPAREN => write!(f, "Token: LPAREN, Literal: ("),
            Token::RPAREN => write!(f, "Token: RPAREN, Literal: )"),
            Token::LBRACE => write!(f, "Token: LBRACE, Literal: {{"),
            Token::RBRACE => write!(f, "Token: RBRACE, Literal: }}"),
            Token::FUNCTION => write!(f, "Token: FUNCTION, Literal: fn"),
            Token::LET => write!(f, "Token: LET, Literal: let"),
            Token::TRUE => write!(f, "Token: TRUE, Literal: true"),
            Token::FALSE => write!(f, "Token: FALSE, Literal: false"),
            Token::IF => write!(f, "Token: IF, Literal: if"),
            Token::ELSE => write!(f, "Token: ELSE, Literal: else"),
            Token::RETURN => write!(f, "Token: RETURN, Literal: return"),
        }
    }
}

pub fn get_token_type(literal: &[u8]) -> Option<Token> {
    use crate::token::Token::*;

    match literal {
        b"=" => Some(ASSIGN),
        b";" => Some(SEMICOLON),
        b"(" => Some(LPAREN),
        b")" => Some(RPAREN),
        b"," => Some(COMMA),
        b"+" => Some(PLUS),
        b"-" => Some(MINUS),
        b"!" => Some(BANG),
        b"*" => Some(ASTERISK),
        b"/" => Some(SLASH),
        b"<" => Some(LT),
        b">" => Some(GT),
        b"==" => Some(EQ),
        b"!=" => Some(NotEQ),
        b"{" => Some(LBRACE),
        b"}" => Some(RBRACE),
        b"let" => Some(LET),
        b"fn" => Some(FUNCTION),
        b"true" => Some(TRUE),
        b"false" => Some(FALSE),
        b"if" => Some(IF),
        b"else" => Some(ELSE),
        b"return" => Some(RETURN),
        [0] => Some(EOF),
        _ => None,
    }
}
