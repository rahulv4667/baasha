use std::collections::HashMap;

use crate::logger;
// mod visitor;
use crate::visitor::MutableVisitor;
use crate::ast::*;
use crate::symbol_table::*;
use crate::lexer::Token;
use crate::globals::*;
use crate::logger::*;


pub struct TypeChecker {
    pub symbol_table: SymbolTable,
    pub current_scope: Scope,
    pub has_errors: bool
}

impl MutableVisitor<(), (), Datatype> for TypeChecker {
    fn visit_decl(&mut self, decl: &mut Decl) {
        match decl {
            Decl::StructDecl{name, fields}
                => {
                    self.symbol_table.struct_decls.insert(
                        name.value.clone(), Decl::StructDecl{name: name.clone(), fields: fields.clone()});
                },
            Decl::FuncDef{prototype, block}
                => {
                    self.visit_decl(prototype);

                    /* storing environment so that local variables dont affect the original env */
                    // let pre_environment = self.symbol_table.clone();
                    self.visit_stmt(block);
                    // self.symbol_table = pre_environment;
                },
            Decl::Prototype{name, parameters, returntype}
                => {
                    self.symbol_table.func_table.insert(name.value.clone(), 
                        Decl::Prototype{name: name.clone(), parameters: parameters.clone(), 
                                returntype: returntype.clone()});
                    for param in parameters {
                        self.symbol_table.variable_table.insert(param.0.value.clone(), Datatype::get_tok_datatype(&param.1));
                    }
                    if let Scope::Impl{name, ..} = &self.current_scope {
                        self.symbol_table.variable_table.insert(
                            "self".to_string(), 
                            Datatype::object{name: name.clone()}
                        );
                    }
                },
            /////////////////////////////////////////////////////////////
            // Decl::ImplDecl{name, trait_name, funcs}
            Decl::ImplDecl{name, funcs, trait_name}
                => {
                    let scope = self.current_scope.clone();
                    self.current_scope = Scope::Impl{
                        name: name.value.clone(), 
                        trait_name: if trait_name.is_none() { 
                            "".to_string()
                        } else { 
                            trait_name.as_ref().unwrap().value.clone()
                        }
                    };
                    for func in funcs {
                        self.visit_decl(func);
                        (*self.symbol_table.impl_decls.entry(
                            name.value.clone()
                        ).or_insert(vec![])).push((*func).clone());
                    }
                    self.current_scope = scope;
                },
            Decl::TraitDecl{name, funcs}
                => {
                    let scope = self.current_scope.clone();
                    self.current_scope = Scope::Trait{name: name.value.clone()};
                    // (*self.symbol_table.trait_decls.entry(name.value.clone()).or_insert(vec![])).append(funcs);
                    for func in funcs {
                        self.visit_decl(func); 
                        (*self.symbol_table.trait_decls.entry(
                            name.value.clone()
                        ).or_insert(vec![])).push((*func).clone());
                    }
                    self.current_scope = scope;
                },
            // _ => ()
        }
    }
    
