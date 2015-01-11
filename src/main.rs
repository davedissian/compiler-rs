#![allow(dead_code)]
#![feature(box_syntax)]
#![feature(plugin)]

#[plugin] extern crate peg_syntax_ext;

mod ast;

peg! wacc(r#"
    use ast;

    blank -> ()
        =  {}

    #[pub]
    program -> ast::Program
        = sl:statement_list { ast::Program(ast::Statement::Block(sl)) }

    statement_list -> Vec<ast::Statement>
        = s:statement sep* statement_sep sep* sl:statement_list { let mut v = vec![s]; v.extend(sl.into_iter()); v }
        / s:statement { vec![s] }

    statement_sep -> ()
        = [;\n]

    statement -> ast::Statement
        = "var" sep+ s:str_lit sep* "=" sep* i:int_expr sep* { ast::Statement::Declare(ast::Type::Unknown, s, i) }

    str_lit -> String
        = [a-z]+ { match_str.to_string() }

    int_expr -> ast::Expression
        = [0-9]+ { ast::Expression::Int(match_str.parse().unwrap()) }

    sep -> ()
        = [ \t]
"#);

fn main() {
    // Generate AST
    let mut program = wacc::program("var x = 0; var y = 2").unwrap();

    // Print program
    println!("{:?}", program);

    // Semantic check and derive types
    match ast::semantic::check_program(&mut program) {
        Err(s) => println!("Semantic Error: {:?}", s),
        _ => {}
    };
    
    // Run program
    ast::eval::eval_program(&program);
}
