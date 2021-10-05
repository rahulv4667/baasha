use std::collections::HashMap;
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
use clap::App;
use clap::Arg;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::targets::TargetMachine;
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

#[allow(dead_code)]
struct Driver {
    file_name: String,
    emit_tokens: bool,
    emit_parse_tree: bool,
    emit_typed_tree: bool,
    emit_llvm_ir: bool,
    target: String 
}

impl Driver {
    fn lex(&self, filecontent: String) -> Vec<lexer::Token> {
        let mut lexer: Lexer = Lexer::new();
        let (tokens, has_errors) = lexer.tokenize(filecontent);
        
        if self.emit_tokens {
            println!("===================================Lexer=================================");
            lexer.print_tokens();
            println!("=========================================================================");
        }

        if has_errors {
            process::exit(1);
        }
        return tokens;
    }


    fn parse(&self, tokens: Vec<lexer::Token>) -> Vec<Box<ast::Decl>> {
        let mut parser: Parser = Parser::new();
        let (declarations, has_errors) = parser.parse(tokens.clone());
        if self.emit_parse_tree {
            println!("===================================Parse Tree=================================");
            let mut printer: Printer = Printer{space_width: 0};
            for decl in &declarations {
                printer.visit_decl(decl);
                // println!("Decl: {:?}", *decl);
            }
            println!("=========================================================================");
            
        }

        if has_errors {
            process::exit(1);
        }
        return declarations;
    }

    fn type_check(&self, declarations: Vec<Box<ast::Decl>>) -> Vec<Box<ast::Decl>> {
        let mut type_checker: TypeChecker = TypeChecker { 
            symbol_table: SymbolTable{
                variable_table: HashMap::new(),
                struct_decls: HashMap::new(),
                impl_decls: HashMap::new(),
                trait_decls: HashMap::new(),
                func_table: HashMap::new()
            },
            has_errors: false 
        };
        let mut decls = declarations.clone();
        for decl in &mut decls {
            type_checker.visit_decl(decl);
        }
        if self.emit_typed_tree {
            println!("===================================Typed Tree=================================");
            let mut printer: Printer = Printer{space_width: 0};
            for decl in &decls {
                printer.visit_decl(decl);
                // println!("Decl: {:?}", *decl);
            }
            println!("=========================================================================");
            
        }

        if type_checker.has_errors {
            process::exit(1);
        }
        return decls;
    }


    fn generate_llvm(&self, decls: Vec<Box<ast::Decl>>) -> () {
        let context = inkwell::context::Context::create();
        let module = context.create_module("main_mod");
        let builder = context.create_builder();

        // let fpm: PassManager<Module> = PassManager::<Module>::create(module);
        // fpm.add_instruction_combining_pass();
        // fpm.add_reassociate_pass();
        // fpm.add_gvn_pass();
        // fpm.add_cfg_simplification_pass();
        // fpm.add_basic_alias_analysis_pass();
        // fpm.add_promote_memory_to_register_pass();
        // fpm.add_instruction_combining_pass();
        // fpm.add_reassociate_pass();

        // fpm.run_on(&module);

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

        if self.emit_llvm_ir {
            let llvmir = codegenerator.module.print_to_string().to_string();
            println!("=======================LLVM IR=======================");
            println!("{}", llvmir);
            println!("=====================================================");
        }
        
        println!("{:#?}", codegenerator.module.verify());
        let target_machine = self.compile_to_obj();
        let path = std::path::Path::new("main.o");
        assert!(target_machine.write_to_file(
            &codegenerator.module, 
            inkwell::targets::FileType::Object, 
            path
        ).is_ok());
        

        return;

        // unimplemented!();
    }

    fn compile_to_obj(&self) -> TargetMachine {
        inkwell::targets::Target::initialize_x86(&inkwell::targets::InitializationConfig::default());

        let opt = inkwell::OptimizationLevel::Default;
        let reloc= inkwell::targets::RelocMode::Default;
        let model = inkwell::targets::CodeModel::Default;
        
    
        let target= inkwell::targets::Target::from_name("x86-64").unwrap();
        let target_machine = target.create_target_machine(
            &inkwell::targets::TargetTriple::create("x86_64-pc-linux-gnu"), 
            "x86-64", 
            "+avx2", 
            opt, 
            reloc, 
            model
        ).unwrap();

        return target_machine;
    }


    fn compile_to_llvm(&self) {
        let  filecontent = fs::read_to_string(self.file_name.clone()).unwrap_or_else(|err| {
            println!("Problem occured while reading the file: {}", err);
            process::exit(1);
        });

        // lexing
        let tokens= self.lex(filecontent);

        // Parsing
        let declarations = self.parse(tokens);
        
        // Type checking
        let declarations = self.type_check(declarations);


        // LLVM IR
        let _ = self.generate_llvm(declarations);        
    }

}

