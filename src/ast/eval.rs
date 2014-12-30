use std::collections::HashMap;
use ast::{Program, Statement, Expression, Type};

pub fn eval_program(p: &Program) {
  let Program(ref top) = *p;
  let mut ctx = Context::new();
  ctx.eval(top);
}

struct Context {
  symbol_value: HashMap<String, Expression>,
  symbol_types: HashMap<String, Type>
}

impl Context {
  fn new() -> Context {
    Context {
      symbol_value: HashMap::new(),
      symbol_types: HashMap::new()
    }
  }

  fn get(&self, s: &String) -> &Expression {
    match self.symbol_value.get(s) {
      Some(i) => i,
      None => panic!("Identifier '{}' is not defined", s)
    }
  }

  fn eval(&mut self, s: &Statement) {
    match *s {
      Statement::Block(ref c) => for s in c.iter() { self.eval(s); },
      Statement::Declare(ref t, ref ident, ref expr) => self.declare(t, ident, expr),
      Statement::Assign(ref ident, ref expr) => self.assign(ident, expr),
      Statement::Print(ref s) => println!("{} = {}", s, self.get(s))
    }
  }

  fn declare(&mut self, t: &Type, ident: &String, expr: &Expression) {
    self.symbol_types.insert(ident.clone(), t.clone());
    self.assign(ident, expr);
  }

  fn assign(&mut self, ident: &String, expr: &Expression) {
    let evaluated_expr = self.eval_expression(expr);
    self.symbol_value.insert(ident.clone(), evaluated_expr);
  }

  fn eval_expression(&self, expr: &Expression) -> Expression {
    expr.clone()
  }
}

