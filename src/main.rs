use std::collections::HashMap;

enum Type {
  Assign(String, int),
  Print(String),
  Block
}

struct Context {
  symbol_table: HashMap<String, int>
}

struct Node {
  t: Type,
  children: Vec<Node>
}

fn set(s: &String, i: int, cxt: &mut Context) {
  if cxt.symbol_table.contains_key(s) {
    *cxt.symbol_table.get_mut(s) = i;
  } else {
    cxt.symbol_table.insert(s.clone(), i);
  }
}

fn lookup(s: &String, cxt: &mut Context) -> int {
  cxt.symbol_table.get_copy(s)
}

fn eval(cxt: &mut Context, n: &Node) {
  // Match on the type
  match n.t {
    Assign(ref s, i) => set(s, i, cxt),
    Print(ref s) => println!("{} = {}", s, lookup(s, cxt)),
    Block => for n in n.children.iter() { eval(cxt, n); }
  }
}

fn main() {
  let mut context = Context { symbol_table: HashMap::new() };
  let program = Node {
    t: Block,
    children: vec!(
      Node {t: Assign("x".to_string(), 5), children: vec!()},
      Node {t: Print("x".to_string()), children: vec!()})
  };
  eval(&mut context, &program);
}
