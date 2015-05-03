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

use std::path::Path;
use std::io::Read;
use std::fs::File;
use std::env;
use getopts::Options;

#[derive(PartialEq)]
enum Stage
{
    AST,
    CodeGeneration
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

fn compile(input: &String,
           use_new_syntax: bool,
           disable_semantic_check: bool,
           run_to_stage: Stage) -> Result<String, CompileError> {
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
    if disable_semantic_check {
        // emit warning
        println!("warning: skipping semantic check");
    } else {
        match ast::semantic::check_program(&mut program) {
            Ok(_)  => {},
            Err(s) => { println!("Semantic Error: {}", s); return Err(CompileError::Semantic) }
        };
    }
  
    // Should we stop here?
    if run_to_stage == Stage::AST {
        println!("AST:\n{:?}", program);
        return Ok(String::new());
    }
    
    // Pass AST to the code generator selected
    
    // All OK
    Ok(String::new())
}

fn main() {
    // Collect command line args
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    // Parse options
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("o", "output", "output file", "<file>");
    opts.optopt("", "syntax","language syntax to use - must be either 'wacc' or 'wacc2'", "<type>");
    opts.optflag("", "ast", "display the abstract syntax tree and exit");
    opts.optflag("", "i-know-what-im-doing", "disable semantic checking");
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
    let disable_semantic_check = matches.opt_present("i-know-what-im-doing");
    let stage = if matches.opt_present("ast") { Stage::AST } else { Stage::CodeGeneration };

    // Compile files

    // Read file
    let mut input = String::new();
    if matches.free.is_empty() {
        std::io::stdin().read_to_string(&mut input);
    } else {
        let mut file_handle = File::open(&matches.free[0]).unwrap();
        file_handle.read_to_string(&mut input).unwrap();
    }
    
    // Compile file
    let output = match compile(&input, use_new_syntax, disable_semantic_check, stage) {
        Ok(o) => o,
        Err(t) => {
            match t {
                CompileError::Syntax   => std::env::set_exit_status(100),
                CompileError::Semantic => std::env::set_exit_status(200)
            };
            return
        }
    };
    
    // Determine output file
    let output_file = if let Some(f) = matches.opt_str("o") {
        f
    } else {
        if matches.free.is_empty() {
            String::from_str("output")
        } else {
            let file = Path::new(&matches.free[0]);
            let filename = file.file_name().unwrap().to_str().unwrap();
            let ext = file.extension().unwrap().to_str().unwrap();
            String::from_str(&filename[0..(filename.len() - ext.len() - 1)])
        }
    };
    println!("{}", output_file);
}
