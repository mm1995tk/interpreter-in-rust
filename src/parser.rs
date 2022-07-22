use crate::lexer::Lexer;
use crate::token::{Token};

#[derive(PartialEq, Clone, Debug)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Parser {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self {
            lexer,
            cur_token,
            peek_token,
        }
    }

    fn next_token(mut self) -> () {
        let cur_token = self.lexer.next_token();
        let peek_token = self.lexer.next_token();
        self.cur_token = cur_token;
        self.peek_token = peek_token;
    }
}
