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
    pub symbol_table: SymbolTable
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
                    let pre_environment = self.symbol_table.clone();
                    self.visit_stmt(block);
                    self.symbol_table = pre_environment;
                },
            Decl::Prototype{name, parameters, returntypes}
                => {
                    self.symbol_table.func_table.insert(name.value.clone(), 
                        Decl::Prototype{name: name.clone(), parameters: parameters.clone(), 
                                returntypes: returntypes.clone()});
                    for param in parameters {
                        self.symbol_table.variable_table.insert(param.0.value.clone(), param.1.clone());
                    }
                },
            // Decl::ImplDecl{name, trait_name, funcs}
            //     => {

            //     },
            _ => ()
        }
    }
    
    fn visit_stmt(&mut self, stmt: &mut Stmt) {
        match stmt {
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
            Expr::AttributeRef{object, name, datatype}
                => self.visit_attributeref_expr(object, name, datatype),
            Expr::Binary{lhs, rhs, operator, datatype}
                => self.visit_binary_expr(lhs, rhs, operator, datatype),
            Expr::Unary{operator, operand, datatype}
                => self.visit_unary_expr(operator, operand, datatype),
            Expr::StructExpr{struct_name, fields, datatype}
                => self.visit_struct_expr(struct_name, fields, datatype),
            Expr::Assignment{target, expr, operator, datatype}
                => self.visit_assignment_expr(target, expr, operator, datatype),
                // => self.visit_assignment_expr(target_list, expr_list, datatype),
            Expr::Grouping{expr}
                => self.visit_grouping_expr(expr),
            Expr::Cast{variable, cast_type} 
                => self.visit_cast_expr(variable, cast_type),
            _ => {return Datatype::yet_to_infer;}
        }
    }
}

#[allow(dead_code, unused)]
impl TypeChecker {
    fn visit_cast_expr(&mut self, variable: &mut Box<Expr>, cast_type: &mut Token) -> Datatype {
        unimplemented!()
    }

    fn visit_grouping_expr(&mut self, expr: &mut Box<Expr>) -> Datatype {
        return self.visit_expr(expr);
    }

