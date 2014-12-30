use std::collections::HashMap;
use ast::*;

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
        match self.derive_type(expr) {
          Ok(ref derived) => {
            if *t == Type::Unknown {
              *t = derived.clone();
            }
            if *t == *derived {
              self.variables[self.depth].insert(ident.clone(), t.clone());
              Ok(())
            } else {
              Err(format!("value being used to initialise '{}' does not match its declared type (expected: {}, actual: {})", ident, t, derived))
            }
          },
          Err(msg) => Err(msg)
        }
      },
      Statement::Assign(ref ident, ref expr) => {
        match self.derive_type(expr) {
          Ok(ref derived) => match self.variables[self.depth].get(ident) {
            Some(ref t) => if **t != *derived {
              Err(format!("cannot assign rvalue to lvalue of a different type (expected: {}, actual: {})", t, derived))
            } else {
              Ok(())
            },
            None => Err(format!("use of undeclared variable '{}'", ident))
          },
          Err(msg) => Err(msg)
        }
      },
      Statement::Print(_) => Ok(())
    }
  }

  fn derive_type(&self, expr: &Expression) -> Result<Type, String> {
    match *expr {
      Expression::Int(_) => Ok(Type::Int),
      Expression::Char(_) => Ok(Type::Char),
      Expression::Bool(_) => Ok(Type::Bool),
      Expression::Unary(ref op, ref expr) => Err("Unimplemented".to_string()),
      Expression::Binary(ref op, ref lhs, ref rhs) => {
        match self.derive_type(&**lhs) {
          Ok(lhs) => match self.derive_type(&**rhs) {
            Ok(rhs) => if lhs == rhs {
              match *op {
                BinaryOp::Add => match lhs {
                  Type::Int => Ok(lhs),
                  _ => Err(format!("invalid type on left of operator '{}' (expected: Int, actual: {})", op, lhs))
                },
                _ => Ok(lhs)
              }
            } else {
              Err(format!("invalid type on right of operator '{}' (expected: {}, actual: {})", op, lhs, rhs))
            },
            Err(msg) => Err(msg)
          },
          Err(msg) => Err(msg)
        }
      }
    }
  }
}
