#![allow(dead_code)]
#![feature(box_syntax)]
#![feature(plugin)]

#[plugin] extern crate peg_syntax_ext;

mod ast;

peg! wacc(r#"
    use ast;

    #[pub]
    program -> ast::Program
        = f:function { ast::Program(vec![f]) }

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
        / "var" sep+ s:str_lit sep* "=" sep* i:int_lit sep* { ast::Statement::Declare(ast::Type::Unknown, s, i) }
        / "println" sep* e:expression { ast::Statement::Print(e) }

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
    let mut program = match wacc::program("func main() { var x = 4; println x }") {
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
