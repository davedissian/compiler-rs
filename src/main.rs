mod ast {
  pub struct Program {
    pub top: Statement
  }

  pub enum Statement {
    Block(Vec<Statement>),
    Assign(String, int),
    Print(String)
  }

  // Evaluate
  pub mod eval {
    use std::collections::HashMap;
    use ast::{Program, Statement};

    struct Context {
      pub symbol_table: HashMap<String, int>
    }

    impl Context {
      pub fn new() -> Context {
        Context {
          symbol_table: HashMap::new()
        }
      }

      fn set(&mut self, s: &String, i: int) {
        self.symbol_table.insert(s.clone(), i);
      }

      fn get(&self, s: &String) -> int {
        match self.symbol_table.get(s) {
          Some(i) => *i,
          None => 0
        }
      }
    }

    pub fn eval_program(p: &Program) {
      eval_statement(&mut Context::new(), &p.top)
    }

    pub fn eval_statement(cxt: &mut Context, s: &Statement) {
      match *s {
        Statement::Block(ref c) => {
          for s in c.iter() {
            eval_statement(cxt, s);
          }
        },
        Statement::Assign(ref s, i) => cxt.set(s, i),
        Statement::Print(ref s) => println!("{} = {}", s, cxt.get(s)),
      }
    }
  }
}

fn main() {
  let program = ast::Program {
    top: ast::Statement::Block(vec!(
      ast::Statement::Assign("x".to_string(), 5),
      ast::Statement::Assign("y".to_string(), 7),
      ast::Statement::Print("x".to_string()),
      ast::Statement::Print("y".to_string()),
      ast::Statement::Assign("x".to_string(), 6),
      ast::Statement::Print("x".to_string())))
  };
  ast::eval::eval_program(&program);
}
