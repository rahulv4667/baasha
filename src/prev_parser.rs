// use std::fs::OpenOptions;
// use std::ptr::null;

// use crate::{ast::*, logger};
// use crate::globals::TokenType;
// use crate::lexer::Token;

// #[allow(non_camel_case_types,dead_code)]
// #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
// enum Precedence {
//     NONE,
//     ASSIGNMENT,    // =, +=, -=, *=, /=, >>=, <<=, &=, |=
//     OR,            // or
//     AND,           // and
//     BITWISE_OR,    // |
//     BITWISE_XOR,   // ^
//     BITWISE_AND,   // &
//     EQUALITY,      // == !=
//     COMPARISION,   // <, >, <=, >=
//     TERM,          // +, -
//     FACTOR,        // * / 
//     UNARY,         // ! - ~
//     CALL,          // . ()
//     PRIMARY
// }

// #[allow(dead_code)]
// impl Precedence {
//     fn next_higher(&self) -> Self {
//         return match self {
//             Self::NONE              => Self::ASSIGNMENT,
//             Self::ASSIGNMENT        => Self::OR,
//             Self::OR                => Self::AND,
//             Self::AND               => Self::BITWISE_OR,
//             Self::BITWISE_OR        => Self::BITWISE_XOR,
//             Self::BITWISE_XOR       => Self::BITWISE_AND,
//             Self::BITWISE_AND       => Self::EQUALITY,
//             Self::EQUALITY          => Self::COMPARISION,
//             Self::COMPARISION       => Self::TERM,
//             Self::TERM              => Self::FACTOR,
//             Self::FACTOR            => Self::UNARY,
//             Self::UNARY             => Self::CALL,
//             Self::CALL              => Self::PRIMARY,
//             Self::PRIMARY           => Self::PRIMARY,
//             // _                       => Precedence::NONE, 
//         };
//     }
// }

// type PrefixFn = Option<for<'r> fn(&'r mut Parser, bool)->  Option<Box<Expr>>>;
// type InfixFn  = Option<for<'r> fn(&'r mut Parser, Option<Box<Expr>>)->Option<Box<Expr>>>; 

// #[derive(Clone)]
// struct ParseRule {
//     infix_func:     InfixFn ,
//     prefix_func:    PrefixFn,
//     precedence:     Precedence
// }


// impl ParseRule {


//     fn null_parse_rule_with_precedence(precedence: Precedence) -> Self {
//         return ParseRule{
//             prefix_func: None,
//             infix_func: None,
//             precedence
//         };
//     }

//     fn get_rule(tok_type: TokenType) -> Self {
        
//         let call:   Option<for<'r> fn(&'r mut Parser, Option<Box<Expr>>) -> _> = Some(Parser::call);
//         let binary: Option<for<'r> fn(&'r mut Parser, Option<Box<Expr>>) -> _> = Some(Parser::binary);

//         let grouping:   Option<for<'r> fn(&'r mut Parser, bool)   ->  Option<Box<Expr>>> = Some(Parser::grouping);
//         let unary:      Option<for<'r> fn(&'r mut Parser, bool)   ->  Option<Box<Expr>>> = Some(Parser::unary);
//         let variable:   Option<for<'r> fn(&'r mut Parser, bool)   ->  Option<Box<Expr>>> = Some(Parser::variable);
//         let primary:    Option<for<'r> fn(&'r mut Parser, bool)   ->  Option<Box<Expr>>> = Some(Parser::primary);

//         let null_parse_rule = ParseRule{prefix_func: None, infix_func: None, precedence: Precedence::NONE};


//         return match tok_type {
//             TokenType::BRACKET_OPEN => ParseRule{prefix_func: grouping, infix_func: call, precedence: Precedence::NONE},

