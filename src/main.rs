use std::env;
use std::fs;
use std::process;
mod globals;
mod logger;
mod lexer;
mod visitor;
mod ast;
mod parser;
mod symbol_table;
mod type_check_visitor;
use lexer::Lexer;
// use ast::{Stmt, Expr};
use parser::Parser;
use visitor::Printer;

// use crate::globals::TokenType;
use crate::visitor::Visitor;


fn main()  {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments: [program name] filename");
        process::exit(1);
    }
    let  filecontent = fs::read_to_string(args[1].clone()).unwrap_or_else(|err| {
        println!("Problem occured while reading the file: {}", err);
        process::exit(1);
    });

    println!("\nFile content:\n{}", filecontent);

    let mut lexer: Lexer = Lexer::new();
    let tokens: Vec<lexer::Token> = lexer.tokenize(filecontent);
    // println!("{:?}", tokens);
    println!("========================================================================");
    lexer.print_tokens();
    println!("=========================================================================");

    let mut parser: Parser = Parser::new();
    let declarations: Vec<Box<ast::Decl>> = parser.parse(tokens.clone());

    println!("Declarations: {}", declarations.len());
    let mut printer: Printer = Printer{space_width: 0};
    for decl in declarations {
        printer.visit_decl(&decl);
        // println!("Decl: {:?}", *decl);
    }
}