fn main()  {

    let matches = App::new("Baasha Compiler")
                    .version("1.0")
                    .author("Rahul V. <4667rahul@gmail.com>")
                    .about("Compiles .bs files")
                    .arg(Arg::with_name("filename")
                        .short("f")
                        .long("filename")
                        .value_name("FILE")
                        .help("Takes the input .bs file")
                        .takes_value(true))
                    .arg(Arg::with_name("tokens")
                        .short("t")
                        .long("emit-tokens")
                        .help("Emits a list of lexical tokens")
                        .takes_value(true))
                    .arg(Arg::with_name("parsetree")
                        .short("p")
                        .long("emit-parse-tree")
                        .help("Emits AST after initial parsing")
                        .takes_value(true))
                    .arg(Arg::with_name("typedtree")
                        .short("d")
                        .long("emit-typed-tree")
                        .help("Emits AST after type checking")
                        .takes_value(true))
                    .arg(Arg::with_name("llvmir")
                        .short("l")
                        .long("emit-llvm")
                        .help("Emits LLVM IR of the given code")
                        .takes_value(true))
                    .arg(Arg::with_name("target")
                        .short("g")
                        .long("target")
                        .help("The target triple of which object files need to be generated")
                        .takes_value(true))
                    .get_matches();
        
        // matches.value_of(name)

    let driver = Driver{
        file_name: matches.value_of("filename").unwrap_or("test.bs").to_string(),
        emit_tokens: matches.value_of("tokens").unwrap_or("false").contains("true"),
        emit_parse_tree: matches.value_of("parsetree").unwrap_or("false").contains("true"),
        emit_typed_tree: matches.value_of("typedtree").unwrap_or("false").contains("true"),
        emit_llvm_ir: matches.value_of("llvmir").unwrap_or("false").contains("true"),
        target: matches.value_of("target")
            .unwrap_or(inkwell::targets::TargetMachine::get_default_triple().as_str().to_str().unwrap())
            .to_string()
    };

    driver.compile_to_llvm();

    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     eprintln!("Not enough arguments: [program name] filename");
    //     process::exit(1);
    // }
    // let  filecontent = fs::read_to_string(args[1].clone()).unwrap_or_else(|err| {
    //     println!("Problem occured while reading the file: {}", err);
    //     process::exit(1);
    // });

    // println!("\nFile content:\n{}", filecontent);

    // let mut lexer: Lexer = Lexer::new();
    // let tokens: Vec<lexer::Token> = lexer.tokenize(filecontent);
    // // println!("{:?}", tokens);
    // println!("========================================================================");
    // lexer.print_tokens();
    // println!("=========================================================================");

    // let mut parser: Parser = Parser::new();
    // let declarations: Vec<Box<ast::Decl>> = parser.parse(tokens.clone());

    // println!("Declarations: {}", declarations.len());
    // let mut printer: Printer = Printer{space_width: 0};
    // for decl in &declarations {
    //     printer.visit_decl(decl);
    //     // println!("Decl: {:?}", *decl);
    // }

    // println!("*****************************************************************************************************");
    // println!("*****************************************************************************************************");
    // println!("*****************************************************************************************************");
    // println!("*****************************************************************************************************");
    // println!("*****************************************************************************************************");
    // let mut type_checker: TypeChecker = TypeChecker { 
    //     symbol_table: SymbolTable{
    //         variable_table: HashMap::new(),
    //         struct_decls: HashMap::new(),
    //         impl_decls: HashMap::new(),
    //         trait_decls: HashMap::new(),
    //         func_table: HashMap::new()
    //     } 
    // };
    // let mut decls = declarations.clone();
    // for decl in &mut decls {
    //     type_checker.visit_decl(decl);
    // }

    // for decl in &decls {
    //     printer.visit_decl(decl);
    // }
 
    // let context = inkwell::context::Context::create();
    // let module = context.create_module("main_mod");
    // let builder = context.create_builder();

    // let mut codegenerator = Codegen{
    //     context: &context,
    //     builder: &builder,
    //     module: &module,
    //     symbol_table: IRSymbolTable { 
    //         variable_table: HashMap::new(), 
    //         struct_decls: HashMap::new(),
    //         impl_decls: HashMap::new(),
    //         trait_decls: HashMap::new(),
    //         func_table: HashMap::new()
    //     },
    //     current_scope: globals::Scope::Global,
    //     curr_fn_value: None,
    //     is_parsing_lvalue: false
    // };

    // for decl in &decls {
    //     codegenerator.visit_decl(decl);
    // }

    // codegenerator.module.print_to_stderr();
}