//             TokenType::BRACKET_CLOSE    => null_parse_rule,
//             TokenType::CURLY_OPEN       => null_parse_rule,
//             TokenType::CURLY_CLOSE      => null_parse_rule,
//             TokenType::COMMA            => null_parse_rule,
//             // TokenType::DOT              => 
//             TokenType::MINUS            => ParseRule{prefix_func: unary, infix_func: binary, precedence: Precedence::TERM},
//             TokenType::MINUS_EQUAL      => Self::null_parse_rule_with_precedence(Precedence::ASSIGNMENT),
//             TokenType::PLUS             => ParseRule{prefix_func: None, infix_func: binary, precedence: Precedence::TERM},
//             TokenType::PLUS_EQUAL       => Self::null_parse_rule_with_precedence(Precedence::ASSIGNMENT),
//             TokenType::SEMICOLON        => null_parse_rule,
//             TokenType::SLASH            => ParseRule{prefix_func: None, infix_func: binary, precedence: Precedence::FACTOR},
//             TokenType::SLASH_EQUAL      => ParseRule{prefix_func: None, infix_func: binary, precedence: Precedence::ASSIGNMENT},
//             TokenType::ASTERISK         => ParseRule{prefix_func: None, infix_func: binary, precedence: Precedence::FACTOR},
//             TokenType::ASTERISK_EQUAL   => ParseRule{prefix_func: None, infix_func: binary, precedence: Precedence::ASSIGNMENT},
//             TokenType::BANG             => ParseRule{prefix_func: unary, infix_func: None, precedence: Precedence::NONE},
//             TokenType::BANG_EQUAL       => ParseRule{prefix_func: None, infix_func: binary, precedence: Precedence::EQUALITY},
//             TokenType::EQUAL            => ParseRule{prefix_func: None, infix_func: None, precedence: Precedence::NONE},
//             TokenType::EQUAL_EQUAL      => ParseRule{prefix_func: None, infix_func: binary, precedence: Precedence::EQUALITY},
//             TokenType::GREAT_THAN       => ParseRule{prefix_func: None, infix_func: binary, precedence: Precedence::COMPARISION},
//             TokenType::GREAT_EQUAL      => ParseRule{prefix_func: None, infix_func: binary, precedence: Precedence::COMPARISION},
//             TokenType::LESS_THAN        => ParseRule{prefix_func: None, infix_func: binary, precedence: Precedence::COMPARISION},
//             TokenType::LESS_EQUAL       => ParseRule{prefix_func: None, infix_func: binary, precedence: Precedence::COMPARISION},
//             TokenType::IDENTIFIER       => ParseRule{prefix_func: variable, infix_func: None, precedence: Precedence::NONE},
//             TokenType::STRING_LITERAL   => ParseRule{prefix_func: primary, infix_func: None, precedence: Precedence::NONE},
//             TokenType::INT_LITERAL      => ParseRule{prefix_func: primary, infix_func: None, precedence: Precedence::NONE},
//             TokenType::FLOAT_LITERAL    => ParseRule{prefix_func: primary, infix_func: None, precedence: Precedence::NONE},
//             TokenType::HEX_LITERAL      => ParseRule{prefix_func: primary, infix_func: None, precedence: Precedence::NONE},
//             TokenType::OCTAL_LITERAL    => ParseRule{prefix_func: primary, infix_func: None, precedence: Precedence::NONE},
//             TokenType::K_AND            => ParseRule{prefix_func: None, infix_func: binary, precedence: Precedence::AND},
//             TokenType::K_OR             => ParseRule{prefix_func: None, infix_func: binary, precedence: Precedence::OR},
//             TokenType::K_FALSE          => ParseRule{prefix_func: primary, infix_func: None, precedence: Precedence::NONE},
//             TokenType::K_TRUE           => ParseRule{prefix_func: primary, infix_func: None, precedence: Precedence::NONE},
//             _                           => null_parse_rule
//         }
//     }
// }



// #[allow(dead_code)]
// pub struct Parser {
//     current:    usize,
//     start:      usize,
//     tokens:     Vec<Token>
// }

// #[allow(dead_code)]
// impl Parser {

