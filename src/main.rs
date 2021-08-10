use std::env;
use std::fs;
use std::process;
mod globals;
mod logger;
mod lexer;
use lexer::Lexer;


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
    /*let tokens: Vec<lexer::Token> =*/ lexer.tokenize(filecontent);
    // println!("{:?}", tokens);
    println!("========================================================================");
    lexer.print_tokens();
}
