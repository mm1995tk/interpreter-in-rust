#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Let(Ident, Expr),
    Dummy
}

#[derive(Debug, Clone)]
pub struct Ident(pub String);

#[derive(Debug)]
pub enum Expr {
    Ident(Ident),
}
