#![allow(dead_code, deprecated)]

// To use String::as_str
#![feature(convert)]

// To use std::env::set_exit_status
#![feature(exit_status)]

// Dependencies for peg
#![feature(collections, str_char)]
#![feature(plugin)]
#![plugin(peg_syntax_ext)]

peg_file! wacc_parse("wacc.peg");
peg_file! wacc2_parse("wacc2.peg");

extern crate getopts;

mod ast;

use std::io::Read;
use std::fs::File;
use std::env;
use getopts::Options;

#[derive(PartialEq)]
enum Stage
{
    AST
}

#[derive(PartialEq)]
enum CompileError
{
    Syntax,
    Semantic
}

fn print_code(s: &str) {
    let lines: Vec<&str> = s.split("\n").collect();
    let mut counter = 1;
    for l in lines {
        println!("{} {}", counter, l);
        counter += 1;
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] <inputs>", program);
    print!("{}", opts.usage(&brief));
}

fn compile(input: &String, use_new_syntax: bool, run_to_stage: Stage) -> Result<(), CompileError> {
    println!("Input:");
    print_code(&input);
    println!("");

    // Parse program
    let mut program = if use_new_syntax {
        match wacc2_parse::program(&input) {
            Ok(p)  => p,
            Err(s) => { println!("Syntax Error: {}", s); return Err(CompileError::Syntax) }
        }
    } else {
        match wacc_parse::program(&input) {
            Ok(p)  => p,
            Err(s) => { println!("Syntax Error: {}", s); return Err(CompileError::Syntax) }
        }
    };

    // Semantic check and derive types
    match ast::semantic::check_program(&mut program) {
        Ok(_)  => {},
        Err(s) => { println!("Semantic Error: {}", s); return Err(CompileError::Semantic) }
    };
  
    // Should we stop here?
    if run_to_stage == Stage::AST {
        println!("AST:\n{:?}", program);
        return Ok(());
    }
    
    // Pass AST to the code generator selected
    
    // All OK
    Ok(())
}

fn main() {
    // Collect command line args
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    // Parse options
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("", "syntax","language syntax to use - must be either 'wacc' or 'wacc2'", "<type>");
    opts.optflag("", "ast", "display the abstract syntax tree and exit");
    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // Display help
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // Read options
    let use_new_syntax = match matches.opt_default("syntax", "wacc2") {
        Some(s) => match s.as_str() {
            "wacc" => false,
            _     => true
        },
        None => true
    };
    let stage = if matches.opt_present("ast") { Stage::AST } else { Stage::AST };
    
    // Compile files
    if !matches.free.is_empty() {
        let mut f = File::open(matches.free[0].clone()).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        if let Err(t) = compile(&s, use_new_syntax, stage) {
            match t {
                CompileError::Syntax   => std::env::set_exit_status(100),
                CompileError::Semantic => std::env::set_exit_status(200)
            }
        }
    } else {
        print_usage(&program, opts);
        return;
    }
}
