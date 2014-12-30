use ast::Statement as Stmt;
use ast::Expression as Expr;
use ast::Type;

mod ast;

fn main() {
  let mut program = ast::Program(
    Stmt::Block(vec!(
      Stmt::Declare(Type::Bool, "x".to_string(), Expr::Int(5)),
      Stmt::Print("x".to_string()),
      Stmt::Assign("x".to_string(), Expr::Int(6)),
      Stmt::Print("x".to_string())))
  );

  // Print program
  println!("{}", program);

  // Semantic check and derive types
  match ast::semantic::check_program(&mut program) {
    Err(s) => println!("Semantic Error: {}", s),
    _ => ()
  };
  
  // Run program
  ast::eval::eval_program(&program);
}
