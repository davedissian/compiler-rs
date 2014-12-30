use ast::{Program, Statement, Expression, Type};

pub fn check_program(p: &mut Program) -> Result<(), &'static str> {
  let Program(ref mut top) = *p;
  let mut ctx = Context::new();
  ctx.check_statement(top)
}

struct Context;

impl Context {
  fn new() -> Context {
    return Context;
  }

  fn check_statement(&mut self, s: &mut Statement) -> Result<(), &'static str>{
    match *s {
      Statement::Declare(ref mut t, ref ident, ref expr) => {
        let derived = self.derive_type(expr);
        if *t == Type::Unknown {
          *t = derived;
        } else {
          return Err("Mismatching type in declare statement")
        }
      },
      _ => println!("Unhandled check")
    };
    Ok(())
  }

  fn derive_type(&self, expr: &Expression) -> Type {
    Type::Unknown
  }
}
