use std::collections::HashMap;
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
mod ir_lowering;
use lexer::Lexer;
// use ast::{Stmt, Expr};
use parser::Parser;
use visitor::Printer;

use crate::ir_lowering::Codegen;
use crate::symbol_table::IRSymbolTable;
use crate::symbol_table::SymbolTable;
use crate::type_check_visitor::TypeChecker;
use crate::visitor::MutableVisitor;
// use crate::globals::TokenType;
use crate::visitor::Visitor;
use crate::visitor::VisitorWithLifeTime;


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
    for decl in &declarations {
        printer.visit_decl(decl);
        // println!("Decl: {:?}", *decl);
    }

    println!("*****************************************************************************************************");
    println!("*****************************************************************************************************");
    println!("*****************************************************************************************************");
    println!("*****************************************************************************************************");
    println!("*****************************************************************************************************");
    let mut type_checker: TypeChecker = TypeChecker { 
        symbol_table: SymbolTable{
            variable_table: HashMap::new(),
            struct_decls: HashMap::new(),
            impl_decls: HashMap::new(),
            trait_decls: HashMap::new(),
            func_table: HashMap::new()
        } 
    };
    let mut decls = declarations.clone();
    for decl in &mut decls {
        type_checker.visit_decl(decl);
    }

    for decl in &decls {
        printer.visit_decl(decl);
    }
 
    let context = inkwell::context::Context::create();
    let module = context.create_module("main_mod");
    let builder = context.create_builder();

    let mut codegenerator = Codegen{
        context: &context,
        builder: &builder,
        module: &module,
        symbol_table: IRSymbolTable { 
            variable_table: HashMap::new(), 
            struct_decls: HashMap::new(),
            impl_decls: HashMap::new(),
            trait_decls: HashMap::new(),
            func_table: HashMap::new()
        },
        current_scope: globals::Scope::Global,
        curr_fn_value: None,
        is_parsing_lvalue: false
    };

    for decl in &decls {
        codegenerator.visit_decl(decl);
    }

    codegenerator.module.print_to_stderr();
}