    fn visit_stmt(&mut self, stmt: &mut Stmt) {
        match stmt {
            Stmt::Block{statements} => {
                let pre_environment = self.symbol_table.clone();
                for statement in statements {
                    self.visit_stmt(statement);
                }
                self.symbol_table = pre_environment;
            },
            Stmt::Decl{decl} => self.visit_decl(decl),
            Stmt::Expression{expr} => {self.visit_expr(expr);},
            Stmt::For{
                for_token,
                initialization, 
                condition, 
                updation, 
                block}
                => {
                    if let Some(init_expr) = initialization {
                        self.visit_expr(init_expr);
                    }

                    if let Some(cond) = condition {
                        if self.visit_expr(cond) != Datatype::bool {
                            self.has_errors = true;
                            log_message(logger::LogLevel::ERROR, 
                                for_token.col, for_token.line, 
                                "Condition expression of `for` should be of type `bool`".to_string());
                        }
                    }

                    if let Some(update) = updation {
                        self.visit_expr(update);
                    }

                    self.visit_stmt(block);
                },
            Stmt::If{if_token, condition, then_block, else_block} 
                => {
                    if self.visit_expr(condition) != Datatype::bool {
                        self.has_errors = true;
                        log_message(logger::LogLevel::ERROR, 
                            if_token.col, if_token.line, 
                            "Condition expression of `if` should be of type `bool`".to_string());
                    }   
                    self.visit_stmt(then_block);
                    if let Some(else_blk) = else_block {
                        self.visit_stmt(else_blk);
                    }
                },
            Stmt::Return{expr} => {self.visit_expr(expr);},
            Stmt::Var{name, datatype, initialization_value}
                => {
                    let mut dtype: Datatype = Datatype::yet_to_infer;
                    if let Some(init_value) = initialization_value {
                        dtype = self.visit_expr(init_value);
                    }

                    if let Some(dttype) = datatype {
                        dtype = Datatype::get_tok_datatype(dttype);
                    }

                    // eprintln!("Variable declared: ")
                    self.symbol_table.variable_table.insert(name.value.clone(), dtype);
                    // self.symbol_table.variable_table[&name.value] = dtype;
                },
            // Stmt::While{condition, block}
            //     => (),
            _ => ()
        }
    }

    #[allow(dead_code,unused_variables)]
    fn visit_expr(&mut self, expr: &mut Expr) -> Datatype {
        
        match expr {
            Expr::Variable{name, datatype, struct_name}
                => self.visit_variable_expr(name, datatype, struct_name),
            Expr::Literal{value, datatype}
                => self.visit_literal_expr(value, datatype),
            Expr::Call{callee, arguments, datatype}
                => self.visit_call_expr(callee, arguments, datatype),
            Expr::AttributeRef{object, name, object_dtype, datatype}
                => self.visit_attributeref_expr(object, name, object_dtype, datatype),
            Expr::Binary{lhs, rhs, operator, datatype}
                => self.visit_binary_expr(lhs, rhs, operator, datatype),
            Expr::Unary{operator, operand, datatype}
                => self.visit_unary_expr(operator, operand, datatype),
            Expr::StructExpr{struct_name, fields, datatype}
                => self.visit_struct_expr(struct_name, fields, datatype),
            Expr::Assignment{target, expr, operator, datatype}
                => self.visit_assignment_expr(target, expr, operator, datatype),
                // => self.visit_assignment_expr(target_list, expr_list, datatype),
            Expr::Grouping{expr, datatype}
                => self.visit_grouping_expr(expr, datatype),
            Expr::Cast{variable, cast_type, from_dtype, to_dtype} 
                => self.visit_cast_expr(variable, cast_type, from_dtype, to_dtype),
            Expr::ExprList{expr_list, datatype}
                => self.visit_exprlist_expr(expr_list, datatype),
            // _ => {return Datatype::yet_to_infer;}
        }
    }
}

#[allow(dead_code, unused)]
impl TypeChecker {

