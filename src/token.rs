#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    KeyWord(KeyWord),
}
#[derive(Debug, PartialEq, Eq)]
pub enum KeyWord {
    FUNCTION,
    LET,
}

pub fn judge_token(literal: &str) -> Option<KeyWord> {
    use crate::token::KeyWord::*;
    match literal {
        "let" => Some(LET),
        "fn" => Some(FUNCTION),
        _ => None,
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
