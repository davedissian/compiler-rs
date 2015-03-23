#![allow(dead_code)]
#![feature(box_syntax)]
#![feature(core)]
#![feature(collections)]
#![feature(plugin)]
#![feature(old_io)]
#![plugin(peg_syntax_ext)]

use std::old_io;

mod ast;

// Old WACC syntax
// TODO

// New syntax (eg. WACC2 or W2)
peg! w2(r#"
    use ast;

    #[pub]
    program -> ast::Program
        = mlsep* f:function mlsep* { ast::Program(vec![f]) }

    // Functions
    function -> ast::Function
        = "func" sep+ n:str_lit "()" mlsep* s:statement {
                ast::Function {
                    name: n,
                    statements: match s {
                        ast::Statement::Block(v) => v,
                        _ => vec![s]
                    }
                }
            }

    // Statements
    statement -> ast::Statement
        = "{" mlsep* sl:(statement ++ (sep* [;\n] sep*)) mlsep* "}" { ast::Statement::Block(sl) }
        / t:type sep+ s:str_lit sep* "=" sep* e:expression sep* { ast::Statement::Declare(t, s, e) }
        / "println" sep* e:expression { ast::Statement::Print(e) }

    // Expressions
    expression -> ast::Expression
        = "-"? [0-9]+ { ast::Expression::Int(match_str.parse().unwrap()) }
        / "true" { ast::Expression::Bool(true) }
        / "false" { ast::Expression::Bool(false) }
        / s:str_lit { ast::Expression::Identifier(s) }

    str_lit -> String
        = [a-z]+ { match_str.to_string() }

    // Types
    type -> ast::Type
        //= t:type "[]" { ast::Type::Array(box t) }
        = "pair<" t1:type "," t2:type ">" { ast::Type::Pair(box t1, box t2) }
        / t:basic_type { t }

    basic_type -> ast::Type
        = "var" { ast::Type::Unknown }
        / "int" { ast::Type::Int }
        / "char" { ast::Type::Char }
        / "bool" { ast::Type::Bool }

    // Separators
    sep
        = [ \t]

    mlsep
        = [ \t\n\r]
"#);

fn main() {
    // Gather input
    let input = String::from_utf8(old_io::stdin().read_to_end().unwrap()).unwrap();
    println!("Input:\n{}\n", input);

    // Parse program
    let mut program = match w2::program(input.as_slice()) {
        Ok(p) => p,
        Err(s) => { println!("Syntax Error: {}", s); return }
    };
    println!("AST:\n{:?}\n", program);

    // Semantic check and derive types
    match ast::semantic::check_program(&mut program) {
        Ok(_) => {},
        Err(s) => { println!("Semantic Error: {}", s); return }
    };
    
    // Run program
    ast::eval::eval_program(&program);
}