    fn get_runtime_function_type(&self, func_name: String) -> Datatype {
        match func_name.as_str() {
            "scani8" => Datatype::function{
                name: func_name, 
                obj_name: None, 
                returntype: Box::new(Datatype::int8),
                param_types: vec![]
            },
            "scani16" => Datatype::function{
                name: func_name, 
                obj_name: None, 
                returntype: Box::new(Datatype::int16),
                param_types: vec![]
            },
            "scani32" => Datatype::function{
                name: func_name, 
                obj_name: None, 
                returntype: Box::new(Datatype::int32),
                param_types: vec![]
            },
            "scani64" => Datatype::function{
                name: func_name, 
                obj_name: None, 
                returntype: Box::new(Datatype::int64),
                param_types: vec![]
            },
            "scanu8" => Datatype::function{
                name: func_name, 
                obj_name: None, 
                returntype: Box::new(Datatype::uint8),
                param_types: vec![]
            },
            "scanu16" => Datatype::function{
                name: func_name, 
                obj_name: None, 
                returntype: Box::new(Datatype::uint16),
                param_types: vec![]
            },
            "scanu32" => Datatype::function{
                name: func_name, 
                obj_name: None, 
                returntype: Box::new(Datatype::uint32),
                param_types: vec![]
            },
            "scanu64" => Datatype::function{
                name: func_name, 
                obj_name: None, 
                returntype: Box::new(Datatype::uint64),
                param_types: vec![]
            },
            "scanf32" => Datatype::function{
                name: func_name, 
                obj_name: None, 
                returntype: Box::new(Datatype::float32),
                param_types: vec![]
            },
            "scanf64" => Datatype::function{
                name: func_name, 
                obj_name: None, 
                returntype: Box::new(Datatype::float64),
                param_types: vec![]
            },
            "printi8" => Datatype::function {
                name: func_name,
                obj_name: None,
                returntype: Box::new(Datatype::yet_to_infer),
                param_types: vec![Box::new(Datatype::int8)]
            },
            "printi16" => Datatype::function {
                name: func_name,
                obj_name: None,
                returntype: Box::new(Datatype::yet_to_infer),
                param_types: vec![Box::new(Datatype::int16)]
            },
            "printi32" => Datatype::function {
                name: func_name,
                obj_name: None,
                returntype: Box::new(Datatype::yet_to_infer),
                param_types: vec![Box::new(Datatype::int32)]
            },
            "printi64" => Datatype::function {
                name: func_name,
                obj_name: None,
                returntype: Box::new(Datatype::yet_to_infer),
                param_types: vec![Box::new(Datatype::int64)]
            },
            "printu8" => Datatype::function {
                name: func_name,
                obj_name: None,
                returntype: Box::new(Datatype::yet_to_infer),
                param_types: vec![Box::new(Datatype::uint8)]
            },
            "printu16" => Datatype::function {
                name: func_name,
                obj_name: None,
                returntype: Box::new(Datatype::yet_to_infer),
                param_types: vec![Box::new(Datatype::uint16)]
            },
            "printu32" => Datatype::function {
                name: func_name,
                obj_name: None,
                returntype: Box::new(Datatype::yet_to_infer),
                param_types: vec![Box::new(Datatype::uint32)]
            },
            "printu64" => Datatype::function {
                name: func_name,
                obj_name: None,
                returntype: Box::new(Datatype::yet_to_infer),
                param_types: vec![Box::new(Datatype::uint64)]
            },
            "printf32" => Datatype::function {
                name: func_name,
                obj_name: None,
                returntype: Box::new(Datatype::yet_to_infer),
                param_types: vec![Box::new(Datatype::float32)]
            },
            "printf64" => Datatype::function {
                name: func_name,
                obj_name: None,
                returntype: Box::new(Datatype::yet_to_infer),
                param_types: vec![Box::new(Datatype::float64)]
            },
            "scanbool" => Datatype::function {
                name: func_name,
                obj_name: None,
                returntype: Box::new(Datatype::bool),
                param_types: vec![]
            },
            "printbool" => Datatype::function {
                name: func_name,
                obj_name: None,
                returntype: Box::new(Datatype::yet_to_infer),
                param_types: vec![Box::new(Datatype::bool)]
            },
            _ => Datatype::yet_to_infer
        }
    }

    fn visit_cast_expr(&mut self, 
        variable: &mut Box<Expr>, 
        casttype: &mut Token, 
        from_dtype: &mut Datatype,
        to_dtype: &mut Datatype) -> Datatype {
        let var_type = self.visit_expr(variable);
        *from_dtype = var_type.clone();
        let mut cast_type = Datatype::get_datatype(&casttype.tok_type);
        if let Datatype::object{..} = cast_type {
            // check if a struct of given casttype exists
            if self.symbol_table.struct_decls.contains_key(&casttype.value) {
                cast_type = Datatype::object{name: casttype.value.clone()};
            } else {
                // casttype doesnt exist.
                self.has_errors = true;
                log_message(logger::LogLevel::ERROR, casttype.col, casttype.line, 
                    "Given cast type doesn't exist. Make sure to declare structs before using.".to_string());
                *to_dtype = var_type.clone();
                return var_type;
            }
        }

        // if both are objects, check if they are castable while converting to LLVM IR by matching 


        if var_type == Datatype::yet_to_infer { 
            *to_dtype = var_type;
            return Datatype::yet_to_infer; 
        }
        *to_dtype = cast_type.clone();
        return cast_type.clone();
    }

