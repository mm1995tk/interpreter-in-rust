#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
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
    KeyWord(KeyWord),
}
#[derive(Debug, PartialEq, Eq)]
pub enum KeyWord {
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

pub fn get_token_type(literal: &str) -> Option<TokenType> {
    use crate::token::{KeyWord::*, TokenType::*};

    match literal {
        "=" => Some(ASSIGN),
        ";" => Some(SEMICOLON),
        "(" => Some(LPAREN),
        ")" => Some(RPAREN),
        "," => Some(COMMA),
        "+" => Some(PLUS),
        "-" => Some(MINUS),
        "!" => Some(BANG),
        "*" => Some(ASTERISK),
        "/" => Some(SLASH),
        "<" => Some(LT),
        ">" => Some(GT),
        "==" => Some(EQ),
        "!=" => Some(NotEQ),
        "{" => Some(LBRACE),
        "}" => Some(RBRACE),
        "let" => Some(KeyWord(LET)),
        "fn" => Some(KeyWord(FUNCTION)),
        "true" => Some(KeyWord(TRUE)),
        "false" => Some(KeyWord(FALSE)),
        "if" => Some(KeyWord(IF)),
        "else" => Some(KeyWord(ELSE)),
        "return" => Some(KeyWord(RETURN)),
        _ => None,
    }
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