    // fn visit_assignment_expr(&mut self, target_list: &mut Vec<Box<Expr>>, 
    //                         expr_list: &mut Vec<Box<Expr>>, datatype:&mut Datatype) -> Datatype {
    //     unimplemented!()
    // }
    fn visit_assignment_expr(&mut self, target: &mut Box<Expr>, 
                expr: &mut Box<Expr>, operator: &mut Token, datatype: &mut Datatype) -> Datatype {
                    
        let lhs_datatype: Datatype = self.visit_expr(target);
        let rhs_datatype: Datatype = self.visit_expr(expr);
            
        if lhs_datatype == Datatype::yet_to_infer || rhs_datatype == Datatype::yet_to_infer {
            return Datatype::yet_to_infer;
        }
                    
        let mut has_error: bool = false;
                    
        if lhs_datatype != rhs_datatype {
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
                                    log_message(logger::LogLevel::ERROR, field_val.col, field_val.line, 
                                        "Datatype of expression being assigned doesn't match type declaration in struct".to_string());
                                    has_error = true;
                                }

                            },
                            _ => {
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

                        log_message(logger::LogLevel::ERROR, struct_name.col, struct_name.line, 
                            format!("Some fields were missing from expression: {:?}", missing_fields));
                        has_error = true;
                    }
                },
                _ => ()
            }

            *datatype = if has_error {Datatype::yet_to_infer} else {Datatype::object};
            return (*datatype).clone();
        } else {
            logger::log_message(logger::LogLevel::ERROR, struct_name.col, struct_name.line, 
                "Couldn't find struct declaration of given name. Make sure to declare struct before using it.".to_string());
            return Datatype::yet_to_infer;
        }
    }

    fn visit_binary_expr(&mut self, lhs:&mut Box<Expr>, rhs: &mut Box<Expr>, operator: &mut Token, datatype: &mut Datatype) -> Datatype {
        let lhs_datatype: Datatype = self.visit_expr(lhs);
        let rhs_datatype: Datatype = self.visit_expr(rhs);

        if lhs_datatype == Datatype::yet_to_infer || rhs_datatype == Datatype::yet_to_infer {
            return Datatype::yet_to_infer;
        }
        
        let mut has_error: bool = false;
        
        if lhs_datatype != rhs_datatype {
            logger::log_message(logger::LogLevel::ERROR, 
                operator.col, operator.line, "Operand types mismatch".to_string());
                has_error = true;
        }
        
        match operator.tok_type {
            TokenType::PLUS|TokenType::MINUS|TokenType::ASTERISK
            |TokenType::SLASH|TokenType::MOD => {
                if lhs_datatype == Datatype::object || lhs_datatype == Datatype::bool {
                    log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                                "LHS of operator is either an object or bool. Operation can't be performed".to_string());
                    has_error = true;
                }
                
                if rhs_datatype == Datatype::object {
                    log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                                "RHS of operator is either an object or bool. Operation can't be performed".to_string());
                    has_error = true;
                }
            },

            TokenType::BITWISE_AND|TokenType::BITWISE_OR|TokenType::BITWISE_XOR => {
                if !Datatype::get_int_types().contains(&lhs_datatype) && lhs_datatype != Datatype::bool {
                    log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                        "LHS of operator is neither integer type nor boolean type. Operation can't be performed".to_string());
                    has_error = true;
                }

                if !Datatype::get_int_types().contains(&rhs_datatype) && rhs_datatype != Datatype::bool {
                    log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                        "RHS of operator is neither integer type nor boolean type. Operation can't be performed".to_string());
                    has_error = true;
                }
            },

            TokenType::LEFT_SHIFT | TokenType::RIGHT_SHIFT => {
                if !Datatype::get_int_types().contains(&lhs_datatype) || !Datatype::get_int_types().contains(&rhs_datatype) {
                    log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                        "One of the operands is not integer. Operation can't be performed".to_string());
                    has_error = true;
                }
            },

            TokenType::EQUAL_EQUAL|TokenType::BANG_EQUAL
            |TokenType::LESS_THAN|TokenType::LESS_EQUAL
            |TokenType::GREAT_THAN|TokenType::GREAT_EQUAL => {
                if lhs_datatype == Datatype::object {
                    // find a way to match object types. Or make this error check after converting to llvm ir.
                }
                return Datatype::bool;
            },
            
            TokenType::K_AND|TokenType::K_OR => {
                if lhs_datatype != Datatype::bool {
                    log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                                "LHS of logical operations needs to be of type 'bool'".to_string());
                    has_error = true;
                }
                
                if rhs_datatype != Datatype::bool {
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
                    log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                        "Operand for '!' needs to be of boolean type".to_string());
                    has_error = true; 
                }
            },
            
            TokenType::PLUS => {
                if !Datatype::get_int_types().contains(&dtype) || !Datatype::get_float_types().contains(&dtype) {
                    log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                        "Can't use unary '+' on types other than integers or floats".to_string());
                        has_error = true;
                }
            },
            
            TokenType::MINUS => {
                if !Datatype::get_signed_types().contains(&dtype) || !Datatype::get_float_types().contains(&dtype) {
                    log_message(logger::LogLevel::ERROR, operator.col, operator.line, 
                        "Can't use unary '-' on types other than signed integers and floats".to_string());
                    has_error = true;
                }
            },

            TokenType::BITWISE_NOT => {
                if !Datatype::get_int_types().contains(&dtype) || dtype != Datatype::bool {
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
        *datatype = Datatype::get_datatype(&value.tok_type);
        return (*datatype).clone();
    }

    fn visit_call_expr(&mut self, callee: &mut Box<Expr>, arguments: &mut Vec<Box<Expr>>/*&mut Box<Expr>*/,
                         datatype: &mut Datatype) -> Datatype {
        unimplemented!()
    }

    fn visit_attributeref_expr(&mut self, object: &mut Box<Expr>, name: &mut Token, datatype: &mut Datatype) -> Datatype {
        unimplemented!()
    }

    fn visit_variable_expr(&mut self, name: &mut Token, datatype: &mut Datatype, struct_name: &mut Option<String>) -> Datatype {
        match self.symbol_table.variable_table.get(&name.value) {
            Some(dtype) => *datatype = Datatype::get_datatype(&dtype.tok_type),
            _ => {
                logger::log_message(logger::LogLevel::ERROR, 
                    name.col, name.line, format!("Undefined variable '{}'", name.value));
                return Datatype::yet_to_infer;
            }
        }

        if *datatype == Datatype::object {
            match struct_name {
                Some(strct_name) => {
                    if self.symbol_table.struct_decls.contains_key(strct_name) {
                        return Datatype::object;
                    } else {
                        logger::log_message(logger::LogLevel::ERROR, 
                            name.col, name.line, 
                            format!("Unable to find struct declaration {} for variable {}", 
                                strct_name, name.value));
                        *datatype = Datatype::yet_to_infer;
                        return Datatype::yet_to_infer;
                    }
                },
                _ => {
                    logger::log_message(logger::LogLevel::ERROR, 
                        name.col, name.line, 
                        "variable declared as object but unable to find its struct type".to_string());
                    *datatype = Datatype::yet_to_infer;
                    return Datatype::yet_to_infer;
                }
            }
        }

        return Datatype::yet_to_infer;
    }
}