//     fn match_(&mut self, ttype: TokenType) -> bool {
//         if self.current >= self.tokens.len() { return false; }
//         if let Some(Token{tok_type,..}) = self.peek()  {
//             if tok_type == ttype {
//                 if let Some(Token { tok_type,..}) = self.advance() {
//                     return tok_type == ttype;
//                 }
//             }
//         } 
//         return false;
//     }

//     fn match_multi(&mut self, tok_types: Vec<TokenType>) -> bool {
//         if self.current >= self.tokens.len() {
//             return false;
//         }

//         for ttype in tok_types {

//             if let Some(Token{tok_type,..}) = self.peek()  {
//                 if tok_type == ttype {
//                     if let Some(Token { tok_type,..}) = self.advance() {
//                         return tok_type == ttype;
//                     }
//                 }
//             } 
//         }

//         return false;
//     }

//     fn is_end(&self) -> bool {
//         return self.current >= self.tokens.len() || 
//             match self.peek() { 
//                 Some(Token{tok_type, ..}) => tok_type  == TokenType::FILE_EOF,
//                 None => false,
//             };
//     }


//     fn advance(&mut self) -> Option<Token> {
//         if !self.is_end() { self.current += 1; }
//         return Some(self.tokens[self.current - 1].clone());
//     }

//     fn peek(&self) -> Option<Token> {
//         return Some(self.tokens[self.current].clone());
//     }

//     fn get_peek_parse_rule(&self) -> ParseRule {
//         if let Some(peek) = self.peek() {
//             return ParseRule::get_rule(peek.tok_type);
//         }
         
//         return ParseRule::get_rule(TokenType::ERROR);
//     }


//     fn peek_next(&self) -> Option<Token> {
//         if self.is_end() || self.tokens[self.current + 1].tok_type == TokenType::FILE_EOF {
//             logger::log_message(logger::LogLevel::CRASH, 
//                 self.tokens[self.current+1].col, self.tokens[self.current+1].line, 
//                 "Reached EOF before completing parsing. Probable incomplete expression.".to_string());
//             return None;            
//         }
//         return Some(self.tokens[self.current + 1].clone());
//     }

//     fn curr(&self) -> Option<Token> {
//         if self.is_end() {
//             logger::log_message(logger::LogLevel::CRASH,
//                  self.tokens[self.current].col, self.tokens[self.current].line, 
//                 "Reached EOF before completing parsing. Probable incomplete expression".to_string());
//             return None;
//         }

//         return Some(self.tokens[self.current - 1].clone());
//     }


//     fn consume(&mut self, ttype: TokenType, message: String, show_error: bool) -> Option<Token> {
//         if self.match_(ttype) { return self.curr(); }

//         if show_error {
//             logger::log_message(logger::LogLevel::ERROR, 
//                 self.tokens[self.current].col, self.tokens[self.current].line, message);
//         }
//         return None;
//     }

//     fn consume_multi(&mut self, ttypes: Vec<TokenType>, msg: String, show_error: bool) -> Option<Token> {
//         if self.match_multi(ttypes) { return self.curr(); }

//         if msg.len() >=1 && show_error {
//             logger::log_message(logger::LogLevel::ERROR, 
//                 self.tokens[self.current].col, self.tokens[self.current].line, msg);
//         }
//         return None;
//     }


//     fn check(&self, ttype: TokenType) -> bool {
//         if self.is_end() { return false; }
//         return match self.peek() {
//             None    => false,
//             Some(Token{tok_type, ..})   => tok_type == ttype
//         }
//     }

//     fn declaration(&mut self)   -> Option<Box<Decl>> {
//         if      self.match_(TokenType::K_FUNC)   { return self.func(); } 
//         else if self.match_(TokenType::K_STRUCT) { return self.struct_decl(); }
//         else if self.match_(TokenType::K_IMPL)   { return self.impl_decl(); }
//         else if self.match_(TokenType::K_TRAIT)  { return self.trait_decl(); }
//         // else if self.match_(TokenType::SEMICOLON){ return None; }
//         else    { return None; }     
//     }


