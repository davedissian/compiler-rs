use std::collections::HashMap;
use ast::{Program, Statement};

struct Context {
  symbol_table: HashMap<String, int>
}

impl Context {
  fn new() -> Context {
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

pub fn eval_statement(ctx: &mut Context, s: &Statement) {
  match *s {
    Statement::Block(ref c) => {
      for s in c.iter() {
        eval_statement(ctx, s);
      }
    },
    Statement::Assign(ref s, i) => ctx.set(s, i),
    Statement::Print(ref s) => println!("{} = {}", s, ctx.get(s)),
  }
}

