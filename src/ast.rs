#[derive(PartialEq, Clone, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    Let(Ident, Expr),
    Dummy
}

#[derive(PartialEq, Clone, Debug)]
pub struct Ident(pub String);

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Ident(Ident),
}
