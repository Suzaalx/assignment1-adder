// Assignment 1: Adder Compiler
// Compiles Adder expressions to x86-64 assembly

use sexp::*;
use sexp::Atom::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

// AST for expressions
#[derive(Debug)]
enum Expr {
    Num(i32),
    Add1(Box<Expr>),
    Sub1(Box<Expr>),
    Negate(Box<Expr>),
}

// Parse S-expression into AST
fn parse_expr(s: &Sexp) -> Expr {
    match s {
        // Numbers
        Sexp::Atom(I(n)) => Expr::Num(i32::try_from(*n).unwrap()),
        
        // Operations
        Sexp::List(vec) => {
            match &vec[..] {
                [Sexp::Atom(S(op)), e] if op == "add1" => 
                    Expr::Add1(Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "sub1" => 
                    Expr::Sub1(Box::new(parse_expr(e))),
                [Sexp::Atom(S(op)), e] if op == "negate" => 
                    Expr::Negate(Box::new(parse_expr(e))),
                _ => panic!("Invalid expression: {:?}", s),
            }
        },
        _ => panic!("Invalid expression: {:?}", s),
    }
}

// Generate assembly code
fn compile_expr(e: &Expr) -> String {
    match e {
        Expr::Num(n) => format!("mov rax, {}", *n),
        Expr::Add1(subexpr) => compile_expr(subexpr) + "\n  add rax, 1",
        Expr::Sub1(subexpr) => compile_expr(subexpr) + "\n  sub rax, 1",
        Expr::Negate(subexpr) => compile_expr(subexpr) + "\n  imul rax, -1",
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <input.snek> <output.s>", args[0]);
        std::process::exit(1);
    }

    let in_name = &args[1];
    let out_name = &args[2];

    // Read and parse input
    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    let sexp = parse(&in_contents).unwrap_or_else(|e| {
        panic!("Parse error: {}", e)
    });
    
    let expr = parse_expr(&sexp);
    let instrs = compile_expr(&expr);
    
    // Create assembly program
    let asm_program = format!(
        "section .text
global our_code_starts_here
our_code_starts_here:
  {}
  ret
",
        instrs
    );

    // Write output
    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}