//     fn prototype(&mut self) -> Option<Box<Decl>> {
//         let name: Token;
//         let msg: String = String::from("Expected identifier after 'func' keyword");
//         match self.consume(TokenType::IDENTIFIER, msg, true) {
//             None    => return None,
//             Some(tok)   => name = tok
//         };

//         match self.consume(TokenType::BRACKET_OPEN, 
//             "Expected '(' after function identifier.".to_string(), true) {
//                 None        => return None,
//                 Some(_)     => {}
//         };

//         let mut params: Vec<(Token, Token)> = Vec::new();
//         let mut param_name: Vec<Token>  = Vec::new();
//         let dtypes_vec = vec![TokenType::K_INT8, TokenType::K_INT16, TokenType::K_INT32,
//             TokenType::K_INT64, TokenType::K_UINT8, TokenType::K_UINT16, TokenType::K_UINT32,
//             TokenType::K_UINT64, TokenType::K_FLOAT32, TokenType::K_FLOAT64, TokenType::K_BOOL, TokenType::IDENTIFIER];


//         // parsing parameters
//         while !self.match_(TokenType::BRACKET_CLOSE) {
//             while self.match_(TokenType::IDENTIFIER) {
                
//                 match self.curr() {
//                     Some(tok)   => param_name.push(tok),
//                     None              => {}
//                 };

//                 if self.match_(TokenType::COMMA) { continue; }
//                 else { break; }
//             }

//             if self.match_(TokenType::COLON) {

//                 match self.consume_multi(dtypes_vec.clone(), 
//                     "Expected datatype after parameters".to_string(), true) {
//                     None =>  return None,
//                     Some(tok) => {
//                         for name in &param_name {
//                             params.push((name.clone(), tok.clone()));
//                         }
//                     } 
//                 }

//             } else {
//                 logger::log_message(logger::LogLevel::ERROR, 
//                     self.tokens[self.current].col, self.tokens[self.current].line, 
//                     "Expected either ':' or ',' after function parameter".to_string());
//                 return None;
//             }
//         }


//         let mut returntypes: Vec<Token> = Vec::new();
//         // parsing return types.
//         while self.match_multi(dtypes_vec.clone()) {
//             match self.curr() {
//                 Some(tok)   => returntypes.push(tok),
//                 None    => ()
//             };

//             if self.match_(TokenType::COMMA) { continue; }
//             else { break; }
//         }

//         return Some(Box::new(Decl::Prototype{name, parameters: params, returntypes}));
//     }

//     fn func(&mut self)  -> Option<Box<Decl>> {
//         let prototype : Box<Decl>;
//         match self.prototype() {
//             Some(decl)  => prototype = decl,
//             None => return None
//         };

//         match self.peek() {
//             Some(Token{tok_type:TokenType::CURLY_OPEN, ..}) => {},
//             _ => return Some(prototype)
//         };

//         let block: Box<Stmt>;
//         match self.block() {
//             Some(decl)  => block = decl,
//             None => return Some(prototype)
//         };

//         return Some(Box::new(Decl::FuncDef{prototype, block}));
//     } 

//     fn struct_decl(&mut self)   -> Option<Box<Decl>> {
//         let name: Token;
//         let mut fields: Vec<(Token, Token)> = Vec::new();
//         let dtypes_vec = vec![TokenType::K_INT8, TokenType::K_INT16, TokenType::K_INT32,
//             TokenType::K_INT64, TokenType::K_UINT8, TokenType::K_UINT16, TokenType::K_UINT32,
//             TokenType::K_UINT64, TokenType::K_FLOAT32, TokenType::K_FLOAT64, TokenType::K_BOOL, TokenType::IDENTIFIER];

//         let msg = String::from("Expected identifier after 'struct' keyword");
//         match self.consume(TokenType::IDENTIFIER, msg, true) {
//             None => return None,
//             Some(tok) => name = tok
//         };

//         match self.consume(TokenType::CURLY_OPEN, 
//             "Expected '{' after identifer for struct".to_string(), true) {
//                 None => return None,
//                 Some(_)     => {}
//         };

