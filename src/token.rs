#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    ILLEGAL,
    EOF,
    IDENT(Vec<u8>),
    INT,
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
