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
      if self.symbol_table.contains_key(s) {
        *self.symbol_table.get_mut(s) = i;
      } else {
        self.symbol_table.insert(s.clone(), i);
      }
    }

    fn get(&mut self, s: &String) -> int {
      self.symbol_table.get_copy(s)
    }
  }

  // Constructors
  pub fn block(cs: Vec<Node>) -> Node {
    Node {
      t: Block,
      children: cs
    }
  }

  pub fn assign(s: &str, i: int) -> Node {
    Node {
      t: Assign(s.to_string(), i),
      children: vec!()
    }
  }

  pub fn print(s: &str) -> Node {
    Node {
      t: Print(s.to_string()),
      children: vec!()
    }
  }

  // Evaluate
  pub fn eval(cxt: &mut Context, n: &Node) {
    match n.t {
      Assign(ref s, i) => cxt.set(s, i),
      Print(ref s) => println!("{} = {}", s, cxt.get(s)),
      Block => for n in n.children.iter() { eval(cxt, n); }
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
