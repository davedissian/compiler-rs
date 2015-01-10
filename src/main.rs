#![feature(box_syntax)]

use ast::Statement as Stmt;
use ast::Expression as Expr;
use ast::UnaryOp as Un;
use ast::BinaryOp as Bin;
use ast::Type;

mod ast;

fn main() {
    let mut program = ast::Program(
        Stmt::Block(vec!(
            Stmt::Declare(Type::Unknown, "x".to_string(), Expr::Int(0)),
            Stmt::Assign("x".to_string(),
                Expr::Binary(Bin::Add, box Expr::Int(2), box Expr::Int(3))),
            Stmt::Print(Expr::Identifier("x".to_string())),
            Stmt::Print(Expr::Binary(Bin::Sub, box Expr::Int(3), box Expr::Identifier("x".to_string())))
        ))
    );

    // Print program
    println!("{:?}", program);

    // Semantic check and derive types
    match ast::semantic::check_program(&mut program) {
        Err(s) => println!("Semantic Error: {:?}", s),
        _ => ()
    };
    
    // Run program
    ast::eval::eval_program(&program);
}
