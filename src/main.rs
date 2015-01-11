#![allow(dead_code)]
#![feature(box_syntax)]
#![feature(plugin)]

#[plugin] extern crate peg_syntax_ext;

mod ast;

peg! wacc(r#"
    use ast;

    #[pub]
    program -> ast::Program
        = s:statement { ast::Program(s) }

    // Statements
    statement -> ast::Statement
        = "var" sep* s:str_lit sep* "=" sep* i:int_lit sep* { ast::Statement::Declare(ast::Type::Unknown, s, i) }
        / "println" sep* e:expression { ast::Statement::Print(e) }
        / "{" mlsep* sl:(statement ++ (sep* [;\n] sep*)) mlsep* "}" { ast::Statement::Block(sl) }

    // Expressions
    expression -> ast::Expression
        = s:str_lit { ast::Expression::Identifier(s) }

    str_lit -> String
        = [a-z]+ { match_str.to_string() }

    int_lit -> ast::Expression
        = [0-9]+ { ast::Expression::Int(match_str.parse().unwrap()) }

    sep
        = [ \t]

    mlsep
        = [ \t\n\r]
"#);

fn main() {
    // Generate AST
    let mut program = match wacc::program("{var x = 4; println x}") {
        Ok(p) => p,
        Err(s) => { println!("Syntax Error: {:?}", s); return }
    };

    // Print program
    println!("{:?}", program);

    // Semantic check and derive types
    match ast::semantic::check_program(&mut program) {
        Ok(_) => {},
        Err(s) => { println!("Semantic Error: {:?}", s); return }
    };
    
    // Run program
    ast::eval::eval_program(&program);
}
