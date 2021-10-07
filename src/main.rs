use std::collections::HashMap;
use std::fs;
use std::io::Write;
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
// use inkwell::module::Module;
use inkwell::passes::PassManager;
// use inkwell::passes::PassManagerBuilder;
use inkwell::targets::TargetMachine;
use inkwell::values::FunctionValue;
use lexer::Lexer;
use logger::log_message;
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
    target: String,
    output_filename: String 
}

impl Driver {
    fn lex(&self, filecontent: String) -> Vec<lexer::Token> {
        let mut lexer: Lexer = Lexer::new();
        let (tokens, has_errors) = lexer.tokenize(filecontent);
        
        if self.emit_tokens {
            eprintln!("===================================Lexer=================================");
            lexer.print_tokens();
            eprintln!("=========================================================================");
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
            eprintln!("===================================Parse Tree=================================");
            let mut printer: Printer = Printer{space_width: 0};
            for decl in &declarations {
                printer.visit_decl(decl);
                // eprintln!("Decl: {:?}", *decl);
            }
            eprintln!("=========================================================================");
            
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
            current_scope: globals::Scope::Global,
            has_errors: false 
        };
        let mut decls = declarations.clone();
        for decl in &mut decls {
            type_checker.visit_decl(decl);
        }
        if self.emit_typed_tree {
            eprintln!("===================================Typed Tree=================================");
            let mut printer: Printer = Printer{space_width: 0};
            for decl in &decls {
                printer.visit_decl(decl);
                // eprintln!("Decl: {:?}", *decl);
            }
            eprintln!("=========================================================================");
            
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

        // let fpmb = PassManagerBuilder::create();
        // let fpm = PassManager::<Module>::create(&module);
        let fpm = PassManager::<FunctionValue>::create(&module);
        // let fpm: PassManager<Module> = PassManager::<Module>::create(module);
        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();
        fpm.add_gvn_pass();
        fpm.add_cfg_simplification_pass();
        fpm.add_basic_alias_analysis_pass();
        fpm.add_promote_memory_to_register_pass();
        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();

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

        codegenerator.add_runtime_declarations();

        for decl in &decls {
            codegenerator.visit_decl(decl);
        }

        if self.emit_llvm_ir {
            let llvmir = codegenerator.module.print_to_string().to_string();
            eprintln!("=======================LLVM IR=======================");
            eprintln!("{}", llvmir);
            eprintln!("=====================================================");
        }
        
        // for (func_name, _) in codegenerator.symbol_table.func_table {
        //     fpm.run_on(&codegenerator.module.get_function(func_name.as_str()).unwrap());
        // }
        eprintln!("{:#?}", codegenerator.module.verify());
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
        inkwell::targets::Target::initialize_all(&inkwell::targets::InitializationConfig::default());

        let opt = inkwell::OptimizationLevel::Default;
        let reloc= inkwell::targets::RelocMode::Default;
        let model = inkwell::targets::CodeModel::Default;
        
        let target_triple = inkwell::targets::TargetMachine::get_default_triple();
        let target = inkwell::targets::Target::from_triple(&target_triple).unwrap();
        // let target= inkwell::targets::Target::from_name("x86-64").unwrap();
        // let target_machine = target.create_target_machine(
        //     &inkwell::targets::TargetTriple::create("x86_64-pc-linux-gnu"), 
        //     "x86-64", 
        //     "+avx2", 
        //     opt, 
        //     reloc, 
        //     model
        // ).unwrap();

        
        let target_machine = target.create_target_machine(
            &inkwell::targets::TargetMachine::get_default_triple(), 
            inkwell::targets::TargetMachine::get_host_cpu_name().to_str().unwrap(), 
            inkwell::targets::TargetMachine::get_host_cpu_features().to_str().unwrap(), 
            opt, 
            reloc, 
            model
        ).unwrap();
        
        

        return target_machine;
    }


    fn compile_to_llvm(&self) {
        let  filecontent = fs::read_to_string(self.file_name.clone()).unwrap_or_else(|err| {
            eprintln!("Problem occured while reading the file: {}", err);
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
        

        eprintln!("Currnet Directory: {:?}", std::env::current_dir());
        if std::process::Command::new("clang").output().is_err() {
            // try gcc
            if std::process::Command::new("gcc").output().is_ok() {
                // use gcc
                let output = std::process::Command::new("gcc")
                    .arg("runtime.c")
                    .arg("main.o")
                    .arg("-o")
                    .arg(self.output_filename.as_str())
                    // .current_dir("")
                    .output()
                    .expect("Error occured while trying to create an executable");

                eprintln!("Status: {}", output.status);
                std::io::stdout().write_all(&output.stdout).unwrap();
                std::io::stderr().write_all(&output.stderr).unwrap();

            } else {
                log_message(logger::LogLevel::ERROR, 0, 0, 
                    "No suitable C/C++ compiler to make an executable".to_string()
                );
            }
        } else {
            // use clang
                
            let output = std::process::Command::new("clang")
                .arg("runtime.c")
                .arg("main.o")
                .arg("-o")
                .arg(self.output_filename.as_str())
                // .current_dir("")
                .output()
                .expect("Error occured while trying to create an executable");

            eprintln!("Status: {}", output.status);
            std::io::stdout().write_all(&output.stdout).unwrap();
            std::io::stderr().write_all(&output.stderr).unwrap();
            

        }
            
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
                    .arg(Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .help("The filename of executable file name")
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
            .to_string(),
        output_filename: matches.value_of("output").unwrap_or("a.out").to_string()
    };

    driver.compile_to_llvm();
}
