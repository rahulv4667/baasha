// mod crate::ast;
use crate::ast::*;

/* Reference Sources: 
    - https://github.com/rust-unofficial/patterns/discussions/236
    - https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html
*/

// pub trait Data {
//     fn accept<V: Visitor>(&self, visitor: &mut V)   -> V::Result;
// }

pub trait Visitor<T> {
    // type Result;

    fn visit_stmt(&mut self, stmt: &Stmt)   -> T;
    fn visit_expr(&mut self, expr: &Expr)   -> T;
    fn visit_decl(&mut self, decl: &Decl)   -> T;
}

pub trait MutableVisitor<D, S, E> {
    fn visit_stmt(&mut self, stmt: &mut Stmt) -> S;
    fn visit_expr(&mut self, expr: &mut Expr) -> E;
    fn visit_decl(&mut self, decl: &mut Decl) -> D;
}

// impl Data for Stmt {
//     fn accept<V:Visitor>(&self, visitor: &mut V) -> V::Result {
//         return visitor.visit_stmt(self);
//     }
// }

// impl Data for Expr {
//     fn accept<V:Visitor>(&self, visitor: &mut V)    -> V::Result {
//         return visitor.visit_expr(self);
//     }
// }

/////////////////////////////////////////////////////////////////////////////
////////////////////////// Printing Visitor /////////////////////////////////
/////////////////////////////////////////////////////////////////////////////
pub struct Printer{
    pub space_width: usize
}

impl Printer {
    fn print_space(&self) {
        let mut s: String = String::new();
        let mut i = 0;
        while i < self.space_width  {
            s += " ";
            i += 1;
        }
        print!("{}", s);
    }

    fn print_data(&self, data: String) {
        self.print_space();
        println!("{}", data);
    }

}

// return type tells how many tabs it needs to go in
impl Visitor<()> for Printer {
    
