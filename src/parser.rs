use crate::ast::{Expr, Ident, Program, Statement};
use crate::lexer::Lexer;
use crate::token::Token;

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self {
            lexer,
            cur_token,
            peek_token,
        }
    }

    fn next_token(&mut self) -> () {
        let cur_token = self.peek_token.clone();
        let peek_token = self.lexer.next_token();
        self.cur_token = cur_token;
        self.peek_token = peek_token;
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements: Vec<Statement> = vec![];
        loop {
            if self.cur_token == Token::EOF {
                break;
            }

            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }

            self.next_token();
        }

        Program { statements }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token {
            Token::LET => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        match self.peek_token.clone() {
            Token::IDENT(literal) => {
                self.next_token();

                if self.peek_token != Token::ASSIGN {
                    return None;
                }

                loop {
                    if self.cur_token == Token::SEMICOLON {
                        break;
                    }
                    self.next_token();
                }

                let ident = Ident(std::str::from_utf8(&literal).unwrap().to_string());
                let expr = Expr::Ident(ident.clone());

                Some(Statement::Let(ident, expr))
            }
            _ => None,
        }
    }
}
// ##################################################################################################################
#[cfg(test)]
mod test {
    use crate::ast::Statement;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[derive(Debug, PartialEq, Eq)]
    enum LetTestErr {
        NotLetStatement,
        NotMatchName,
    }

    #[test]
    fn test_let_statements() {
        let input = "let x = 5;
let y = 10;
let foobar = 838383;";
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let statements = parser.parse_program().statements;

        if statements.len() != 3 {
            panic!("statementsの要素が3でない")
        }

        for (statement, name) in statements.iter().zip(vec!["x", "y", "foobar"]) {
            let result = content_of_test_let_statements(statement, name);
            if let Some(err) = &result {
                println!("{:?}", &err)
            }
            assert_eq!(result, None)
        }
    }

    fn content_of_test_let_statements(statement: &Statement, name: &str) -> Option<LetTestErr> {
        match statement {
            Statement::Let(ident, _) => {
                if ident.0 == name {
                    None
                } else {
                    Some(LetTestErr::NotMatchName)
                }
            }
            _ => Some(LetTestErr::NotLetStatement),
        }
    }
}