    fn visit_grouping_expr(&mut self, expr: &mut Box<Expr>, datatype: &mut Datatype) -> Datatype {
        *datatype = self.visit_expr(expr);
        return (*datatype).clone();
    }


    fn visit_assignment_expr(&mut self, target: &mut Box<Expr>, 
                expr: &mut Box<Expr>, operator: &mut Token, datatype: &mut Datatype) -> Datatype {
                    
        let lhs_datatype: Datatype = self.visit_expr(target);
        let rhs_datatype: Datatype = self.visit_expr(expr);
            
        if lhs_datatype == Datatype::yet_to_infer || rhs_datatype == Datatype::yet_to_infer {
            return Datatype::yet_to_infer;
        }
                    
        let mut has_error: bool = false;
                    
        if lhs_datatype != rhs_datatype {
            self.has_errors = true;
            logger::log_message(logger::LogLevel::ERROR, 
            operator.col, operator.line, "Operand types mismatch".to_string());
            has_error = true;
        }

        *datatype = if has_error {Datatype::yet_to_infer} else {lhs_datatype.clone()};
        return (*datatype).clone();
    }

    fn visit_struct_expr(&mut self, struct_name: &mut Token, fields: &mut Vec<(Token, Box<Expr>)>, datatype: &mut Datatype) -> Datatype {
        let mut field_vals = fields;
        let mut has_error = false;
        
        
        if let Some(decl) = self.symbol_table.struct_decls.get(&struct_name.value)/* .as_mut()*/ {
            match decl {
                Decl::StructDecl{name, fields} => {

                    // creating a hashmap for fields declaration
                    let mut field_decl_map: HashMap<String, Token> = HashMap::new();
                    for field_decl in fields {
                        field_decl_map.insert(field_decl.0.value.clone(), field_decl.1.clone());
                    }

                    // creating a hashmap to check if all types are given in structexpr
                    let mut field_presence_map: HashMap<String, bool> = HashMap::new();

                    // iterating through fields in structexpr to find if they exist in declaration and if type matches.
                    for (field_val , field_expr)in field_vals {
                        match field_decl_map.get(&field_val.value) {
                            Some(field_type) => {
                                // update presence map 
                                field_presence_map.insert(field_val.value.clone(), true);

                                let dtype = self.visit_expr(field_expr);

                                if dtype != Datatype::get_datatype(&field_type.tok_type) {
                                    self.has_errors = true;
                                    log_message(logger::LogLevel::ERROR, field_val.col, field_val.line, 
                                        "Datatype of expression being assigned doesn't match type declaration in struct".to_string());
                                    has_error = true;
                                }

                            },
                            _ => {
                                self.has_errors = true;
                                log_message(logger::LogLevel::ERROR, field_val.col, field_val.line, 
                                    "Couldn't find field name in struct declaration".to_string());
                                has_error = true;
                            }
                        }
                    }

                    // check if all declared fields were present in structexpr.
                    if field_presence_map.len() != field_decl_map.len() {
                        let mut missing_fields: Vec<String> = vec![];

                        for field_decl in field_decl_map {
                            match field_presence_map.get(&field_decl.0) {
                                Some(_) => (),
                                None => {missing_fields.push(field_decl.0.clone());}
                            }
                        }

                        self.has_errors = true;
                        log_message(logger::LogLevel::ERROR, struct_name.col, struct_name.line, 
                            format!("Some fields were missing from expression: {:?}", missing_fields));
                        has_error = true;
                    }
                },
                _ => ()
            }

            *datatype = if has_error {Datatype::yet_to_infer} else {Datatype::object{name: struct_name.value.clone()}};
            return (*datatype).clone();
        } else {
            self.has_errors = true;
            logger::log_message(logger::LogLevel::ERROR, struct_name.col, struct_name.line, 
                "Couldn't find struct declaration of given name. Make sure to declare struct before using it.".to_string());
            return Datatype::yet_to_infer;
        }
    }

