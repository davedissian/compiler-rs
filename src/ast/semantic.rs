use std::collections::HashMap;
use ast::*;

pub fn check_program(p: &mut Program) -> Result<(), String> {
    let Program(ref mut top) = *p;
    let mut ctx = Context::new();
    ctx.check_statement(top)
}

struct Context {
    depth: usize,
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
                        Ok(_) => {}
                    }
                }
                self.pop_scope();
                Ok(())
            },

            Statement::Declare(ref mut t, ref ident, ref expr) => {
                let derived = try!(self.derive_type(expr));

                if *t == Type::Unknown {
                    *t = derived.clone();
                }
                if *t == derived {
                    self.variables[self.depth].insert(ident.clone(), t.clone());
                    Ok(())
                } else {
                    Err(format!("value being used to initialise '{:?}' does not match its declared type (expected: {:?}, actual: {:?})", ident, t, derived))
                }
            },

            Statement::Assign(ref ident, ref expr) => {
                let derived = try!(self.derive_type(expr));
                match self.variables[self.depth].get(ident) {
                    Some(ref t) => if **t != derived {
                        Err(format!("cannot assign rvalue to lvalue of a different type (expected: {:?}, actual: {:?})", t, derived))
                    } else {
                        Ok(())
                    },
                    None => Err(format!("use of undeclared variable '{:?}'", ident))
                }
            },

            Statement::Print(ref expr) => {
                try!(self.derive_type(expr));
                Ok(())
            }
        }
    }

    fn derive_type(&self, expr: &Expression) -> Result<Type, String> {
        match *expr {
            Expression::Int(_) => Ok(Type::Int),
            Expression::Char(_) => Ok(Type::Char),
            Expression::Bool(_) => Ok(Type::Bool),

            Expression::Identifier(ref ident) => {
                match self.variables[self.depth].get(ident) {
                    Some(ref t) => Ok((*t).clone()),
                    None => Err(format!("use of undeclared variable '{:?}'", ident))
                }
            },

            Expression::Unary(ref op, ref expr) => {
                let expr_type = try!(self.derive_type(&**expr));
                match *op {
                    UnaryOp::Neg => {
                        match expr_type {
                            Type::Int => Ok(expr_type),
                            _ => Err(format!("invalid type in unary operator '{:?}' (expected: int, actual: {:?})", op, expr_type))
                        }
                    }
                }
            },

            Expression::Binary(ref op, ref lhs, ref rhs) => {
                let t1 = try!(self.derive_type(&**lhs));
                let t2 = try!(self.derive_type(&**rhs));
                
                if t1 == t2 {
                    match *op {
                        BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul => {
                            match t1 {
                                Type::Int => Ok(t1),
                                _ => Err(format!("invalid type on left of operator '{:?}' (expected: int, actual: {:?})", op, t1))
                            }
                        }
                    }
                } else {
                    Err(format!("invalid type on right of operator '{:?}' (expected: {:?}, actual: {:?})", op, t1, t2))
                }
            }
        }
    }
}
