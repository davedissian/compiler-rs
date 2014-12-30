mod ast;

fn main() {
  let program = ast::Program {
    top: ast::Statement::Block(vec!(
      ast::Statement::Assign("x".to_string(), 5),
      ast::Statement::Assign("y".to_string(), 7),
      ast::Statement::Print("x".to_string()),
      ast::Statement::Print("y".to_string()),
      ast::Statement::Assign("x".to_string(), 6),
      ast::Statement::Print("x".to_string())))
  };
  ast::eval::eval_program(&program);
}
