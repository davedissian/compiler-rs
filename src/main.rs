#![allow(dead_code, deprecated)]

// Dependencies for peg
#![feature(collections, str_char)]
#![feature(plugin)]
#![plugin(peg_syntax_ext)]

peg_file! wacc_parse("wacc.peg");
peg_file! wacc2_parse("wacc2.peg");

mod ast;

use std::io::Read;

fn print_code(s: &str) {
    let lines: Vec<&str> = s.split("\n").collect();
    let mut counter = 1;
    for l in lines {
        println!("{} {}", counter, l);
        counter += 1;
    }
}

fn main() {
    // Gather input
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    println!("Input:");
    print_code(&input);
    println!("");

    // Parse program
    let mut program = match wacc2_parse::program(&input) {
        Ok(p) => p,
        Err(s) => { println!("Syntax Error: {}", s); return }
    };
    println!("AST:\n{:?}", program);

    // Semantic check and derive types
    match ast::semantic::check_program(&mut program) {
        Ok(_) => {},
        Err(s) => { println!("Semantic Error: {}", s); return }
    };
    
    // Pass AST to the code generator selected
}
