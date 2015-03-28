#![allow(dead_code)]
#![allow(deprecated)]
#![feature(box_syntax)]
#![feature(core)]
#![feature(collections)]
#![feature(plugin)]
#![feature(old_io)]
#![plugin(peg_syntax_ext)]

use std::old_io;

mod ast;

// Old WACC syntax
peg! w1(r#"
    use ast;

    #[pub]
    program -> ast::Program
        = .* { ast::Program(vec![]) }
"#);

// New syntax (eg. WACC2 or W2)
peg! w2(r#"
    use ast;

    #[pub]
    program -> ast::Program
        = mlsep* fs:(function ++ (mlsep*)) mlsep* { ast::Program(fs) }

    // Functions
    function -> ast::Function
        = "func" sep+ n:identifier function_args mlsep* s:statement {
                ast::Function {
                    name: n,
                    returns: ast::Type::Void,
                    statements: match s {
                        ast::Statement::Block(v) => v,
                        _ => vec![s]
                    }
                }
            }
        / "func" sep+ n:identifier function_args sep* "->" sep* t:type mlsep* s:statement {
                ast::Function {
                    name: n,
                    returns: t,
                    statements: match s {
                        ast::Statement::Block(v) => v,
                        _ => vec![s]
                    }
                }
            }

    function_args
        = "(" sep* ((type sep* identifier) ** (sep* "," sep*)) sep* ")"

    // Statements
    statement -> ast::Statement
        = "{" mlsep* sl:(statement ++ (sep* [;\n] sep*)) mlsep* "}" { ast::Statement::Block(sl) }
        / t:type sep+ s:identifier sep* "=" sep* e:expression sep* { ast::Statement::Declare(t, s, e) }
        / "return" sep* e:expression { ast::Statement::Return(e) }
        / "println" sep* e:expression { ast::Statement::Print(e) }

    // Expressions
    expression -> ast::Expression
        = e:add_expression { e }

    add_expression -> ast::Expression
        = e1:basic_expression sep* "+" sep* e2:add_expression {
                ast::Expression::Binary(ast::BinaryOp::Add, box e1, box e2)
            }
        / e:basic_expression { e }

    basic_expression -> ast::Expression
        = "-"? [0-9]+ { ast::Expression::Int(match_str.parse().unwrap()) }
        / '"' s:str_literal '"' { ast::Expression::Str(s) }
        / "true" { ast::Expression::Bool(true) }
        / "false" { ast::Expression::Bool(false) }
        / s:identifier "(" sep* ((char_literal*) ** (sep* "," sep*)) ")" {
                ast::Expression::Str("Hello".to_string())
            }
        / s:identifier { ast::Expression::Identifier(s) }

    str_literal -> String
        = char_literal* { match_str.to_string() }
    
    char_literal -> char
        = [a-zA-Z0-9 \n] { match_str.char_at(0) }

    identifier -> String
        = [a-zA-Z_]+ { match_str.to_string() }

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
    sep = [ \t]
    mlsep = [ \t\n\r]
"#);

fn print_code(s: String) {
    let lines: Vec<&str> = s.split("\n").collect();
    let mut counter = 1;
    for l in lines {
        println!("{} {}", counter, l);
        counter += 1;
    }
}

fn main() {
    // Gather input
    let input = String::from_utf8(old_io::stdin().read_to_end().unwrap()).unwrap();
    println!("Input:");
    print_code(input.clone());
    println!("");

    // Parse program
    let mut program = match w2::program(input.as_slice()) {
        Ok(p) => p,
        Err(s) => { println!("Syntax Error: {}", s); return }
    };
    println!("AST:\n{:?}", program);

    // Semantic check and derive types
    match ast::semantic::check_program(&mut program) {
        Ok(_) => {},
        Err(s) => { println!("Semantic Error: {}", s); return }
    };
    
    // Generate Code
}