//         while !self.match_(TokenType::CURLY_CLOSE) {
//             let field_name: Token;
//             let field_type: Token;
//             if self.match_(TokenType::IDENTIFIER) {
//                 match self.curr() {
//                     None    => continue,
//                     Some(tok) => field_name = tok
//                 }
//             } else {
//                 logger::log_message(logger::LogLevel::ERROR, 
//                     self.tokens[self.current].col, self.tokens[self.current].line, 
//                     "Expected field name in struct".to_string());
//                 continue;
//             }

//             self.consume(TokenType::COLON, "Expected ':' after field name".to_string(), true);

//             if self.match_multi(dtypes_vec.clone()) {
//                 match self.curr() {
//                     None => continue,
//                     Some(tok)   => field_type = tok
//                 }
//             } else {
//                 logger::log_message(logger::LogLevel::ERROR, 
//                     self.tokens[self.current].col, self.tokens[self.current].line, 
//                     "Expected field type after field name in struct".to_string());

//                 continue;
//             }

//             self.consume(TokenType::COMMA, "Expected ',' after field declaration".to_string(),
//                  true);

//             fields.push((field_name, field_type));
//         }

//         return Some(Box::new(Decl::StructDecl{name, fields}));

//         // return None;
//     }

//     fn impl_decl(&mut self)     -> Option<Box<Decl>> {
//         let name: Token;
//         let mut trait_name: Option<Token>;

//         let msg = String::from("Expected identifier after 'impl' keyword");
//         trait_name = self.consume(TokenType::IDENTIFIER, msg, true);

//         let msg = String::from("Expected identifier after 'impl-for' phrase");
//         match self.consume(TokenType::K_FOR, "".to_string(), false) {
//             Some(_)     => 
//                 match self.consume(TokenType::IDENTIFIER,msg, true) {
//                     Some(tok)       => name = tok,
//                     None                  => return None
//                 },
//             None                => 
//                 match trait_name {
//                     None    => return None,
//                     Some(tok)   => {
//                         name = tok;
//                         trait_name = None
//                     }
//                 }
//         };

//         match self.consume(TokenType::CURLY_OPEN, 
//             "Expected '{' after 'impl-for' phrase".to_string(), true) {
//                 None => return None,
//                 Some(_) => {}
//         };

//         let mut funcs: Vec<Box<Decl>> = Vec::new();
//         while !self.match_(TokenType::CURLY_CLOSE) {
//             match self.consume(TokenType::K_FUNC, 
//                 "Expected function definition inside 'impl' block".to_string(), false) {
//                 Some(_) => match self.func() {
//                     Some(decl)  => funcs.push(decl),
//                     None => continue
//                 },

//                 None    => continue
//             };
//         };

//         return Some(Box::new(Decl::ImplDecl{name, trait_name, funcs}));
//     }

//     fn trait_decl(&mut self)    -> Option<Box<Decl>> {
//         let name: Token;

//         let msg: String = String::from("Expected identifier after 'trait' keyword."); 
//         match self.consume(TokenType::IDENTIFIER, msg, true) {
//             None        => return None,
//             Some(tok)     => name = tok
//         };


//         match self.consume(TokenType::CURLY_OPEN, 
//             "Expected '{' after 'trait' declaration".to_string(), true) {
//                 None                => return None,
//                 Some(_)     => {}
//         };

//         let mut funcs: Vec<Box<Decl>> = Vec::new();
//         while !self.match_(TokenType::CURLY_CLOSE) {
//             match self.consume(TokenType::K_FUNC, 
//                 "Expected function definition/declaration in 'trait' block".to_string(), true) {
//                     None => continue,
//                     Some(_)  => match self.func() {
//                         Some(decl)  => funcs.push(decl),
//                         None                 => continue,
//                     }
//             };
//         }

//         return Some(Box::new(Decl::TraitDecl{name, funcs}));
//     }