    fn visit_binary_expr(&mut self, lhs:&mut Box<Expr>, rhs: &mut Box<Expr>, operator: &mut Token, datatype: &mut Datatype) -> Datatype {
        let lhs_datatype: Datatype = self.visit_expr(lhs);
        let rhs_datatype: Datatype = self.visit_expr(rhs);
        

        eprintln!("Binary expr :::: Lhs type: {:?}, Rhs type: {:?}", lhs_datatype, rhs_datatype);
        if lhs_datatype == Datatype::yet_to_infer || rhs_datatype == Datatype::yet_to_infer {
            return Datatype::yet_to_infer;
        }
        
        let mut has_error: bool = false;
        
        if lhs_datatype != rhs_datatype {
            self.has_errors = true;
            logger::log_message(logger::LogLevel::ERROR, 
                operator.col, operator.line, "Operand types mismatch".to_string());
                has_error = true;
        }
        
        match operator.tok_type {
            TokenType::PLUS|TokenType::MINUS|TokenType::ASTERISK
            |TokenType::SLASH|TokenType::MOD => {
                // if lhs_datatype == Datatype::object{..} || lhs_datatype == Datatype::bool {
                //     log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                //                 "LHS of operator is either an object or bool. Operation can't be performed".to_string());
                //     has_error = true;
                // }
                match lhs_datatype {
                    Datatype::bool|Datatype::object{..} => {
                        self.has_errors = true;
                        log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                            "LHS of operator is either an object or bool. Operation can't be performed".to_string());
                        has_error = true;
                    },
                    _ => ()
                }

                // if rhs_datatype == Datatype::object {
                //     log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                //                 "RHS of operator is either an object or bool. Operation can't be performed".to_string());
                //     has_error = true;
                // }
                match rhs_datatype {
                    Datatype::bool|Datatype::object{..} => {
                        self.has_errors = true;
                        log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                            "RHS of operator is either an object or bool. Operation can't be performed.".to_string());
                        has_error = true;
                    },
                    _ => ()
                }
            },

            TokenType::BITWISE_AND|TokenType::BITWISE_OR|TokenType::BITWISE_XOR => {
                if !Datatype::get_int_types().contains(&lhs_datatype) && lhs_datatype != Datatype::bool {
                    self.has_errors = true;
                    log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                        "LHS of operator is neither integer type nor boolean type. Operation can't be performed".to_string());
                    has_error = true;
                }

