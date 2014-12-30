use std::collections::HashMap;
use ast::{Program, Statement, Expression, Type};

pub fn check_program(p: &mut Program) -> Result<(), String> {
  let Program(ref mut top) = *p;
  let mut ctx = Context::new();
  ctx.check_statement(top)
}

struct Context {
  depth: uint,
  variables: Vec<HashMap<String, Type>>
}

impl Context {
  fn new() -> Context {
    Context {
      depth: 0,
      variables: Vec::new()
    }
  }

  fn push_scope(&mut self) {
    self.variables.push(HashMap::new());
    self.depth = self.variables.len() - 1;
  }

  fn pop_scope(&mut self) {
    self.variables.pop();
    self.depth = self.variables.len() - 1;
  }

  fn check_statement(&mut self, s: &mut Statement) -> Result<(), String>{
    match *s {
      Statement::Block(ref mut v) => {
        self.push_scope();
        for s in v.iter_mut() {
          match self.check_statement(s) {
            Err(msg) => { self.pop_scope(); return Err(msg); },
            Ok(_) => ()
          }
        }
        self.pop_scope();
        Ok(())
      },
      Statement::Declare(ref mut t, ref ident, ref expr) => {
        let derived = self.derive_type(expr);
        if *t == Type::Unknown { *t = derived.clone(); }
        if *t == derived {
          self.variables[self.depth].insert(ident.clone(), t.clone());
          Ok(())
        } else {
          Err(format!("value being used to initialise '{}' does not match its declared type (expected: {}, actual: {})", ident, t, derived))
        }
      },
      Statement::Assign(ref ident, ref expr) => {
        let derived = self.derive_type(expr);
        match self.variables[self.depth].get(ident) {
          Some(ref t) => if **t != derived {
            Err(format!("cannot assign rvalue to lvalue of a different type (expected: {}, actual: {})", t, derived))
          } else {
            Ok(())
          },
          None => Err(format!("use of undeclared variable '{}'", ident))
        }
      },
      Statement::Print(_) => Ok(())
    }
  }

  fn derive_type(&self, expr: &Expression) -> Type {
    match *expr {
      Expression::Int(_) => Type::Int,
      Expression::Bool(_) => Type::Bool
    }
  }
}