//     fn block(&mut self) -> Option<Box<Stmt>> {
//         // currently has it parsing from '{'. If removing that part, make according changes in `self.func()`
//         self.consume(TokenType::CURLY_OPEN, "Expected '{' at the starting of a block".to_string(), true);

//         let mut stmts: Vec<Box<Stmt>> = Vec::new();
//         while !self.match_(TokenType::CURLY_CLOSE) {
//             match self.statement() {
//                 Some(stmt)  => stmts.push(stmt),
//                 None => ()
//             }
//         }

//         return Some(Box::new(Stmt::Block{statements: stmts}));
//     }

//     fn statement(&mut self) -> Option<Box<Stmt>> {
//         if self.match_(TokenType::K_IF)            { return self.if_stmt(); }
//         else if self.match_(TokenType::K_RETURN)   { return self.return_stmt(); }
//         else if self.match_(TokenType::K_FOR)      { return self.for_stmt(); }
//         else if self.match_(TokenType::CURLY_OPEN) { return self.block(); }
//         else { return self.expr_stmt(); }
//     }

//     fn if_stmt(&mut self) -> Option<Box<Stmt>> { unimplemented!() }
//     fn return_stmt(&mut self)   -> Option<Box<Stmt>> { unimplemented!() }
//     fn for_stmt(&mut self)  -> Option<Box<Stmt>>    { unimplemented!() }

//     fn expr_stmt(&mut self) -> Option<Box<Stmt>> {
//         return match self.expression() {
//             Some(expr) => Some(Box::new(Stmt::Expression{expr})),
//             None => None
//         };
//     }

//     fn expression(&mut self) -> Option<Box<Expr>> {
//         return self.parse_precedence(Precedence::ASSIGNMENT);
//     }

//     fn parse_precedence(&mut self, precedence: Precedence) -> Option<Box<Expr>> {

//         let prefix_rule: PrefixFn = self.get_peek_parse_rule().prefix_func;


//             // let prefix_rule: PrefixFn = ParseRule::get_rule(peek.tok_type).prefix_func;
//         let pref_rule: for<'r> fn(&'r mut Parser, bool)->  Option<Box<Expr>>; 
//         match prefix_rule {
//             None => return None,
//             Some(rl) => pref_rule = rl
//         };
            
//         let tok = self.advance();

//         let can_assign: bool = precedence <= Precedence::ASSIGNMENT;
//         let mut expr = pref_rule(self, can_assign);


//         while precedence <= self.get_peek_parse_rule().precedence {
//             let tok: Token;
//             match self.advance() {
//                 None => tok = Token{tok_type: TokenType::ERROR, value:String::new(), line: usize::MAX, col: usize::MAX},
//                 Some(token) => tok = token,
//             };

//             let infix_rule: InfixFn = ParseRule::get_rule(tok.tok_type).infix_func;
//             let inf_rule: for<'r> fn(&'r mut Parser, Option<Box<Expr>>) -> Option<Box<Expr>>;
            
//             match infix_rule {
//                 None => return expr,
//                 Some(rl) => inf_rule = rl,
//             };

//             expr = inf_rule(self, expr);
//         }

//         if can_assign && self.match_(TokenType::EQUAL) {
//             logger::log_message(logger::LogLevel::ERROR, 
//                 self.tokens[self.current].col, self.tokens[self.current].line, 
//                 "Invalid assignment target".to_string());
//             return None;
//         }
        
//         return expr;
//     }


//     #[allow(unused)]
//     fn unary(&mut self, can_assign: bool)       ->  Option<Box<Expr>>     { 
//         let operator: Token;
//         if let Some(oprtr) = self.curr() {
//             operator = oprtr;
//         } else { return None; }

//         let operand: Box<Expr>;

//         if let Some(oprnd) = self.parse_precedence(Precedence::UNARY) {
//             operand = oprnd;
//         } else  { return None; }

//         let datatype = Datatype::yet_to_infer;
//         return Some(Box::new(Expr::Unary{operator, operand, datatype}));
//     }


