use std::env;
use std::fs;
use std::process;
mod globals;
mod logger;
mod lexer;
mod visitor;
mod ast;
mod parser;
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
    // let expr = Expr::AttributeRef{
    //     object: Box::new(Expr::Literal{
    //         value: lexer::Token{
    //             tok_type: TokenType::STRING_LITERAL,
    //             col: 2, 
    //             line: 1,
    //             value: "hello".to_string()
    //         },
    //         datatype: ast::Datatype::yet_to_infer
    //     }),
    //     name: lexer::Token{
    //         tok_type: TokenType::IDENTIFIER,
    //         value: String::from("dummy"),
    //         col : 2,
    //         line : 1
    //     },
    //     datatype: ast::Datatype::int32
    // };

    // let mut printer: Printer = Printer{
    //     space_width: 0
    // };
    // let stmt: Stmt = Stmt::Expression{
    //     expr: Box::new(expr),
    // };

    // let expr2 = Expr::Literal{
    //     // ptype: ast::Primary_Type::Bool_literal,
    //     value: lexer::Token { tok_type: TokenType::K_TRUE, value: String::from("true"), line: 3, col: 4 },
    //     datatype: ast::Datatype::bool
    // };
    // let stmt2: Stmt = Stmt::Expression{
    //     expr: Box::new(expr2)
    // };

    // let stmt3: Stmt = Stmt::Block{
    //     statements: vec![Box::new(stmt), Box::new(stmt2)]
    // };
    // printer.visit_stmt(&stmt3); 
}