    fn visit_expr(&mut self, expr: &Expr) {

        self.print_data("|".to_string());
        self.print_data("|".to_string());

        match expr {
            Expr::Variable{name, datatype, struct_name}
                => {
                    match struct_name {
                    Some(str_name)=>self.print_data(format!("Identifier{{ Name:{:?}, Datatype:{:?}, Struct_name:{}}}"
                    , name, datatype, str_name)),
                    None => self.print_data(format!("Identifier{{ Name:{:?}, Datatype:{:?} }}", name, datatype))
                    }
                },

            Expr::AttributeRef {object, name, datatype}
                => {
                    self.print_data(format!("Get{{ object, Name:{:?}, Datatype: {:?} }}", name, datatype));
                    self.space_width += 6;
                    self.visit_expr(object);
                    self.space_width -= 6;
                },

            Expr::Binary {lhs, rhs, operator, datatype}
                => {
                    self.print_data(format!("Binary{{ lhs, rhs, operator:{:?}, Datatype: {:?} }}", operator, datatype));
                    
                    self.space_width += 10;
                    
                    self.print_data("LHS".to_string());
                    self.visit_expr(lhs);
                    
                    self.print_data("RHS".to_string());
                    self.visit_expr(rhs);
                    
                    self.space_width -= 10;
                },

            Expr::Literal{value, datatype}
                => {
                    self.print_data(format!("Primary{{ value:{:?}, datatype: {:?} }}", value, datatype));
                },

            Expr::Unary{operator, operand, datatype}
                => {
                    self.print_data(format!("Unary{{ operand, operator: {:?}, datatype: {:?} }}",
                    operator, datatype));
                    
                    self.space_width += 10;
                    self.print_data("Operand".to_string());
                    self.visit_expr(operand);
                    self.space_width -= 10;
                },

            #[allow(dead_code, unused_variables)]
            Expr::StructExpr{struct_name, fields, datatype}
                => {
                    self.print_data(format!("StructExpr{{ struct_name: {:?} }}", struct_name));
                    self.space_width += 15;

                    for field in fields {
                        self.print_data(format!("{:?} : ", field.0));
                        self.space_width += 5;
                        self.visit_expr(&field.1);
                        self.space_width -= 5;
                    }

                    self.space_width -= 15;
                },

            #[allow(unused_variables)]
            // Expr::Assignment{target_list, expr_list, datatype}
            Expr::Assignment{target, expr, operator, datatype}
                => {
                    // let mut i: usize = 0;
                    self.print_data("Assignment{{ }}".to_string());
                    self.space_width += 13;
                    self.print_data("Target".to_string());
                    self.visit_expr(target);
                    // while i < target_list.len() {
                    //     self.print_data("Target".to_string());
                    //     self.visit_expr(&target_list[i]);
                    //     i += 1;
                    // }

                    self.print_data("Source".to_string());
                    self.visit_expr(expr);
                    // // keeping things separate coz, there could be function call on RHS which
                    // // makes number of values in the lists to not match.
                    // let mut i: usize = 0;
                    // while i < expr_list.len() {
                    //     self.print_data("Source".to_string());
                    //     self.visit_expr(&expr_list[i]);
                    //     i += 1;
                    // }
                    self.space_width -= 13;
                },
            
            Expr::Grouping { expr, datatype} 
                => {
                    self.print_data(format!("Grouping{{ }}, Datatype: {:?}", datatype));
                    self.space_width += 10;
                    self.visit_expr(&expr);
                    self.space_width -= 10;
                },

            Expr::Cast { variable, cast_type, datatype} 
                => {
                    self.print_data(format!("Cast{{ cast_to: {:?} }}, Datatype: {:?}", cast_type, datatype));
                    self.space_width += 5;
                    self.visit_expr(&variable);
                    self.space_width -= 5;
                },
            Expr::ExprList{expr_list, datatype}
                => {
                    self.print_data(format!("Expression List{{ }}, Datatype: {:?}", datatype));
                    self.space_width += 13;
                    let mut i: usize = 0;
                    while i < expr_list.len() {
                        self.print_data(format!("Expression {}", i+1));
                        self.visit_expr(&expr_list[i]);
                        i += 1;
                    } 
                    self.space_width -= 13;
                },
            #[allow(unused_variables)]
            Expr::Call{callee, arguments, datatype}
                => {
                    self.print_data("Function Call{{ }}".to_string());
                    self.space_width += 13;
                    
                    self.print_data("Callee {{ }}".to_string());
                    self.space_width += 10;

                    self.visit_expr(callee);
                    let mut i=0;
                    while i < arguments.len() {
                        self.print_data(format!("Argument {}", i+1));
                        self.visit_expr(&arguments[i]);
                        i += 1;
                    }

                    self.space_width -= 10;

                    self.space_width -= 13;
                },
            #[allow(unreachable_patterns)]
            _ => unimplemented!()
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt)  {
        self.print_data("|".to_string());
        self.print_data("|".to_string());
        match stmt {

            Stmt::Decl{decl} 
                => {
                    self.print_data("Declaration{{ }}".to_string());
                    self.space_width += 10;
                    self.visit_decl(&decl);
                    self.space_width -= 10;
                },

            Stmt::While{condition, block}
                => {
                    self.print_data("While{{ }}".to_string());
                    self.space_width += 5;
                    self.visit_expr(&condition);
                    self.visit_stmt(&block);
                    self.space_width -= 5;
                },

            Stmt::Block{statements}    
                => {
                    self.print_data("Block{{ }}".to_string());
                    self.space_width += 7;
                    for stmt in statements {
                        self.visit_stmt(&stmt);
                    }
                    self.space_width -= 7;
                },

            Stmt::Expression{expr}
                => {
                    self.print_data("Expression{{ }}".to_string());
                    self.space_width += 13;
                    self.visit_expr(expr);
                    self.space_width -= 13;
                },

            // Stmt::Return{expr_list}
            Stmt::Return{expr}
                => {
                    self.print_data("Return{{ }}".to_string());
                    self.space_width += 10;
                    self.visit_expr(&expr);
                    // for expr in expr_list {
                    //     self.visit_expr(&expr);
                    // }
                    self.space_width -= 10;
                },

            Stmt::If{condition, then_block, else_block, ..}
                => {
                    self.print_data("If{{ }}".to_string());
                    self.space_width += 5;
                    
                    self.print_data("Condition{{ }}".to_string());
                    self.space_width += 10;
                    self.visit_expr(condition);
                    self.space_width -= 10;

                    self.print_data("Then{{ }}".to_string());
                    self.space_width += 5;
                    self.visit_stmt(then_block);
                    self.space_width -= 5;

                    self.print_data("Else{{ }}".to_string());
                    self.space_width += 5;
                    match else_block {
                        Some(blk)   =>  self.visit_stmt(blk),
                        None                  =>  ()
                    };
                    self.space_width -= 5;

                    self.space_width -= 5;
                },

            Stmt::For{initialization, condition, updation, block, ..}
                => {
                    self.print_data("For{{ }}".to_string());
                    self.space_width += 5;

                    
                    if let Some(init_expr) = initialization {
                        self.print_data("Initialization{{ }}".to_string());
                        self.space_width += 15;
                        self.visit_expr(init_expr);
                        self.space_width -= 15;

                    }
                    
                    if let Some(cond) = condition {
                        self.print_data("Condition{{ }}".to_string());
                        self.space_width += 10;
                        self.visit_expr(cond);
                        self.space_width -= 10;
                    }
                    

                    if let Some(update) = updation {
                        self.print_data("Updation{{ }}".to_string());
                        self.space_width += 10;
                        self.visit_expr(update);
                        self.space_width -= 10;
                    }
                    

                    self.print_data("Block{{ }}".to_string());
                    self.space_width += 5;
                    self.visit_stmt(block);
                    self.space_width -= 5;

                    self.space_width -= 5;

                },

            Stmt::Var{name, datatype, initialization_value}
                => {
                    self.print_data(format!("Var Stmt{{ Name: {:?}, Datatype: {:?} }}",
                            name, datatype));
                    self.space_width += 10;
                    match initialization_value {
                        Some(val) => {
                            self.print_data("Initialization value: {{ }}".to_string());
                            self.visit_expr(&val);
                        },
                        _ => self.print_data("No initialization data{{ }}".to_string())
                    }
                    self.space_width -= 10;
                }

            // Stmt::Loop{condition} 
            //     => {
            //         self.print_data("Loop Condition{{ }}".to_string());
            //         self.space_width += 15;
            //         self.visit_expr(condition);
            //         self.space_width -= 15;
            //     },
            // _ => unimplemented!()
        }
    }


    fn visit_decl(&mut self, decl: &Decl)  {
        self.print_data("|".to_string());
        self.print_data("|".to_string());
        match decl {
            Decl::FuncDef{prototype, block}
                => {
                    self.print_data("FuncDef{{ }}".to_string());

                    self.space_width += 10;
                    self.visit_decl(prototype);
                    self.visit_stmt(block);
                    self.space_width += 10;
                },

            Decl::ImplDecl{name, trait_name, funcs}
                => {
                    if let Some(tr_name) = trait_name  {
                        self.print_data(format!("Impl{{ name: {:?}, trait_name: {:?} }}", name, tr_name));
                    } else {
                        self.print_data(format!("Impl{{ name: {:?} }}", name));
                    }
                    
                    self.space_width += 6;
                    for func in funcs {
                        self.visit_decl(&func);
                    }
                    self.space_width -= 6;
                },

            // Decl::Prototype{name, parameters, returntypes}
            Decl::Prototype{name, parameters, returntype}
                => {
                    self.print_data(format!("Prototype{{ Name:{:?} }}", name));

                    self.space_width += 10;
                    self.print_data("Parameters {{ }}".to_string());
                    for param in parameters {
                        self.print_data(format!("Field Name: {:?}, Datatype: {:?}", param.0, param.1));
                    }
                    self.space_width -= 10;

                    self.space_width += 10;
                    self.print_data(format!("ReturnType{{ {:?} }}", returntype));
                    // for typ in returntypes {
                    //     self.print_data(format!("Type:{:?}", typ));
                    // }
                    self.space_width -= 10;
                },

            Decl::StructDecl{name, fields}
                => {
                    self.print_data(format!("StructDecl{{ Name: {:?} }}", name));

                    self.space_width += 10;
                    self.print_data("Fields{{ }}".to_string());
                    for field in fields {
                        self.print_data(format!("Field name: {:?}, datatype: {:?}", field.0, field.1));
                    }
                    self.space_width -= 10;
                },

            Decl::TraitDecl{name, funcs}
                => {
                    self.print_data(format!("Trait{{ Name: {:?} }}", name));

                    self.space_width += 10;
                    self.print_data("Funcs{{ }}".to_string());
                    for func in funcs {
                        self.visit_decl(func);
                    }
                    self.space_width -= 10;
                },

            // Decl::Program{decls}
            //     => {
            //         for decl in decls {
            //             self.visit_decl(decl);
            //         }
            //     }
            // _ => unimplemented!()
        }
    }
}