//     fn grouping(&mut self, can_assign: bool)    ->  Option<Box<Expr>>     { 
//         unimplemented!()
//     }
    
//     fn variable(&mut self, can_assign: bool)    ->  Option<Box<Expr>>     { unimplemented!() }
    
//     fn primary(&mut self, can_assign: bool)     ->  Option<Box<Expr>>     { 
//         let value: Token;
//         if let Some(val) = self.curr() {
//             let next: Token;
//             if let Some(nxt) = self.peek() {
//                 if nxt.tok_type == TokenType::CURLY_OPEN && val.tok_type == TokenType::IDENTIFIER {
//                     return self.struct_expr();
//                 }
//             }
//             value = val;
//         } else {
//             return None;
//         }

//         let ptype: Primary_Type;
//         let dtype: Datatype;
//         match value.tok_type {
//             TokenType::HEX_LITERAL => {dtype=Datatype::int64; ptype=Primary_Type::Hex_literal},
//             TokenType::OCTAL_LITERAL => {dtype=Datatype::int64; ptype = Primary_Type::Octal_literal},
//             TokenType::INT_LITERAL => {dtype=Datatype::int64; ptype = Primary_Type::Int_literal},
//             TokenType::IDENTIFIER => {dtype=Datatype::yet_to_infer; ptype = Primary_Type::Identifier},
//             TokenType::K_TRUE|TokenType::K_FALSE => {dtype= Datatype::bool; ptype = Primary_Type::Bool_literal},
//             TokenType::STRING_LITERAL => {dtype=Datatype::string; ptype=Primary_Type::String_literal},
//             TokenType::FLOAT_LITERAL => {dtype=Datatype::float64; ptype=Primary_Type::Float_literal},
//             _   => return None
//         }

//         return Some(Box::new(Expr::Primary{ptype, value, datatype: dtype}));
//     }

//     fn struct_expr(&mut self) -> Option<Box<Expr>> {
//         // assumes it is already checked for validity of `Identifier {` with pointer pointing to `Identifier`
//         let name: Token;
//         if let Some(val) = self.curr() {
//             // name = Token{ tok_type: TokenType::IDENTIFIER, ..val};
//             name = val;
//         } else { return None; }


//         let mut fields: Vec<(Token, Box<Expr>)> = Vec::new();
//         if self.match_(TokenType::CURLY_OPEN) {
//             while !self.match_(TokenType::CURLY_CLOSE) {
//                 if let Some(field_name) = self.consume(TokenType::IDENTIFIER, "".to_string(), false) {

//                     self.consume(TokenType::COLON, "Expected ':' after identifier in struct-expr".to_string(), true);

//                     if let Some(expr) = self.expression()  {
                        
//                         fields.push((field_name, expr));
//                     }

//                     self.consume(TokenType::COMMA, 
//                         "Expected ',' after value expression in struct-expr".to_string(), true);
//                 } 
//             }

//             return Some(Box::new(Expr::StructExpr{struct_name:name, fields}));
//         }
//         return None;
//     }

//     #[allow(unused)]
//     fn call(&mut self, lhs: Option<Box<Expr>>)          ->  Option<Box<Expr>>   { return None; }


//     fn binary(&mut self, lhs: Option<Box<Expr>>)        ->  Option<Box<Expr>>   { 

//         let left: Box<Expr>;
//         if let Some(loprnd) = lhs { left = loprnd; }
//         else { return None; }

//         let operator: Token;
//         if let Some(oprtr) = self.curr() { operator = oprtr; }
//         else { return None; }

//         let rule = ParseRule::get_rule(operator.tok_type.clone());
        
//         let right: Box<Expr>;
//         if let Some(roprnd) = self.parse_precedence(rule.precedence) { right = roprnd; }
//         else { return None; }

//         return Some(Box::new(Expr::Binary{lhs: left, rhs: right, operator: operator, datatype: Datatype::yet_to_infer}));
//     }


//     fn parse(tokens: Vec<Token>) -> Vec<Decl> {


//         drop(tokens);
//         return vec![];
//     }
// }