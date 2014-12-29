mod ast {
  use std::collections::HashMap;

  // Type
  pub enum Type {
    Assign(String, int),
    Print(String),
    Block
  }

  pub struct Node {
    t: Type,
    children: Vec<Node>
  }

  // Context
  pub struct Context {
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

    fn get(&mut self, s: &String) -> int {
      match self.symbol_table.get(s) {
        Some(i) => *i,
        None => 0
      }
    }
  }

  // Constructors
  pub fn block(cs: Vec<Node>) -> Node {
    Node {
      t: Type::Block,
      children: cs
    }
  }

  pub fn assign(s: &str, i: int) -> Node {
    Node {
      t: Type::Assign(s.to_string(), i),
      children: vec!()
    }
  }

  pub fn print(s: &str) -> Node {
    Node {
      t: Type::Print(s.to_string()),
      children: vec!()
    }
  }

  // Evaluate
  pub fn eval(cxt: &mut Context, n: &Node) {
    match n.t {
      Type::Assign(ref s, i) => cxt.set(s, i),
      Type::Print(ref s) => println!("{} = {}", s, cxt.get(s)),
      Type::Block => for n in n.children.iter() { eval(cxt, n); }
    }
  }
}

fn main() {
  let program =
    ast::block(vec!(
      ast::assign("x", 5),
      ast::assign("y", 7),
      ast::print("x"),
      ast::print("y"),
      ast::assign("x", 6),
      ast::print("x")));
  ast::eval(&mut ast::Context::new(), &program);
}