                if !Datatype::get_int_types().contains(&rhs_datatype) && rhs_datatype != Datatype::bool {
                    self.has_errors = true;
                    log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                        "RHS of operator is neither integer type nor boolean type. Operation can't be performed".to_string());
                    has_error = true;
                }
            },

            TokenType::LEFT_SHIFT | TokenType::RIGHT_SHIFT => {
                if !Datatype::get_int_types().contains(&lhs_datatype) || !Datatype::get_int_types().contains(&rhs_datatype) {
                    self.has_errors = true;
                    log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                        "One of the operands is not integer. Operation can't be performed".to_string());
                    has_error = true;
                }
            },

            TokenType::EQUAL_EQUAL|TokenType::BANG_EQUAL
            |TokenType::LESS_THAN |TokenType::LESS_EQUAL
            |TokenType::GREAT_THAN|TokenType::GREAT_EQUAL => {
                // if lhs_datatype == Datatype::object {
                //     // find a way to match object types. Or make this error check after converting to llvm ir.
                // }
                return Datatype::bool;
            },
            
            TokenType::K_AND|TokenType::K_OR => {
                if lhs_datatype != Datatype::bool {
                    self.has_errors = true;
                    log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                                "LHS of logical operations needs to be of type 'bool'".to_string());
                    has_error = true;
                }
                
                if rhs_datatype != Datatype::bool {
                    self.has_errors = true;
                    log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                                "RHS of logical operations needs to be of type 'bool'".to_string());
                    has_error = true;
                }
            },
            
            _ => ()
        }
        *datatype = if has_error {Datatype::yet_to_infer} else {lhs_datatype.clone()};
        return (*datatype).clone();

    }

    fn visit_unary_expr(&mut self, operator: &mut Token, operand: &mut Box<Expr>, datatype: &mut Datatype) -> Datatype {
        let dtype = self.visit_expr(operand);
        let mut has_error: bool = false;
        match operator.tok_type {
            TokenType::BANG => {
                if dtype != Datatype::bool {
                    self.has_errors = true;
                    log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                        "Operand for '!' needs to be of boolean type".to_string());
                    has_error = true; 
                }
            },
            
            TokenType::PLUS => {
                if !Datatype::get_int_types().contains(&dtype) || !Datatype::get_float_types().contains(&dtype) {
                    self.has_errors = true;
                    log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                        "Can't use unary '+' on types other than integers or floats".to_string());
                        has_error = true;
                }
            },
            
            TokenType::MINUS => {
                if !Datatype::get_signed_types().contains(&dtype) || !Datatype::get_float_types().contains(&dtype) {
                    self.has_errors = true;
                    log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                        "Can't use unary '-' on types other than signed integers and floats".to_string());
                    has_error = true;
                }
            },

            TokenType::BITWISE_NOT => {
                if !Datatype::get_int_types().contains(&dtype) || dtype != Datatype::bool {
                    self.has_errors = true;
                    log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                        "Bitwise NOT can't be performed on types other than integers and boolean".to_string());
                    has_error;
                }
            }

            _ => ()
        }

        *datatype = if has_error {Datatype::yet_to_infer} else {dtype};
        return (*datatype).clone();
    }

    fn visit_literal_expr(&mut self, value: &mut Token, datatype: &mut Datatype) -> Datatype {
        // unimplemented!()
        *datatype = Datatype::get_datatype(&value.tok_type);
        eprintln!("Literal expr type: {:?}", *datatype);
        return (*datatype).clone();
    }

    fn visit_call_expr(&mut self, callee: &mut Box<Expr>, arguments: &mut Vec<Box<Expr>>/*&mut Box<Expr>*/,
                         datatype: &mut Datatype) -> Datatype {

        let func_type = self.visit_expr(callee);

        // let func_name;
        // let obj_name;
        // let 
        if let Datatype::function{name: func_name, obj_name, returntype, param_types}
            = func_type {

            if arguments.len() != param_types.len() {
                self.has_errors = true;
                logger::log_message(logger::LogLevel::ERROR, usize::MAX, usize::MAX, 
                    format!("Arity not right when calling {:?}: ({:?}) Parameters in definition but {:?} arguments provided",
                        if obj_name.is_some() { obj_name.clone().unwrap()+func_name.as_str()} else {func_name.clone()}, 
                        param_types.len(), arguments.len())
                );
                return Datatype::yet_to_infer;
            }
            for (i, argument) in arguments.into_iter().enumerate() {
                let arg_type = self.visit_expr(argument);
                if arg_type != *param_types[i] {
                    self.has_errors = true;
                    logger::log_message(logger::LogLevel::ERROR, usize::MAX, usize::MAX, 
                        format!("Argument type not matching at param number {:?} for function {:?}",
                            i+1, if obj_name.is_some() { obj_name.clone().unwrap()+func_name.as_str()} else {func_name.clone()})
                    );
                    return Datatype::yet_to_infer;
                }
            }

            *datatype = *returntype;
            return (*datatype).clone();
        } else {

        }
        return Datatype::yet_to_infer;

    }

    fn visit_attributeref_expr(&mut self, 
        object: &mut Box<Expr>, 
        name: &mut Token, 
        object_dtype: &mut Datatype,
        datatype: &mut Datatype) 
    -> Datatype {
        let mut attr_name = name;
        *object_dtype = self.visit_expr(object);

        if let Datatype::object{name: obj_name} = object_dtype {
            if let Decl::StructDecl{name: struct_name, fields} 
                = self.symbol_table.struct_decls.get(obj_name).unwrap() {
                    
                    for (field_name, field_type) in fields {
                        if field_name.value == attr_name.value {
                            *datatype = Datatype::get_tok_datatype(field_type);
                            return (*datatype).clone();
                        }
                    }

            }

            
            let impl_decls = self.symbol_table.impl_decls.get(obj_name).unwrap();
            for impl_decl in impl_decls {
                if let Decl::ImplDecl{name: struct_name, trait_name, funcs}
                    = &**impl_decl {

                        for func in funcs {

                            if let Decl::FuncDef{prototype, block} = &**func {

                                if let Decl::Prototype{name: func_name, parameters, returntype}
                                    = &**prototype {

                                    if func_name.value == attr_name.value {
                                        // create Datattype::Function{}
                                        let mut param_types = vec![];
                                        let ret_type = Box::new(Datatype::get_tok_datatype(returntype));

                                        for (param_name, param_type) in parameters {
                                            param_types.push(Box::new(Datatype::get_tok_datatype(param_type)));
                                        }

                                        return Datatype::function{
                                            name: func_name.value.clone(),
                                            obj_name: Some(struct_name.value.clone()),
                                            returntype: ret_type,
                                            param_types
                                        };
                                    }

                                }

                            }

                        }

                }
            }
        }

        return Datatype::yet_to_infer;

    }

    fn visit_variable_expr(&mut self, name: &mut Token, datatype: &mut Datatype, struct_name: &mut Option<String>) -> Datatype {
        match self.symbol_table.variable_table.get(&name.value) {
            Some(dtype) => {
                *datatype = (*dtype).clone();
                eprintln!("Variable datatype: {:?}", *datatype);
                return (*datatype).clone();
            },
            _ => {
                // self.has_errors = true;
                // logger::log_message(logger::LogLevel::ERROR, 
                //     name.col, name.line, format!("Undefined variable '{}'", name.value));
                // return Datatype::yet_to_infer;
            }
        }

        match *datatype {
            Datatype::object{..} => {
                match struct_name {
                    Some(strct_name) => {
                        if self.symbol_table.struct_decls.contains_key(strct_name) {
                            return Datatype::object{name: strct_name.clone()};
                        }
                    },
                    _ => {
                        self.has_errors = true;
                        log_message(logger::LogLevel::ERROR, name.col, name.line, 
                            "Variable declared as object but unable to find its struct type".to_string());
                        *datatype = Datatype::yet_to_infer;
                        return Datatype::yet_to_infer;
                    }
                }
            },
            _ => ()
        }

        if self.symbol_table.func_table.get(&name.value).is_some() {
            

                if let Decl::Prototype{name: func_name, parameters, returntype}
                    = self.symbol_table.func_table.get(&name.value).unwrap() {

                        if func_name.value == name.value {
                            // create Datattype::Function{}
                            let mut param_types = vec![];
                            let ret_type = Box::new(Datatype::get_tok_datatype(returntype));

                            for (param_name, param_type) in parameters {
                                param_types.push(Box::new(Datatype::get_tok_datatype(param_type)));
                            }

                            return Datatype::function{
                                name: func_name.value.clone(),
                                obj_name: None,
                                returntype: ret_type,
                                param_types
                            };
                        }

                    

                }

            
        }

        *datatype = self.get_runtime_function_type(name.value.clone());
        return (*datatype).clone();
        // return Datatype::yet_to_infer;
    }

    fn visit_exprlist_expr(&mut self, expr_list: &mut Vec<Box<Expr>>, datatype: &mut Datatype) -> Datatype {
        for expr in expr_list {
            *datatype = self.visit_expr(expr);
            eprintln!("Exprlist type: {:?}", *datatype);
        }
        return (*datatype).clone();
    }
}