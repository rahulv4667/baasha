// use std::borrow::Borrow;
// use std::fs::OpenOptions;
// use std::usize::MAX;

use crate::logger::log_message;
use crate::{ast::*, logger};
use crate::globals::TokenType;
use crate::lexer::Token;

#[allow(dead_code, non_camel_case_types)]
#[derive(PartialEq)]
enum Restriction {
    STRUCT_EXPR
}

#[allow(dead_code)]
pub struct Parser {
    current: usize,
    start: usize,
    tokens: Vec<Token>,
    restrictions: Vec<Restriction>,
    has_errors: bool
}

#[allow(dead_code)]
impl Parser {

    fn match_(&mut self, tok_type: TokenType) -> bool {
        if self.check(tok_type) {
            self.advance();
            return true;
        } else {
            return false;
        }
    }

    fn match_multi(&mut self, tok_types: Vec<TokenType>) -> bool {
        if self.current >= self.tokens.len() {
            return false;
        }

        if let Some(Token{tok_type, ..}) = self.peek() {
            if tok_types.contains(&tok_type) {
                self.advance();
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }


    fn check(&self, ttype: TokenType) -> bool {
        if self.is_end() { return false; }
        return match self.peek() {
            None    => false,
            Some(Token{tok_type, ..})   => tok_type == ttype
        }
    }

    fn is_end(&self) -> bool {
        return self.current >= self.tokens.len() || 
            match self.peek() {
                Some(Token{tok_type, ..}) => tok_type == TokenType::FILE_EOF,
                None => return false
            } /*||
            match self.curr() {
                Some(Token{tok_type,..}) => tok_type == TokenType::FILE_EOF,
                None => return false
            }*/;
    }

    fn peek(&self) -> Option<Token> {
        return Some(self.tokens[self.current].clone());
    }

    fn peek_next(&self) -> Option<Token> {
        if self.current + 1 >= self.tokens.len() { return None; }
        return Some(self.tokens[self.current + 1].clone());
    }

    fn curr(&self) -> Option<Token> { 
        if self.current >= self.tokens.len() { return None; }
        return Some(self.tokens[self.current - 1].clone());
    }

    fn advance(&mut self) -> Option<Token> {
        if !self.is_end() { self.current += 1; }
        return self.curr();
    }

    fn consume(&mut self, ttype: TokenType, error_msg: String) -> Option<Token> {
        if self.check(ttype) { return self.advance(); }
        match self.peek() {
            Some(tok) => {
                self.has_errors = true;
                logger::log_message(logger::LogLevel::ERROR, tok.col, tok.line, error_msg)
            },
            _ => match self.curr() {
                Some(tok) => logger::log_message(logger::LogLevel::ERROR, tok.col, tok.line, error_msg),
                _ => {
                    self.has_errors = true;
                    log_message(logger::LogLevel::ERROR, usize::MAX, usize::MAX, 
                    "Unexpected error occured\n".to_string())
                }
            }
        };

        return None;
    }

    fn consume_multi(&mut self, ttypes: Vec<TokenType>, error_msg: String) -> Option<Token> {
        if self.match_multi(ttypes) {
            return self.curr();
        } 

        match self.peek() {
            Some(tok) => {
                self.has_errors = true;
                logger::log_message(logger::LogLevel::ERROR, tok.col, tok.line, error_msg)
            }
            _ => match self.curr() {
                Some(tok) => logger::log_message(logger::LogLevel::ERROR, tok.col, tok.line, error_msg),
                _ => {
                    self.has_errors = true;
                    log_message(logger::LogLevel::ERROR, usize::MAX, usize::MAX, 
                    "Unexpected error occured\n".to_string())
                }
            }
        };

        return None;
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_end() {
            if let Some(tok) = self.curr() {
                if tok.tok_type == TokenType::SEMICOLON || tok.tok_type == TokenType::CURLY_CLOSE
                {return; }
            }

            if let Some(tok) = self.peek() {
                match tok.tok_type {
                    TokenType::K_STRUCT|TokenType::K_IMPL|TokenType::K_TRAIT|TokenType::K_FUNC
                    => { return; },
                    _ => { self.advance(); () }
                }
            }

        }

    }

    // declaration -> structDecl | implDecl | traitDecl | funcDef
    fn declaration(&mut self) -> Option<Box<Decl>> {
        eprintln!("In declaration()");
        if      self.match_(TokenType::K_STRUCT)    { return self.struct_declaration(); }
        else if self.match_(TokenType::K_IMPL)      { return self.impl_declaration(); }
        else if self.match_(TokenType::K_TRAIT)     { return self.trait_declaration(); }
        else if self.match_(TokenType::K_FUNC)      { return self.func(); }
        else { self.synchronize(); return None; }
    }

    // structDecl -> "struct" IDENTIFIER "{" (IDENTIFIER ("," IDENTIFIER)*) ":" TYPES "}"
    fn struct_declaration(&mut self) -> Option<Box<Decl>>   { 
        eprintln!("In struct_declaration()");
        let name: Token;
        match self.consume(TokenType::IDENTIFIER, 
            "Expected identifier after 'struct' keyword".to_string()) {
            Some(tok)  => name = tok,
            _ => return None,
        };


        match self.consume(TokenType::CURLY_OPEN, "Expected '{' after struct identifier.".to_string()) {
            None => return None,
            Some(_) => ()
        };


        let mut fields: Vec<(Token, Token)> = Vec::new();
        while !self.match_(TokenType::CURLY_CLOSE) {
            let field_name : Token; let field_type: Token;
        
            match self.consume(TokenType::IDENTIFIER,
                "Expected field name inside struct declaration".to_string()) {
                Some(tok) => field_name = tok,
                _ => return None,
            };


            match self.consume(TokenType::COLON, 
                "Expected ':' after field name in struct declaration.".to_string()) {
                    Some(_) => (),
                    None => return None,
            };

            
            match self.consume_multi(TokenType::get_datatypes(), 
            "Expected datatype for the field".to_string()) {
                Some(tok) => field_type = tok,
                _ => return None,
            }

            fields.push((field_name, field_type));

            // if !self.match_(TokenType::COMMA) && !self.match_(TokenType::CURLY_CLOSE) { 
            //     return None; 
            // }
            // self.match_(TokenType::COMMA);
            if !self.match_(TokenType::COMMA) {
                if let Some(peek) = self.peek() {
                    if peek.tok_type != TokenType::CURLY_CLOSE {
                        self.has_errors = true;
                        logger::log_message(logger::LogLevel::ERROR, peek.col, peek.line, 
                            "Expected ',' or '}' after field declaration".to_string());
                        return None;
                    }
                }
            }

        }

        return Some(Box::new(Decl::StructDecl{name, fields}));
    }

    fn impl_declaration(&mut self) -> Option<Box<Decl>>     { 
        eprintln!("In impl_declaration()");
        let mut name: Token;
        let trait_name: Option<Token>;

        match self.consume(TokenType::IDENTIFIER, 
            "Expected struct/trait name after 'impl' keyword".to_string()) {
                Some(tok) => name = tok,
                _ => return None,
        }

        if self.match_(TokenType::K_FOR) {
            trait_name = Some(name);
            

            match self.consume(TokenType::IDENTIFIER, 
                "Expected struct name for which trait is being implemented".to_string()) {
                    Some(tok) => name = tok,
                    _ => return None
            }
        } else {
            trait_name = None;
        }

        match self.consume(TokenType::CURLY_OPEN, "Expected '{' after 'impl' head".to_string()) {
            Some(_) => (),
            None => return None
        }

        let mut funcs: Vec<Box<Decl>> = Vec::new();
        while !self.match_(TokenType::CURLY_CLOSE) {
            match self.consume(TokenType::K_FUNC, 
                "Expected function definition inside impl declaration".to_string()) {
                    Some(_) => {

                        let func_decl = self.func();
                        
                        if let Some(decl) = func_decl {
                        
                            match *decl {
                        
                                Decl::FuncDef{prototype, block} 
                                    => funcs.push(Box::new(Decl::FuncDef{prototype, block})),
                        
                                _ => {
                                    self.has_errors = true;
                                    log_message(logger::LogLevel::ERROR, 
                                        self.tokens[self.current].col, self.tokens[self.current].line, 
                                        "Expected function definition inside impl declaration".to_string());
                                    return None;
                                }
                    
                            }
                    
                        }

                    },
                    _ => return None,

            }
        }

        return Some(Box::new(Decl::ImplDecl{name, trait_name, funcs}));

    }



    fn trait_declaration(&mut self) -> Option<Box<Decl>>    { 
        eprintln!("In trait_declaration()");
        let name: Token;
        match self.consume(TokenType::IDENTIFIER, 
            "Expected trait name after 'trait' keyword".to_string()) {
                Some(tok) => name = tok,
                _ => return None,
        }


        match self.consume(TokenType::CURLY_OPEN, "Expected '{' after 'trait' head".to_string()) {
            Some(_) => (),
            None => return None
        }

        let mut funcs: Vec<Box<Decl>> = Vec::new();
        while !self.match_(TokenType::CURLY_CLOSE) {
            match self.consume(TokenType::K_FUNC, 
                "Expected function definition inside trait declaration.
                             Unlike `Rust`, `Baasha` expects a default implementation.".to_string()) {
                    Some(_) => {

                        match self.func() {
                            Some(decl) => funcs.push(decl),
                            _ => continue,
                        }

                    },
                    _ => return None,

            }
        }

        return Some(Box::new(Decl::TraitDecl{name, funcs}));


    }
    
    // func -> prototype (";" | block)
    fn func(&mut self) -> Option<Box<Decl>>  { 
        eprintln!("In block()");
        let prototype: Box<Decl>;
        match self.prototype() {
            Some(proto) => prototype = proto,
            _ => return None,
        }

        let block: Box<Stmt>;
        match self.block() {
            Some(blk) => block = blk,
            None => return Some(prototype),
        }

        return Some(Box::new(Decl::FuncDef{prototype, block}));
    }

    fn prototype(&mut self) -> Option<Box<Decl>> { 
        eprintln!("In prototype()");
        let name: Token;
        match self.consume(TokenType::IDENTIFIER, 
            "Expected function name after 'func' keyword".to_string()) {
                Some(tok) => name = tok,
                _ => return None, 
        }

        match self.consume(TokenType::BRACKET_OPEN, 
            "Expected '(' after function name".to_string()) {
            Some(_) => (),
            _ => return None,
        }

        let mut params: Vec<(Token, Token)> = Vec::new();

        // reading params
        while !self.match_(TokenType::BRACKET_CLOSE) {

            let mut param_names: Vec<Token> = Vec::new();
            let params_type: Token;
            match self.consume(TokenType::IDENTIFIER, 
                "Expected parameter name".to_string()) {
                    Some(tok) => param_names.push(tok),
                    _ => return None,
            }

            while self.match_(TokenType::COMMA) {
                match self.consume(TokenType::IDENTIFIER, "Expected parameter name".to_string()) {
                    Some(tok) => param_names.push(tok),
                    _ => ()
                }
            }

            match self.consume(TokenType::COLON,
                "Expected ':' after parameter names to separate type".to_string()) {
                    Some(_) => (),
                    _ => return None,
            }

            
            match self.consume_multi(TokenType::get_datatypes(), 
            "Expected typename for parameters".to_string()) {
                Some(tok) => params_type = tok,
                _ => return None,
            }

            for param in param_names {
                params.push((param, params_type.clone()));
            }

            if self.match_(TokenType::COMMA) {}
        }

        // reading return types
        // let mut ret_types: Vec<Token> = vec![];
        let /*mut*/ returntype: Token;// = Token { tok_type: TokenType::ERROR, value: String::new(), line: usize::MAX, col: usize::MAX };
        if self.match_(TokenType::RIGHT_ARROW) {
            
            match self.consume_multi(TokenType::get_datatypes(), 
            "Expected a return type".to_string()) {
                Some(tok) => returntype = tok,//ret_types.push(tok),
                _ => return None,
            }

            return Some(Box::new(Decl::Prototype{name, parameters: params, returntype/*ret_types*/}));

            // while self.match_(TokenType::COMMA) {
            //     match self.consume_multi(TokenType::get_datatypes(), 
            //     "Expected a return type".to_string()) {
            //         Some(tok) => ret_types.push(tok),
            //         _ => return None,
            //     }
            // }

        } else {
            // For now generating error. In future, should default to '()' type.
            self.has_errors = true;
            log_message(logger::LogLevel::ERROR, name.col, name.line, 
                "A return type needs to be provided in prototype of function".to_string());
            // self.synchronize();
            return None;
        }


        //return Some(Box::new(Decl::Prototype{name, parameters: params, returntype/*ret_types*/}));
    }

    fn block(&mut self) -> Option<Box<Stmt>> { 
        eprintln!("In block()");
        match self.consume(TokenType::CURLY_OPEN, 
            "Expected '{' at starting of a block".to_string()) {
                Some(_) => (),
                _ => return None
        }

        let mut statements: Vec<Box<Stmt>> = vec![];
        while !self.match_(TokenType::CURLY_CLOSE) {
            match self.statement() {
                Some(stmt) => statements.push(stmt),
                _ => continue,
            }
        }

        return Some(Box::new(Stmt::Block{statements}));
    }

    fn statement(&mut self) -> Option<Box<Stmt>> { 
        eprintln!("In statement()");
        match self.peek() {
            Some(Token{tok_type, ..}) => {
                match tok_type {
                    TokenType::K_IF     => self.if_stmt(),
                    TokenType::K_FOR    => self.for_stmt(),
                    TokenType::K_RETURN => self.return_stmt(),
                    TokenType::K_WHILE  => self.while_stmt(),
                    TokenType::K_VAR    => self.var_stmt(),
                    TokenType::K_STRUCT|TokenType::K_IMPL|
                    TokenType::K_TRAIT|TokenType::K_FUNC => self.decl_stmt(),
                    TokenType::SEMICOLON    => {self.match_(TokenType::SEMICOLON); None},
                    TokenType::FILE_EOF     => {eprintln!("Peek:{:?}", self.peek());self.advance(); None},
                    _ => self.expr_stmt()
                }
            },
            _ => None,
        }
    }

    fn decl_stmt(&mut self) -> Option<Box<Stmt>> { 
        eprintln!("In decl_stmt()");
        match self.declaration() {
            Some(decl) => Some(Box::new(Stmt::Decl{decl})),
            _ => None,
        }
    }

    fn var_stmt(&mut self) -> Option<Box<Stmt>> {
        eprintln!("In var_stmt()");
        self.consume(TokenType::K_VAR, "Expected 'var' keyword".to_string());

        let name: Token;
        match self.consume(TokenType::IDENTIFIER, 
            "Expected variable name after 'var' keyword".to_string()) {
            Some(tok) => name = tok,
            _ => return None
        }


        let mut typename: Option<Token> = None;
        if self.match_(TokenType::COLON) {
            typename = self.consume_multi(TokenType::get_datatypes(), "Expected type name".to_string());
        }

        let mut initialization_value: Option<Box<Expr>> = None;
        if self.match_(TokenType::EQUAL) {
            // initialization_value = self.expression();
            initialization_value = self.logical_OR_expr();
        }

        if typename.is_none() && initialization_value.is_none() {
            self.has_errors = true;
            log_message(logger::LogLevel::ERROR, 
                name.col, name.line, "A variable declaration requires either a datatype or an initialization value".to_string());
            return None;
        }

        self.consume(TokenType::SEMICOLON, "Expected ';' after variable declaration".to_string());
        return Some(Box::new(Stmt::Var{name, datatype: typename, initialization_value}));
    }

    fn if_stmt(&mut self) -> Option<Box<Stmt>> { 
        eprintln!("In if_stmt()");
        let if_token:Token;
        match self.consume(TokenType::K_IF, "Expected `if` statement".to_string()) {
            Some(tok) => if_token = tok,
            _ => return None
        }

        let condition: Box<Expr>;
        self.restrictions.push(Restriction::STRUCT_EXPR);
        match self.expression() {
            Some(expr) => condition = expr,
            _ => return None,
        }
        self.restrictions.pop();

        let then_block: Box<Stmt>;
        match self.block() {
            Some(blk) => then_block = blk,
            _ => return None,
        }

        let else_block: Option<Box<Stmt>>;
        if self.match_(TokenType::K_ELSE) {
            match self.peek() {
                Some(tok) if tok.tok_type == TokenType::K_IF => else_block = self.if_stmt(),
                _ => else_block = self.block()
            }
        } else {
            else_block = None;
        }

        return Some(Box::new(Stmt::If{if_token, condition, then_block, else_block}));

    }

    fn while_stmt(&mut self) -> Option<Box<Stmt>> { 
        eprintln!("In while_stmt()");
        self.consume(TokenType::K_WHILE, "Expected 'while' keyword".to_string());

        let condition: Box<Expr>;
        match self.expression() {
            Some(expr) => condition = expr,
            _ => return None,
        }

        let block: Box<Stmt>;
        match self.block() {
            Some(blk )=> block = blk,
            _ => return None
        }

        return Some(Box::new(Stmt::While{condition, block}));
    }

    
    fn for_stmt(&mut self) -> Option<Box<Stmt>> { 
        eprintln!("In for stmt()");
        let for_token: Token;
        match self.consume(TokenType::K_FOR, "Expected 'for' keyword".to_string()) {
            Some(tok) => for_token = tok,
            _ => return None
        }

        self.restrictions.push(Restriction::STRUCT_EXPR);
        let mut condition: Box<Expr>;
        let has_condition:bool;
        match self.expression() {
            Some(expr) => condition = expr,
            _ => {
                self.restrictions.pop();
                if let Some(blk) = self.block() {
                    return Some(Box::new(Stmt::For{for_token, initialization:None, condition: None, updation: None, block: blk}));
                } 
                return None;
            }
        }

        if self.match_(TokenType::SEMICOLON) {
            let initialization = condition.clone();
            // has_condition = false;
            match self.expression() {
                Some(expr) => { 
                    has_condition = true;
                    condition = expr;
                }
                _ => {
                    has_condition = false;
                }
            }

            if self.match_(TokenType::SEMICOLON) {
                let updation: Box<Expr>;
                match self.expression() {
                    Some(expr) => {
                        updation = expr;
                        self.restrictions.pop();
                        if let Some(blk) = self.block() {
                            return Some(
                                Box::new(
                                    Stmt::For{
                                        for_token,
                                        initialization:Some(initialization),
                                        condition: if has_condition { Some(condition) } else {None},
                                        updation: Some(updation),
                                        block: blk
                                    }
                                )
                            );
                        } else {
                            return None;
                        }
                        
                    },
                    _ => {
                        self.has_errors = true;
                        log_message(logger::LogLevel::ERROR, self.tokens[self.current].col, self.tokens[self.current].line, 
                            "Expected expression after second ';' in `for` loop".to_string());
                        // self.synchronize();
                        return None;
                    }
                }
            }
        }
        self.restrictions.pop();
        if let Some(blk) = self.block() {
            return Some(Box::new(Stmt::For{
                for_token,
                initialization: None,
                condition: Some(condition),
                updation: None,
                block: blk
            }));
        }
        return None;    
    }


    fn return_stmt(&mut self) -> Option<Box<Stmt>> { 
        eprintln!("In return_stmt()");
        match self.consume(TokenType::K_RETURN, "Expected 'return' keyword".to_string()) {
            Some(_) => (),
            _ => return None,
        }

        match self.expression() {
            Some(expr) => Some(Box::new(Stmt::Return{expr})),
            _ => None
        }
    
    }
    

    fn expr_stmt(&mut self) -> Option<Box<Stmt>> { 
        eprintln!("In expr_stmt()");
        // match self.assignment() {
        //     Some(expr) => Some(Box::new(Stmt::Expression{expr})),
        //     _ => match self.expression() {
        //         Some(expr) => return Some(Box::new(Stmt::Expression{expr})),
        //         _ => {
        //             logger::log_message(logger::LogLevel::ERROR, 
        //                 self.tokens[self.current].col, self.tokens[self.current].line, 
        //                 "Unexpected token in expression".to_string());
        //             self.synchronize();
        //             None
        //         }
        //     }
        // }
        match self.expression() {
            Some(expr) => Some(Box::new(Stmt::Expression{expr})),
            _ => {
                self.has_errors = true;
                logger::log_message(
                    logger::LogLevel::ERROR, 
                    self.tokens[self.current].col, self.tokens[self.start].line, 
                    "Unexpected token in expression".to_string()
                );
                // self.synchronize();
                None
            }
        }
    }


    fn expression(&mut self) -> Option<Box<Expr>> { 
        eprintln!("In expression()");
        let mut expr_list: Vec<Box<Expr>> = vec![];

        match self.assignment() {
            Some(expr) => expr_list.push(expr),
            _ => return None
        }



        while self.match_(TokenType::COMMA) {

            match self.assignment() {
                Some(expr) => expr_list.push(expr),
                _ => {
                    self.has_errors = true;
                    logger::log_message(logger::LogLevel::ERROR, 
                        self.tokens[self.current].col, self.tokens[self.current].line, 
                        "Expected expression after comma".to_string()
                    );
                    self.synchronize();
                    break;
                }
            }
        } 
        
        return Some(Box::new(Expr::ExprList{expr_list, datatype: Datatype::yet_to_infer}));
        
        // return self.logical_OR_expr();
    }

    fn expression_list(&mut self) -> Vec<Box<Expr>> { 
        eprintln!("In expression_list()");
        let mut expr_list: Vec<Box<Expr>> = vec![];

        match /*self.expression()*/self.logical_OR_expr() {
            Some(expr) => expr_list.push(expr),
            _ => return expr_list,
        }

        while self.match_(TokenType::COMMA) {
            match /*self.expression()*/self.logical_OR_expr() {
                Some(expr) => expr_list.push(expr),
                _ => return expr_list
            }
        }

        return expr_list;
    }

    fn target(&mut self) -> Option<Box<Expr>> { 
        eprintln!("In target()");
        let mut atom: Box<Expr>;
        match self.consume(TokenType::IDENTIFIER, 
            "L-value needs to be either variable or attribute reference".to_string()) {
                Some(tok) => atom = Box::new(Expr::Variable{name: tok, datatype: Datatype::yet_to_infer, struct_name: None}),
                _ => return None,
        }

        while self.match_(TokenType::DOT) {
            match self.consume(TokenType::IDENTIFIER, 
                "L-value needs to be either variable or attribute references".to_string()) {
                    Some(tok) => 
                        atom = Box::new(Expr::AttributeRef{
                            object: atom, 
                            name: tok, 
                            object_dtype: Datatype::yet_to_infer,
                            datatype: Datatype::yet_to_infer}),
                    _ => return None,
            }
        }

        return Some(atom);
    }

    fn target_list(&mut self) -> Vec<Box<Expr>> { 
        eprintln!("In target_list()");
        let mut tar_list: Vec<Box<Expr>> = vec![];

        match self.target() {
            Some(target) => tar_list.push(target),
            _ => return tar_list
        }

        while self.match_(TokenType::COMMA) {
            match self.target() {
                Some(target) => tar_list.push(target),
                _ => return tar_list
            }
        }

        return tar_list;
    }

    fn is_target_list_valid(&mut self, target_list: &Vec<Box<Expr>>) -> bool {
        let mut result: bool = true;
        for target in target_list {
            match **target {
                Expr::AttributeRef{..} => result &= true,
                Expr::Variable{..}      => result &= true,
                // in future - subscription and slicing can also be added
                _ => return false
            }
            
        }
        return result;
    }

    fn is_target_valid(&mut self, target: &Box<Expr>) -> bool {
        match **target {
            Expr::AttributeRef{..} => return true,
            Expr::Variable{..} => return true,
            // in future - subscription and slicing can also be added
            _ => return false
        }
    }

    fn assignment(&mut self) -> Option<Box<Expr>> { 
        eprintln!("In assignment()");
        let lhs = self.logical_OR_expr();
        if lhs.is_none() { return None; }

        if let Some(peek) = self.peek() {
            if TokenType::get_assignment_ops().contains(&peek.tok_type) {
                // IF Borrow checking is supported in future, then a separate branch for `=` needs to be created.
                // all assignment ops except `=` could be considered binary operator(Expr::Binary) 
                // and Expr::Assignment can get rid of operator attribute
                self.advance();
                let operator = peek;
                if let Some(expr) = self.assignment() {
                    match lhs {
                        Some(target) 
                            => {
                                if !self.is_target_valid(&target) {
                                    self.has_errors = true;
                                    log_message(logger::LogLevel::ERROR, 
                                        self.tokens[self.current].col, self.tokens[self.current].line, 
                                        "L-value incorrect. Only variables and attribute refs allowed".to_string());
                                    return None;
                                }
                                return Some(Box::new(Expr::Assignment{target, operator, expr, datatype: Datatype::yet_to_infer}));
                            }
                        _ => {
                            return None;
                        }
                    }
                }
            } 
        }

        return lhs;
    }

    // fn conditional_expr(&self) -> Option<Box<Expr>> { 

    // }

    #[allow(non_snake_case)]
    fn logical_OR_expr(&mut self) -> Option<Box<Expr>> { 
        eprintln!("In logical_OR_expr()");
        let mut lhs: Box<Expr>;

        match self.logical_AND_expr() {
            Some(expr) => lhs = expr,
            _ => return None
        }


        while self.match_(TokenType::K_OR) {
            let operator: Token;
            let rhs: Box<Expr>;
            
            match self.curr() {
                Some(tok) => operator = tok,
                _ => return None,
            }

            match self.logical_AND_expr() {
                Some(expr) => rhs = expr,
                _ => return None
            }

            lhs = Box::new(Expr::Binary{lhs, operator, rhs, datatype: Datatype::bool});
        } 
        
        return Some(lhs);
    }

    #[allow(non_snake_case)]
    fn logical_AND_expr(&mut self) -> Option<Box<Expr>> { 
        eprintln!("In logical_AND_expr()");
        let mut lhs: Box<Expr>;

        match self.inclusive_OR_expr() {
            Some(expr) => lhs = expr,
            _ => return None,
        }

        while self.match_(TokenType::K_AND) {
            let operator: Token;
            let rhs: Box<Expr>;
            
            match self.curr() {
                Some(tok) => operator = tok,
                _ => return None,
            }

            match self.inclusive_OR_expr() {
                Some(expr) => rhs = expr,
                _ => return None,
            }

            lhs = Box::new(Expr::Binary{lhs, operator, rhs, datatype: Datatype::bool});
        }

        return Some(lhs);
    }

    #[allow(non_snake_case)]
    fn inclusive_OR_expr(&mut self) -> Option<Box<Expr>> { 
        eprintln!("In inclusive_OR_expr()");
        let mut lhs: Box<Expr>;

        match self.exclusive_OR_expr() {
            Some(expr) => lhs = expr,
            _ => return None,
        }

        while self.match_(TokenType::BITWISE_OR) {
            let operator: Token;
            let rhs: Box<Expr>;
            
            match self.curr() {
                Some(tok) => operator = tok,
                _ => return None,
            }

            match self.exclusive_OR_expr() {
                Some(expr) => rhs = expr,
                _ => return None,
            }

            lhs = Box::new(Expr::Binary{lhs, operator, rhs, datatype: Datatype::yet_to_infer});
        }

        return Some(lhs);
    }

    #[allow(non_snake_case)]
    fn exclusive_OR_expr(&mut self) -> Option<Box<Expr>> { 
        eprintln!("In exclusive_OR_expr()");
        let mut lhs: Box<Expr>;

        match self.AND_expr() {
            Some(expr) => lhs = expr,
            _ => return None,
        }

        while self.match_(TokenType::BITWISE_XOR) {
            let operator: Token;
            let rhs: Box<Expr>;
            
            match self.curr() {
                Some(tok) => operator = tok,
                _ => return None,
            }

            match self.AND_expr() {
                Some(expr) => rhs = expr,
                _ => return None,
            }

            lhs = Box::new(Expr::Binary{lhs, operator, rhs, datatype: Datatype::yet_to_infer});
        }

        return Some(lhs);
    }

    #[allow(non_snake_case)]
    fn AND_expr(&mut self)  -> Option<Box<Expr>> { 
        eprintln!("In AND_expr()");
        let mut lhs: Box<Expr>;

        match self.equality_expr() {
            Some(expr) => lhs = expr,
            _ => return None,
        }

        while self.match_(TokenType::BITWISE_AND) {
            let operator: Token;
            let rhs: Box<Expr>;
            
            match self.curr() {
                Some(tok) => operator = tok,
                _ => return None,
            }

            match self.equality_expr() {
                Some(expr) => rhs = expr,
                _ => return None,
            }

            lhs = Box::new(Expr::Binary{lhs, operator, rhs, datatype: Datatype::yet_to_infer});
        }

        return Some(lhs);
    }

    fn equality_expr(&mut self) -> Option<Box<Expr>> {
        eprintln!("In equality_expr()"); 
        let mut lhs: Box<Expr>;

        match self.relational_expr() {
            Some(expr) => lhs = expr,
            _ => return None,
        }

        while self.match_multi(TokenType::get_equality_ops()) {
            let operator: Token;
            let rhs: Box<Expr>;
            
            match self.curr() {
                Some(tok) => operator = tok,
                _ => return None,
            }

            match self.relational_expr() {
                Some(expr) => rhs = expr,
                _ => return None,
            }

            lhs = Box::new(Expr::Binary{lhs, operator, rhs, datatype: Datatype::bool});
        }

        return Some(lhs);
    }

    fn relational_expr(&mut self) -> Option<Box<Expr>> { 
        eprintln!("In relational_expr()");
        let mut lhs: Box<Expr>;

        match self.shift_expr() {
            Some(expr) => lhs = expr,
            _ => return None,
        }

        while self.match_multi(TokenType::get_relational_ops()) {
            let operator: Token;
            let rhs: Box<Expr>;
            
            match self.curr() {
                Some(tok) => operator = tok,
                _ => return None,
            }

            match self.shift_expr() {
                Some(expr) => rhs = expr,
                _ => return None,
            }

            lhs = Box::new(Expr::Binary{lhs, operator, rhs, datatype: Datatype::bool});
        }

        return Some(lhs);
    }

    fn shift_expr(&mut self) -> Option<Box<Expr>> { 
        eprintln!("In shift_expr()");
        let mut lhs: Box<Expr>;

        match self.additive_expr() {
            Some(expr) => lhs = expr,
            _ => return None,
        }

        while self.match_multi(TokenType::get_shift_ops()) {
            let operator: Token;
            let rhs: Box<Expr>;
            
            match self.curr() {
                Some(tok) => operator = tok,
                _ => return None,
            }

            match self.additive_expr() {
                Some(expr) => rhs = expr,
                _ => return None,
            }

            lhs = Box::new(Expr::Binary{lhs, operator, rhs, datatype: Datatype::yet_to_infer});
        }

        return Some(lhs);
    }

    fn additive_expr(&mut self) -> Option<Box<Expr>> { 
        eprintln!("In additive_expr()");
        let mut lhs: Box<Expr>;

        match self.multiplicative_expr() {
            Some(expr) => lhs = expr,
            _ => return None,
        }

        while self.match_multi(TokenType::get_additive_ops()) {
            let operator: Token;
            let rhs: Box<Expr>;
            
            match self.curr() {
                Some(tok) => operator = tok,
                _ => return None,
            }

            match self.multiplicative_expr() {
                Some(expr) => rhs = expr,
                _ => return None,
            }

            lhs = Box::new(Expr::Binary{lhs, operator, rhs, datatype: Datatype::yet_to_infer});
        }

        return Some(lhs);
    }

    fn multiplicative_expr(&mut self) -> Option<Box<Expr>> { 
        eprintln!("In multiplicative_expr()");
        let mut lhs: Box<Expr>;

        match self.unary_expr() {
            Some(expr) => lhs = expr,
            _ => return None,
        }

        while self.match_multi(TokenType::get_multiplicative_ops()) {
            let operator: Token;
            let rhs: Box<Expr>;
            
            match self.curr() {
                Some(tok) => operator = tok,
                _ => return None,
            }

            match self.unary_expr() {
                Some(expr) => rhs = expr,
                _ => return None,
            }

            lhs = Box::new(Expr::Binary{lhs, operator, rhs, datatype: Datatype::yet_to_infer});
        }

        return Some(lhs);
    }

    fn unary_expr(&mut self) -> Option<Box<Expr>> { 
        eprintln!("In unary_expr()");
        if self.match_multi(TokenType::get_unary_ops()) {
            let operator: Token;
            let operand: Box<Expr>;

            match self.curr() {
                Some(tok) => operator = tok,
                _ => return None,
            }

            match self.unary_expr() {
                Some(expr) => operand = expr,
                _ => return None,
            }

            return Some(Box::new(Expr::Unary{operator, operand, datatype: Datatype::yet_to_infer}));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Option<Box<Expr>> { 
        eprintln!("In primary()");
        let mut atom: Box<Expr>;
        match self.atom() {
            Some(expr) => atom = expr,
            _ => return None,
        }

        loop {
            eprintln!("In primary loop");
            if self.match_(TokenType::DOT) {
                eprintln!("In primary loop: attributeref");
                // attributeref
                match self.consume(TokenType::IDENTIFIER, 
                    "Expected an identifier after '.'".to_string()) {
                        Some(tok) => 
                            atom = Box::new(
                                Expr::AttributeRef{
                                    object: atom, 
                                    name: tok, 
                                    object_dtype: Datatype::yet_to_infer,
                                    datatype: Datatype::yet_to_infer
                                }),
                        _ => return Some(atom)
                }

            } else if self.match_(TokenType::BRACKET_OPEN) {
                eprintln!("In primary loop: call");
                let arguments = self.expression_list();
                self.consume(TokenType::BRACKET_CLOSE, "Expected ')' after expressions list".to_string());
                atom = Box::new(Expr::Call{callee: atom, arguments, datatype: Datatype::yet_to_infer});
                // if let Some(arguments) = self.expression() {
                //     atom = Box::new(Expr::Call{callee: atom, arguments, datatype: Datatype::yet_to_infer});
                //     self.consume(TokenType::BRACKET_CLOSE, "Expected ')' after expressions list".to_string());

                // } else {
                //     self.consume(TokenType::BRACKET_CLOSE, "Expected ')' after expressions list".to_string());
                //     return Some(atom);
                // }
            } else if self.match_(TokenType::K_AS) {
                eprintln!("In primary loop: casting");
                match self.consume_multi(TokenType::get_datatypes(), 
                "Expected a datatype after 'as' keyword for type casting".to_string()) {
                    Some(tok) => 
                        atom = Box::new(
                            Expr::Cast{
                                variable:atom, 
                                cast_type: tok, 
                                from_dtype: Datatype::yet_to_infer, 
                                to_dtype: Datatype::yet_to_infer
                            }
                        ),
                    _ => return Some(atom)
                }
            } else {
                break;
                // return Some(atom);
            }
        }

        return Some(atom);
    }
    
    fn atom(&mut self) -> Option<Box<Expr>> { 
        eprintln!("In atom()");
        if let Some(peek) = self.peek() {
            eprintln!("Peek: {:?}", peek);
            if peek.tok_type == TokenType::BRACKET_OPEN                          { return self.grouping(); }
            else if TokenType::get_literal_types().contains(&peek.tok_type)      { return self.literal(); }
            else if peek.tok_type == TokenType::IDENTIFIER {

                if self.restrictions.contains(&Restriction::STRUCT_EXPR) {
                    return self.variable();
                }

                // check if it could be a struct-expr
                if let Some(next_peek) = self.peek_next() {
                   if next_peek.tok_type == TokenType::CURLY_OPEN {
                        return self.struct_expr();
                    }
                }

                return self.variable();
            } 
        }  
        eprintln!("Returning none from atom");  
        return None;
    }

    fn grouping(&mut self) -> Option<Box<Expr>> { 
        eprintln!("In grouping()");
        let mut group: Option<Box<Expr>> = None;
        match self.consume(TokenType::BRACKET_OPEN, 
            "Expected '(' at the starting of paranthesized expression".to_string()) {
                Some(_) => (),
                _ => return None,
        }

        match self.expression() {
            Some(expr) => group = Some(Box::new(Expr::Grouping{expr, datatype: Datatype::yet_to_infer})),
            _ => (),
        }

        self.consume(TokenType::BRACKET_CLOSE, "Expected ')' at the end of paranthesized expression".to_string());
        return group;
    }

    fn variable(&mut self) -> Option<Box<Expr>> { 
        eprintln!("In variable()");
        match self.advance() {
            Some(tok) if tok.tok_type == TokenType::IDENTIFIER 
                => Some(Box::new(Expr::Variable{name: tok, datatype: Datatype::yet_to_infer, struct_name: None})),
            _ => None
        }
    }

    fn literal(&mut self) -> Option<Box<Expr>> { 
        eprintln!("In literal()");
        match self.advance() {
            Some(tok) if TokenType::get_literal_types().contains(&tok.tok_type)
                => Some(Box::new(Expr::Literal{value: tok, datatype: Datatype::yet_to_infer})),
            _ => None
        }
    }

    fn struct_expr(&mut self) -> Option<Box<Expr>> { 
        eprintln!("In struct_expr()");
        let name: Token;
        match self.consume(TokenType::IDENTIFIER, 
            "Expected identifier in struct expression".to_string()) {
                Some(tok) => name = tok,
                _ => { self.synchronize(); return None;},
        }

        match self.consume(TokenType::CURLY_OPEN, 
            "Expected '{' after identifier in struct expression".to_string()) {
                Some(_) => (),
                _ => return Some(Box::new(Expr::Variable{name, datatype: Datatype::yet_to_infer, struct_name: None}))
                // _ => return None,
        }

        
        let mut fields: Vec<(Token, Box<Expr>)> = vec![];
        while !self.match_(TokenType::CURLY_CLOSE) {
            let field_name: Token;
            let field_value: Box<Expr>;

            match self.consume(TokenType::IDENTIFIER, 
                "Expected field name in struct expression".to_string()) {
                    Some(tok) => field_name = tok,
                    _ => { self.synchronize(); return None;},
                    // _ => {
                    //     while self.curr() == Some(name) {
                    //         self.current-=1;
                    //     }
                    //     return Some(Box::new(Expr::Variable{name, datatype: Datatype::yet_to_infer, struct_name: None}));
                    // }
            }

            match self.consume(TokenType::COLON, 
                "Expected ':' after field name in struct expression".to_string()) {
                    Some(_) => (),
                    // _ => return Some(Box::new(Expr::Variable{name, datatype: Datatype::yet_to_infer, struct_name: None}))
                    _ => { self.synchronize(); return None;},
            }

            // match self.expression() {
            match self.assignment() {
                Some(expr) => field_value = expr,
                _ => {
                    self.synchronize();
                    // return None;
                    return Some(Box::new(Expr::Variable{name, datatype: Datatype::yet_to_infer, struct_name: None}));
                }
            }

            fields.push((field_name, field_value));

            if !self.match_(TokenType::COMMA) {
                if let Some(peek) = self.peek() {
                    if peek.tok_type != TokenType::CURLY_CLOSE {
                        self.has_errors = true;
                        logger::log_message(logger::LogLevel::ERROR, peek.col, peek.line, 
                            "Expected ',' or '}' after field expression".to_string());
                        return None;
                    }
                }
            }
        }

        return Some(Box::new(Expr::StructExpr{struct_name: name, fields, datatype: Datatype::yet_to_infer}));
    }

    

    // fn attributeref(&self) -> Option<Box<Expr>> { unimplemented!() }

    // fn call(&self) -> Option<Box<Expr>> { unimplemented!() }
    

    pub fn parse(&mut self, tokens: Vec<Token>) -> (Vec<Box<Decl>>, bool) {
        self.tokens = tokens;
        self.current = 0;
        self.start = 0;
        
        let mut declarations: Vec<Box<Decl>> = vec![];
        while !self.is_end() {
            eprintln!("Calling declaration");
            match self.declaration() {
                Some(decl) => declarations.push(decl),
                _ => ()
            }
        }
        
        return (declarations, self.has_errors);
        // drop(tokens);
        // return vec![];
    }

    pub fn new() -> Self {
        Parser { current: 0, start: 0, tokens: vec![], restrictions: vec![], has_errors: false }
    }
}