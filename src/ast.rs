use crate::token::Token;

trait Node<'a> {
    fn token_literal(&'a self) -> &'a str;
}

trait Statement<'a>: Node<'a> {
    fn statement_node(&'a self) -> &'a str;
}

trait Expression<'a>: Node<'a> {
    fn expression_node(&'a self) -> &'a str;
}

struct Program<'a> {
    statements: Vec<Box<dyn Statement<'a>>>,
}

impl<'a> Program<'a> {
    fn token_literal(&'a self) -> Option<&'a str> {
        if self.statements.len() > 0 {
            Some(self.statements[0].token_literal())
        } else {
            None
        }
    }
}

struct LetStatement<'a> {
    token: Token,
    name: Identifier<'a>,
    expression: dyn Expression<'a>,
}

impl Node<'_> for LetStatement<'_> {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Statement<'_> for LetStatement<'_> {
    fn statement_node(&self) -> &str {
        todo!()
    }
}

struct Identifier<'a> {
    token: Token,
    value: &'a str,
}

impl Node<'_> for Identifier<'_> {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Expression<'_> for Identifier<'_> {
    fn expression_node(&self) -> &str {
        ""
    }
}
