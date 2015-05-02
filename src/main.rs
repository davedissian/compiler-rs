#![allow(dead_code, deprecated)]

// To use String::as_str
#![feature(convert)]

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

fn print_code(s: &str) {
    let lines: Vec<&str> = s.split("\n").collect();
    let mut counter = 1;
    for l in lines {
        println!("{} {}", counter, l);
        counter += 1;
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn compile(input: &String, use_new_standard: bool) { 
    println!("Input:");
    print_code(&input);
    println!("");

    // Parse program
    let mut program = if use_new_standard {
        match wacc2_parse::program(&input) {
            Ok(p) => p,
            Err(s) => { println!("Syntax Error: {}", s); return }
        }
    } else {
        match wacc_parse::program(&input) {
            Ok(p) => p,
            Err(s) => { println!("Syntax Error: {}", s); return }
        }
    };
    println!("AST:\n{:?}", program);

    // Semantic check and derive types
    match ast::semantic::check_program(&mut program) {
        Ok(_) => {},
        Err(s) => { println!("Semantic Error: {}", s); return }
    };
    
    // Pass AST to the code generator selected
}

fn main() {
    // Collect command line args
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    // Parse options
    let mut opts = Options::new();
    opts.optopt("", "std","language standard, either 'new' or 'old'", "(new|old)");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // Display help
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let use_new_standard = match matches.opt_default("std", "new") {
        Some(s) => match s.as_str() {
            "old" => false,
            _ => true
        },
        None => true
    };
    
    // Read files
    if !matches.free.is_empty() {
        let mut f = File::open(matches.free[0].clone()).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        compile(&s, use_new_standard);
    } else {
        print_usage(&program, opts);
        return;
    }
}
