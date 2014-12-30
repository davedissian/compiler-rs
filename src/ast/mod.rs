pub struct Program {
  pub top: Statement
}

pub enum Statement {
  Block(Vec<Statement>),
  Assign(String, int),
  Print(String)
}

pub mod eval;

