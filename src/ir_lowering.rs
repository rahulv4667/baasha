// use inkwell::types::BasicTypeEnum;

// use std::any::Any;

// use std::collections::HashMap;

use std::collections::HashMap;

use inkwell::data_layout::DataLayout;
// use inkwell::basic_block::BasicBlock;
// use inkwell::builder;
use inkwell::types::{BasicTypeEnum, StringRadix};
// use generational_arena::Arena;
// use inkwell::types::AnyTypeEnum;
// use inkwell::types::IntType;
// use inkwell::types::AnyTypeEnum;
use inkwell::values::{AnyValue, AnyValueEnum, BasicValue, BasicValueEnum, FunctionValue, InstructionValue, IntValue};

use crate::globals::{self, Scope, TokenType};
use crate::lexer::Token;
// use crate::logger::log_message;
use crate::symbol_table::IRSymbolTable;
use crate::visitor::{VisitorWithLifeTime};
// use crate::visitor::MutableVisitor;
use crate::ast::*;
// use inkwell::builder::Builder;
// use inkwell::context::Context;

#[allow(unused)]
pub struct Codegen<'a, 'ctx> {
    pub context: &'ctx inkwell::context::Context,
    pub module: &'a inkwell::module::Module<'ctx>,
    pub builder: &'a inkwell::builder::Builder<'ctx>,
    // execution_engine: ExecutionEngine<'ctx>
    pub symbol_table: IRSymbolTable<'ctx>,
    pub current_scope: globals::Scope,
    pub curr_fn_value : Option<FunctionValue<'ctx>>,
    pub is_parsing_lvalue: bool
}


#[allow(unused)]
impl<'a, 'ctx> 
    VisitorWithLifeTime<
        Option<inkwell::values::AnyValueEnum<'ctx>>/*D*/, 
        Option<inkwell::values::AnyValueEnum<'ctx>>/*S*/, 
        // inkwell::values::AnyValueEnum<'ctx>/*E*/
        inkwell::values::BasicValueEnum<'ctx>
    > 
    for Codegen<'a, 'ctx> {
    fn visit_expr(&mut self, expr: &Expr) -> inkwell::values::BasicValueEnum<'ctx> {
        match expr {
            Expr::Assignment{target, operator, expr, datatype} 
                => self.visit_assignment_expr(target, operator, expr, datatype),
            Expr::AttributeRef{object, name, object_dtype, datatype} 
                => self.visit_attributeref_expr(object, name, object_dtype, datatype),
            Expr::Binary{lhs, rhs, operator, datatype} 
                => self.visit_binary_expr(lhs, rhs, operator, datatype),
            Expr::Call{callee, arguments, datatype} 
                => self.visit_call_expr(callee, arguments, datatype),
            Expr::Cast{variable, cast_type, from_dtype, to_dtype} 
                => self.visit_cast_expr(variable, cast_type, from_dtype, to_dtype),
            Expr::ExprList{expr_list, datatype} 
                => self.visit_exprlist_expr(expr_list, datatype),
            Expr::Grouping{expr, datatype} 
                => self.visit_grouping_expr(expr, datatype),
            Expr::Literal{value, datatype} 
                => self.visit_literal_expr(value, datatype),
            Expr::StructExpr{struct_name, fields, datatype} 
                => self.visit_struct_expr(struct_name, fields, datatype),
            Expr::Unary{operator, operand, datatype} 
                => self.visit_unary_expr(operator, operand, datatype),
            Expr::Variable{name, datatype, struct_name} 
                => self.visit_variable_expr(name, datatype, struct_name),
            // _ => inkwell::values::AnyValueEnum::ArrayValue(_),
        }
        // unimplemented!();
    }

    fn visit_decl(&mut self, decl: &Decl) -> Option<inkwell::values::AnyValueEnum<'ctx>> {
        match decl {
            Decl::FuncDef{prototype, block} 
                => self.visit_funcdef_decl(prototype, block),
            Decl::ImplDecl{name, trait_name, funcs} 
                => self.visit_impl_decl(name, trait_name, funcs),
            Decl::Prototype{name, parameters, returntype} 
                => self.visit_prototype_decl(name, parameters, returntype),
            Decl::StructDecl{name, fields} 
                => self.visit_struct_decl(name, fields),
            Decl::TraitDecl{name, funcs} 
                => self.visit_trait_decl(name, funcs),
            _ => None
        }
        // unimplemented!();
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> Option<inkwell::values::AnyValueEnum<'ctx>> {
        match stmt {
            Stmt::Block{statements}     
                => self.visit_block_stmt(statements),
            Stmt::Decl{decl} 
                => self.visit_decl_stmt(decl),
            Stmt::Expression{expr} 
                => self.visit_expression_stmt(expr),
            Stmt::For{
                for_token, 
                initialization, 
                condition, 
                updation,
                block
            } 
                => self.visit_for_stmt(for_token, initialization, condition, updation, block),
            Stmt::If{
                if_token,
                condition,
                then_block,
                else_block
            } => self.visit_if_stmt(if_token, condition, then_block, else_block),
            Stmt::Return{expr} 
                => self.visit_return_stmt(expr),
            Stmt::Var{name, datatype, initialization_value} 
                => self.visit_var_stmt(name, datatype, initialization_value),
            Stmt::While{..} => None,
            _ => None
        }
        // unimplemented!();
    }

}


#[allow(unused)]
impl<'a, 'ctx> Codegen<'a, 'ctx> {


    fn get_type(&mut self, token: &Token) -> inkwell::types::BasicTypeEnum<'ctx> {
        match token {
            Token{tok_type: TokenType::K_INT8, ..} 
            | Token{tok_type: TokenType::K_UINT8, ..} => self.context.i8_type().into(),
            Token{tok_type: TokenType::K_INT16, ..} 
            | Token{tok_type: TokenType::K_UINT16, ..} => self.context.i16_type().into(),
            Token{tok_type: TokenType::K_INT32, ..} 
            | Token{tok_type: TokenType::K_UINT32, ..} => self.context.i32_type().into(),
            Token{tok_type: TokenType::K_INT64, ..} 
            | Token{tok_type: TokenType::K_UINT64, ..} => self.context.i64_type().into(),

            Token{tok_type: TokenType::K_FLOAT32, ..} => self.context.f32_type().into(),
            Token{tok_type: TokenType::K_FLOAT64, ..} => self.context.f64_type().into(),

            Token{tok_type: TokenType::K_BOOL, ..} => self.context.bool_type().into(),
            Token{tok_type: TokenType::IDENTIFIER, value, ..} => {
                if let Some(struct_type) = self.module.get_struct_type(value) {
                    return struct_type.into();
                } else {
                    return self.context.opaque_struct_type(value).into();
                }
            }
            
            _ => self.context.bool_type().into()
        }
    }

    fn get_llvm_type(&mut self, datatype: &Datatype) -> inkwell::types::BasicTypeEnum<'ctx> {
        match datatype {
            Datatype::int8      |   Datatype::uint8     => self.context.i8_type().into(),
            Datatype::int16     |   Datatype::uint16    => self.context.i16_type().into(),
            Datatype::int32     |   Datatype::uint32    => self.context.i32_type().into(),
            Datatype::int64     |   Datatype::uint64    => self.context.i64_type().into(),
            Datatype::float32                           => self.context.f32_type().into(),
            Datatype::float64                           => self.context.f64_type().into(),
            Datatype::bool                              => self.context.bool_type().into(),
            _   => self.context.bool_type().into()
        }
    }

    fn add_(&mut self, lhs: BasicValueEnum<'ctx>, rhs: BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx> {
        if lhs.is_int_value() {
            return BasicValueEnum::IntValue(
                self.builder.build_int_add(
                    lhs.into_int_value(), 
                    rhs.into_int_value(), 
                    "integer.add_")
            );
        } else if lhs.is_float_value() {
            return BasicValueEnum::FloatValue(
                self.builder.build_float_add(
                    lhs.into_float_value(), 
                    rhs.into_float_value(), 
                    "floating.add_")
            );
        } else if lhs.is_vector_value() {
            // return AnyValueEnum::VectorValue(
            //     self.builder.build_add
            // );
        }
        unimplemented!();
    }

    fn sub_(&mut self, lhs: BasicValueEnum<'ctx>, rhs: BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx> {
        if lhs.is_int_value() {
            return BasicValueEnum::IntValue(
                self.builder.build_int_sub(
                    lhs.into_int_value(), 
                    rhs.into_int_value(), 
                    "integer.sub_")
            );
        } else if lhs.is_float_value() {
            return BasicValueEnum::FloatValue(
                self.builder.build_float_sub(
                    lhs.into_float_value(), 
                    rhs.into_float_value(), 
                    "floating.sub_")
            );
        } else if lhs.is_vector_value() {
            // return AnyValueEnum::VectorValue(
            //     self.builder.build_add
            // );
        }
        unimplemented!();
    }

    fn mul_(&mut self, lhs: BasicValueEnum<'ctx>, rhs: BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx> {
        if lhs.is_int_value() {
            return BasicValueEnum::IntValue(
                self.builder.build_int_mul(
                    lhs.into_int_value(), 
                    rhs.into_int_value(), 
                    "integer.mul_")
            );
        } else if lhs.is_float_value() {
            return BasicValueEnum::FloatValue(
                self.builder.build_float_mul(
                    lhs.into_float_value(), 
                    rhs.into_float_value(), 
                    "floating.mul_")
            );
        } else if lhs.is_vector_value() {
            // return AnyValueEnum::VectorValue(
            //     self.builder.build_add
            // );
        }
        unimplemented!();
    }

    fn div_(&mut self, lhs: BasicValueEnum<'ctx>, rhs: BasicValueEnum<'ctx>, datatype: &Datatype) -> BasicValueEnum<'ctx> {
        if lhs.is_int_value() {
            if Datatype::get_unsigned_types().contains(datatype) {
                return BasicValueEnum::IntValue(
                    self.builder.build_int_unsigned_div::<IntValue>(
                        lhs.into_int_value(), 
                        rhs.into_int_value(), 
                        "integer.unsigned.div_")
                );
            } else {
                return BasicValueEnum::IntValue(
                    self.builder.build_int_signed_div(
                        lhs.into_int_value(), 
                        rhs.into_int_value(), 
                        "integer.signed.div_")
                );
            }
        } else if lhs.is_float_value() {
            return BasicValueEnum::FloatValue(
                self.builder.build_float_div(
                    lhs.into_float_value(), 
                    rhs.into_float_value(), 
                    "floating.div_")
            );
        }
        unimplemented!();
    }

    fn mod_(&mut self, lhs: BasicValueEnum<'ctx>, rhs: BasicValueEnum<'ctx>, datatype: &Datatype) -> BasicValueEnum<'ctx> {
        if lhs.is_int_value() {
            if Datatype::get_unsigned_types().contains(datatype) {
                return BasicValueEnum::IntValue(
                    self.builder.build_int_unsigned_rem::<IntValue>(
                        lhs.into_int_value(), 
                        rhs.into_int_value(), 
                        "integer.unsigned.mod_")
                );
            } else {
                return BasicValueEnum::IntValue(
                    self.builder.build_int_signed_rem(
                        lhs.into_int_value(), 
                        rhs.into_int_value(), 
                        "integer.signed.mod_")
                );
            }
        } else if lhs.is_float_value() {
            return BasicValueEnum::FloatValue(
                self.builder.build_float_rem(
                    lhs.into_float_value(), 
                    rhs.into_float_value(), 
                    "floating.mod_")
            );
        }
        unimplemented!();
    }

    // only integer and bool
    fn bitwise_and(&mut self, lhs: BasicValueEnum<'ctx>, rhs: BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx> {
        if lhs.is_int_value() {
            return BasicValueEnum::IntValue(self.builder.build_and(
                lhs.into_int_value(), 
                rhs.into_int_value(), 
                "integer.and_")
            );
        } 
        unimplemented!();
    }

    // only integer and bool
    fn bitwise_or(&mut self, lhs: BasicValueEnum<'ctx>, rhs: BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx> {
        if lhs.is_int_value() {
            return BasicValueEnum::IntValue(
                self.builder.build_or(
                    lhs.into_int_value(), 
                    rhs.into_int_value(), 
                    "integer.or_")
            );
        }
        unimplemented!();
    }

    // only integer and bool
    fn bitwise_xor(&mut self, lhs: BasicValueEnum<'ctx>, rhs: BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx> {
        if lhs.is_int_value() {
            return BasicValueEnum::IntValue(
                self.builder.build_xor(
                    lhs.into_int_value(), 
                    rhs.into_int_value(), 
                    "")
            );
        }
        unimplemented!();
    }

    // only integer and bool
    fn left_shift(&mut self, lhs: BasicValueEnum<'ctx>, rhs: BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx> {
        return BasicValueEnum::IntValue(
            self.builder.build_left_shift(
                lhs.into_int_value(), 
                rhs.into_int_value(), 
                "integer.leftshift_"
            )
        );
        unimplemented!();
    }

    // only integer and bool
    fn right_shift(&mut self, lhs: BasicValueEnum<'ctx>, rhs: BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx> {
        return BasicValueEnum::IntValue(
            self.builder.build_right_shift(
                lhs.into_int_value(), 
                rhs.into_int_value(), 
                true, 
                "integer.rightshift_.signextended_"
            )
        );
        unimplemented!();
    }

    // only integer and bool.
    /// Needs testing for sure.
    fn bitwise_not(&mut self, lhs: BasicValueEnum<'ctx>, datatype: &Datatype) -> BasicValueEnum<'ctx> {
        if lhs.is_int_value() {
            
            return BasicValueEnum::IntValue(
                lhs.into_int_value()
                .const_xor(
                    lhs.get_type()
                    .into_int_type()
                    .const_int(
                        u64::MAX, 
                        Datatype::get_int_types().contains(datatype)
                    )
                )
            );
        }
        unimplemented!();
    }

    /// All comparision ops are using Ordered for float ops. 
    /// i.e., neither operands are QNAN. 
    fn equal_equal(&mut self, lhs: BasicValueEnum<'ctx>, rhs: BasicValueEnum<'ctx>, datatype: &Datatype) 
    -> BasicValueEnum<'ctx> {
        if lhs.is_int_value() {
            if Datatype::get_signed_types().contains(datatype) {
                return BasicValueEnum::IntValue(
                    self.builder.build_int_compare(
                        inkwell::IntPredicate::EQ, 
                        lhs.into_int_value(), 
                        rhs.into_int_value(), 
                        "integer.signed.eqeq_")
                );
            } else {
                return BasicValueEnum::IntValue(
                    self.builder.build_int_compare(
                        inkwell::IntPredicate::EQ, 
                        lhs.into_int_value(), 
                        rhs.into_int_value(), 
                        "integer.unsigned.eqeq_")
                );
            }
        } else if lhs.is_float_value() {
            return BasicValueEnum::IntValue(
                self.builder.build_float_compare(
                    inkwell::FloatPredicate::OEQ, 
                    lhs.into_float_value(), 
                    rhs.into_float_value(), 
                    "float.eqeq_")
            );
        }
        unimplemented!();
    }

    fn bang_equal(&mut self, lhs: BasicValueEnum<'ctx>, rhs: BasicValueEnum<'ctx>, datatype: &Datatype) 
    -> BasicValueEnum<'ctx> {
        if lhs.is_int_value() {
            if Datatype::get_signed_types().contains(datatype) {
                return BasicValueEnum::IntValue(
                    self.builder.build_int_compare(
                        inkwell::IntPredicate::NE, 
                        lhs.into_int_value(), 
                        rhs.into_int_value(), 
                        "integer.signed.not_eq_")
                );
            } else {
                return BasicValueEnum::IntValue(
                    self.builder.build_int_compare(
                        inkwell::IntPredicate::NE, 
                        lhs.into_int_value(), 
                        rhs.into_int_value(), 
                        "integer.unsigned.not_eq_")
                );
            }
        } else if lhs.is_float_value() {
            return BasicValueEnum::IntValue(
                self.builder.build_float_compare(
                    inkwell::FloatPredicate::ONE, 
                    lhs.into_float_value(), 
                    rhs.into_float_value(), 
                    "float.not_eq_")
            );
        }
        unimplemented!();
    }

    fn less_than(&mut self, lhs: BasicValueEnum<'ctx>, rhs: BasicValueEnum<'ctx>, datatype: &Datatype) 
    -> BasicValueEnum<'ctx> {
        if lhs.is_int_value() {
            if Datatype::get_signed_types().contains(datatype) {
                return BasicValueEnum::IntValue(
                    self.builder.build_int_compare(
                        inkwell::IntPredicate::SLT, 
                        lhs.into_int_value(), 
                        rhs.into_int_value(), 
                        "integer.signed.less_")
                );
            } else {
                return BasicValueEnum::IntValue(
                    self.builder.build_int_compare(
                        inkwell::IntPredicate::ULT, 
                        lhs.into_int_value(), 
                        rhs.into_int_value(), 
                        "integer.unsigned.less_")
                );
            }
        } else if lhs.is_float_value() {
            return BasicValueEnum::IntValue(
                self.builder.build_float_compare(
                    inkwell::FloatPredicate::OLT, 
                    lhs.into_float_value(), 
                    rhs.into_float_value(), 
                    "float.less_")
            );
        }
        unimplemented!();
    }

    fn less_equal(&mut self, lhs: BasicValueEnum<'ctx>, rhs: BasicValueEnum<'ctx>, datatype: &Datatype) 
    -> BasicValueEnum<'ctx> {
        if lhs.is_int_value() {
            if Datatype::get_signed_types().contains(datatype) {
                return BasicValueEnum::IntValue(
                    self.builder.build_int_compare(
                        inkwell::IntPredicate::SLE, 
                        lhs.into_int_value(), 
                        rhs.into_int_value(), 
                        "integer.signed.less_eq_")
                );
            } else {
                return BasicValueEnum::IntValue(
                    self.builder.build_int_compare(
                        inkwell::IntPredicate::ULE, 
                        lhs.into_int_value(), 
                        rhs.into_int_value(), 
                        "integer.unsigned.less_eq_")
                );
            }
        } else if lhs.is_float_value() {
            return BasicValueEnum::IntValue(
                self.builder.build_float_compare(
                    inkwell::FloatPredicate::OLE, 
                    lhs.into_float_value(), 
                    rhs.into_float_value(), 
                    "float.less_eq_")
            );
        }
        unimplemented!();
    }

    fn greater_than(&mut self, lhs: BasicValueEnum<'ctx>, rhs: BasicValueEnum<'ctx>, datatype: &Datatype) 
    -> BasicValueEnum<'ctx> {
        if lhs.is_int_value() {
            if Datatype::get_signed_types().contains(datatype) {
                return BasicValueEnum::IntValue(
                    self.builder.build_int_compare(
                        inkwell::IntPredicate::SGT, 
                        lhs.into_int_value(), 
                        rhs.into_int_value(), 
                        "integer.signed.greater_")
                );
            } else {
                return BasicValueEnum::IntValue(
                    self.builder.build_int_compare(
                        inkwell::IntPredicate::UGT, 
                        lhs.into_int_value(), 
                        rhs.into_int_value(), 
                        "integer.unsigned.greater_")
                );
            }
        } else if lhs.is_float_value() {
            return BasicValueEnum::IntValue(
                self.builder.build_float_compare(
                    inkwell::FloatPredicate::OGT, 
                    lhs.into_float_value(), 
                    rhs.into_float_value(), 
                    "float.greater_")
            );
        }
        unimplemented!();
    }

    fn greater_equal(&mut self, lhs: BasicValueEnum<'ctx>, rhs: BasicValueEnum<'ctx>, datatype: &Datatype) 
    -> BasicValueEnum<'ctx> {
        if lhs.is_int_value() {
            if Datatype::get_signed_types().contains(datatype) {
                return BasicValueEnum::IntValue(
                    self.builder.build_int_compare(
                        inkwell::IntPredicate::SGE, 
                        lhs.into_int_value(), 
                        rhs.into_int_value(), 
                        "integer.signed.greater_eq_")
                );
            } else {
                return BasicValueEnum::IntValue(
                    self.builder.build_int_compare(
                        inkwell::IntPredicate::UGE, 
                        lhs.into_int_value(), 
                        rhs.into_int_value(), 
                        "integer.unsigned.greater_eq_")
                );
            }
        } else if lhs.is_float_value() {
            return BasicValueEnum::IntValue(
                self.builder.build_float_compare(
                    inkwell::FloatPredicate::OGE, 
                    lhs.into_float_value(), 
                    rhs.into_float_value(), 
                    "float.greater_eq_")
            );
        }
        unimplemented!();
    }

    /// Needs testing definitely
    fn unary_minus(&mut self, operand: BasicValueEnum<'ctx>, datatype: &Datatype) -> BasicValueEnum<'ctx> {
        if operand.is_int_value() {
            return BasicValueEnum::IntValue(operand.into_int_value().const_neg());
        } else if operand.is_float_value() {
            return BasicValueEnum::FloatValue(operand.into_float_value().const_neg());
        }
        unimplemented!();
    }

    fn cast_int_to_float(&mut self, var_value: BasicValueEnum<'ctx>, from_dtype: &Datatype, to_dtype: &Datatype)
    -> BasicValueEnum<'ctx> {
        if Datatype::is_signed_int(from_dtype) && Datatype::is_float(to_dtype) {

            let dtype = self.get_llvm_type(to_dtype).into_float_type();
            return BasicValueEnum::FloatValue(self.builder.build_signed_int_to_float(
                var_value.into_int_value(), 
                dtype,
                // self.get_llvm_type(to_dtype).into_float_type(), 
                "cast.signed_to_float_"
            ));

        } else /*if Datatype::is_unsigned_int(from_dtype) && Datatype::is_float(to_dtype)*/ {

            let dtype = self.get_llvm_type(to_dtype).into_float_type();
            
            return BasicValueEnum::FloatValue(
                self.builder.build_unsigned_int_to_float(
                    var_value.into_int_value(), 
                    dtype, 
                    "cast.unsigned_to_float_")
            );
            
        } 
        unimplemented!();
    }

    fn cast_int_to_int(&mut self, var_value: BasicValueEnum<'ctx>, from_dtype: &Datatype, to_dtype: &Datatype)
    -> BasicValueEnum<'ctx> {

        if Datatype::lhs_has_more_width(from_dtype, to_dtype) {
            return BasicValueEnum::IntValue(
                self.builder.build_int_truncate_or_bit_cast(
                    var_value.into_int_value(), 
                    self.get_llvm_type(to_dtype).into_int_type(), 
                    "cast.truncate_or_bitcast_"
                )
            );
        } else {
            if Datatype::is_signed_int(from_dtype) {
                return BasicValueEnum::IntValue(
                    self.builder.build_int_s_extend_or_bit_cast(
                        var_value.into_int_value(), 
                        self.get_llvm_type(to_dtype).into_int_type(), 
                        "cast.sext_or_bitcast_"
                    )
                );
            } else {
                return BasicValueEnum::IntValue(
                    self.builder.build_int_z_extend_or_bit_cast(
                        var_value.into_int_value(), 
                        self.get_llvm_type(to_dtype).into_int_type(), 
                        "cast.zext_or_bitcast_"
                    )
                );
            }
            
        }

        // unimplemented!();
    }

    fn cast_float_to_int(&mut self, var_value: BasicValueEnum<'ctx>, from_dtype: &Datatype, to_dtype: &Datatype)
    -> BasicValueEnum<'ctx> {
        if Datatype::is_signed_int(to_dtype) {
            return BasicValueEnum::IntValue(
                self.builder.build_float_to_signed_int(
                    var_value.into_float_value(), 
                    self.get_llvm_type(to_dtype).into_int_type(), 
                    "cast.float_to_signed_"
                )
            );
        } else /*if Datatype::is_unsigned_int(to_dtype)*/ {
            return BasicValueEnum::IntValue(
                self.builder.build_float_to_unsigned_int(
                    var_value.into_float_value(), 
                    self.get_llvm_type(to_dtype).into_int_type(), 
                    "cast.float_to_unsigned_"
                )
            );
        }
        // unimplemented!();
    }

    fn cast_float_to_float(&mut self, var_value: BasicValueEnum<'ctx>, from_dtype: &Datatype, to_dtype: &Datatype) 
    -> BasicValueEnum<'ctx> {
        if Datatype::lhs_has_more_width(from_dtype, to_dtype) {
            return BasicValueEnum::FloatValue(
                self.builder.build_float_trunc(
                    var_value.into_float_value(), 
                    self.get_llvm_type(to_dtype).into_float_type(), 
                    "cast.float64_to_float32_"
                )
            );
        } else {
            return BasicValueEnum::FloatValue(
                self.builder.build_float_cast(
                    var_value.into_float_value(), 
                    self.get_llvm_type(to_dtype).into_float_type(), 
                    "cast.float32_to_float64_"
                )
            );
        }
        // unimplemented!();
    }


    fn get_radix_type(&self, value: &TokenType) -> StringRadix {
        match value {
            TokenType::K_INT8|TokenType::K_INT16|TokenType::K_INT32|TokenType::K_INT64|
            TokenType::K_UINT8|TokenType::K_UINT16|TokenType::K_UINT32|TokenType::K_UINT64
                => StringRadix::Decimal,
            TokenType::HEX_LITERAL => StringRadix::Hexadecimal,
            TokenType::OCTAL_LITERAL => StringRadix::Octal,
            TokenType::STRING_LITERAL => StringRadix::Alphanumeric,
            _ => StringRadix::Decimal
        }
    }


    fn call_function(
        &mut self, 
        name: String, 
        object: Option<BasicValueEnum<'ctx>>, 
        object_name: Option<String>, 
        arguments: &Vec<Box<Expr>>)
    -> inkwell::values::BasicValueEnum<'ctx> {
        let function = self.module.get_function(name.as_str()).unwrap();
        // let function = self.symbol_table.functions_table.get(name.as_str()).unwrap();
        let mut args = vec![];

        if object_name.is_some() {
            let obj_name = object_name.unwrap();
            // pass object as reference in function call.
            println!("Symbol Table: {:#?}", self.symbol_table);
            args.push(object.unwrap());
            // args.push(BasicValueEnum::PointerValue(*self.symbol_table.variable_table.get(obj_name.as_str()).unwrap()));
        }

        for argument in arguments {
            args.push(self.visit_expr(argument));
        }
        
        let call_val =  self.builder.build_call(
            function, 
            &args, 
            (name+"..call").as_str()
        ).try_as_basic_value(); 
        
        return call_val.left().unwrap();
    }


    fn visit_object_assignment_expr(
        &mut self, 
        target: &Box<Expr>, 
        operator: &Token, 
        expr: &Box<Expr>,
        datatype: &Datatype
    ) -> inkwell::values::BasicValueEnum<'ctx> {

        let is_lvalue_parsing = self.is_parsing_lvalue;
        self.is_parsing_lvalue = true;
        let lhs_ptr = self.visit_expr(target);
        self.is_parsing_lvalue = is_lvalue_parsing;
        println!("Codegen-ObjAssignExpr: LHS: {:#?}", lhs_ptr);

        if let Datatype::object{name} = datatype {
            if let Decl::StructDecl{name, fields:fields_decl} 
                = self.symbol_table.struct_decls.get(name).unwrap()
            {
                
                
                println!("Codegen-ObjAssignExpr: StructDecl: {:#?}", fields_decl.clone());
                println!("Codegen-ObjAssignExpr: Expr: {:#?}", expr.clone());
                let mut field_name_index_map = HashMap::new();
                for (i, (field_name, _)) in fields_decl.into_iter().enumerate() {
                    field_name_index_map.insert(field_name.value.clone(), i);
                }

                if let Expr::StructExpr{struct_name, fields, datatype}
                    = &**expr 
                {
                    println!("Codegen-ObjAssignExpr: Fields: {:#?}", fields.clone());
                    for (i, (field_name, field_expr)) in fields.into_iter().enumerate() {
                        let igep = self.builder.build_struct_gep(
                            lhs_ptr.into_pointer_value(), 
                            *field_name_index_map.get(&field_name.value).unwrap() as u32, 
                            field_name.value.as_str()
                        ).unwrap();
                        let field_val = self.visit_expr(field_expr);
                        self.builder.build_store(igep, field_val);
                    }

                }
            }
        }

        if self.is_parsing_lvalue {
            return lhs_ptr;
        } else {
            return self.builder.build_load(lhs_ptr.into_pointer_value(), "ptr.load");
        }
        // unimplemented!();
    }


    fn visit_struct_decl(&mut self, name: &Token, fields: &Vec<(Token, Token)>)
    -> Option<inkwell::values::AnyValueEnum<'ctx>> {
        self.symbol_table.struct_decls.insert(
            name.value.clone(), 
            Decl::StructDecl{name:name.clone(), fields: fields.clone()}
        );
        let mut field_types = vec![];
        for (_, dtype) in fields {
            field_types.push(self.get_type(dtype));
        }

        if let Some(struct_type) = self.module.get_struct_type(&name.value) {
            struct_type.set_body(&field_types, false);
        } else {
            let struct_type = self.context.opaque_struct_type(&name.value);
            struct_type.set_body(&field_types, false);
        }

        return None;
        // unimplemented!();
    }

    fn visit_prototype_decl(&mut self, name: &Token, parameters: &Vec<(Token, Token)>, returntype: &Token)
    -> Option<inkwell::values::AnyValueEnum<'ctx>> {
        // TODO: add support for object references for struct methods.
        let ret_type = self.get_type(returntype);
        let mut param_types = vec![];
        let mut is_method = false;
        
        if let Scope::Impl{name: struct_name, trait_name} = &self.current_scope {
            // self.module.get_struct_type(struct_name).unwrap().ptr_type(inkwell::AddressSpace::Generic);
            param_types.push(self.module.get_struct_type(struct_name).unwrap().ptr_type(inkwell::AddressSpace::Generic).into());
            is_method = true;
        }

        for param in parameters {
            param_types.push(self.get_type(&param.1));
        }

        let param_types = param_types.as_slice();

        let fn_type = match ret_type {
            BasicTypeEnum::ArrayType(a) => a.fn_type(param_types, false),
            BasicTypeEnum::FloatType(f) => f.fn_type(param_types, false),
            BasicTypeEnum::IntType(i) => i.fn_type(param_types, false),
            BasicTypeEnum::PointerType(p)   => p.fn_type(param_types, false),
            BasicTypeEnum::StructType(s) => s.fn_type(param_types, false),
            BasicTypeEnum::VectorType(v) => v.fn_type(param_types, false)
        };

        let name = match &self.current_scope {
            globals::Scope::Global => name.value.clone(),
            globals::Scope::Impl { name:impl_name, trait_name } 
                => impl_name.clone() + "." + name.value.clone().as_str(),
            globals::Scope::Trait{name: trait_name} => trait_name.clone() + "." + name.value.clone().as_str()
        };
        println!("Codegen-Prototype: name:{}", name);
        println!("Codegen-Prototype: num_params:{}", fn_type.count_param_types());
        let fn_val = self.module.add_function(name.as_str(), fn_type, None);

        for (i, arg) in fn_val.get_param_iter().enumerate() {
            let param_name = if i==0 && is_method { "self" } else { parameters[i-1].0.value.as_str() };
            // let param_name = parameters[i].0.value.as_str();
            match param_types[i] {
                BasicTypeEnum::ArrayType(a) 
                    => arg.into_array_value().set_name(param_name),
                BasicTypeEnum::FloatType(f) 
                    => arg.into_float_value().set_name(param_name),
                BasicTypeEnum::IntType(i)   
                    => arg.into_int_value().set_name(param_name),
                BasicTypeEnum::PointerType(p)   
                    => arg.into_pointer_value().set_name(param_name),
                BasicTypeEnum::StructType(s) 
                    => arg.into_struct_value().set_name(param_name),
                BasicTypeEnum::VectorType(v) 
                    => arg.into_vector_value().set_name(param_name)
            }
        }

        return Some(AnyValueEnum::FunctionValue(fn_val));
        unimplemented!();
    }

    fn visit_funcdef_decl(&mut self, prototype: &Box<Decl>, block: &Box<Stmt>)
    -> Option<inkwell::values::AnyValueEnum<'ctx>> {
        // TODO: add support for object references for struct methods. Probably not required if already added in prototype decl
        // self.visit_decl(prototype);
        // self.visit_stmt(block);
        let proto_name; 
        let proto_args; 
        let proto_return;
        let mut is_method = false;

        if let Scope::Impl{..} = &self.current_scope {
            is_method = true;
        }

        match &**prototype {
            Decl::Prototype{name, parameters, returntype}
                => {
                    proto_name = name;
                    proto_args = parameters;
                    proto_return = returntype;
                },
            _ => {
                // print some error
                return None;
            }
        }
        // let function = self.visit_decl(prototype)?.into_function_value();
        let function = self.visit_prototype_decl(
            &proto_name, &proto_args, &proto_return)?.into_function_value();
        
        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        self.curr_fn_value = Some(function);


        let prev_env = self.symbol_table.clone();
        self.symbol_table.variable_table.reserve(function.count_params() as usize);

        
        
        for (i, arg) in function.get_param_iter().enumerate() {
            let arg_name = if i== 0 && is_method { "self".to_string() } else { proto_args[i-1].0.value.clone() };
            // let arg_type = self.get_type(&proto_args[i].1);
            let arg_type = arg.get_type();

            let builder_new = self.context.create_builder();
            let entry_block = self.curr_fn_value?.get_first_basic_block().unwrap();

            match entry.get_first_instruction() {
                Some(first_instr)   => builder_new.position_before(&first_instr),
                None => builder_new.position_at_end(entry)
            }

            let alloca = match arg_type {
                BasicTypeEnum::ArrayType(a) => builder_new.build_alloca(a, arg_name.as_str()),
                BasicTypeEnum::FloatType(f) => builder_new.build_alloca(f, arg_name.as_str()),
                BasicTypeEnum::IntType(i)   => builder_new.build_alloca(i, arg_name.as_str()),
                BasicTypeEnum::PointerType(p)   => builder_new.build_alloca(p, arg_name.as_str()),
                BasicTypeEnum::StructType(s)    => builder_new.build_alloca(s, arg_name.as_str()),
                BasicTypeEnum::VectorType(v)    => builder_new.build_alloca(v, arg_name.as_str())
            };
            // let alloca = self.create_entry_block_alloca(arg_name);

            self.builder.build_store(alloca, arg);
            // self.symbol_table.variable_table.insert(arg_name.clone(), BasicValueEnum::PointerValue(alloca));
            
            self.symbol_table.variable_table.insert(arg_name.clone(), alloca);
        }


        let body = self.visit_stmt(block)?;
        
        match body {
            // only basic blocks.
            AnyValueEnum::ArrayValue(a)             => {self.builder.build_return(Some(&a));},
            AnyValueEnum::FloatValue(f)             => {self.builder.build_return(Some(&f));},
            AnyValueEnum::IntValue(i)                 => {self.builder.build_return(Some(&i));},
            AnyValueEnum::PointerValue(p)          => {self.builder.build_return(Some(&p));},
            AnyValueEnum::StructValue(s)           => {self.builder.build_return(Some(&s));},
            AnyValueEnum::VectorValue(v)           => {self.builder.build_return(Some(&v));},
            AnyValueEnum::InstructionValue(_)
            |AnyValueEnum::FunctionValue(_)
            |AnyValueEnum::PhiValue(_) => {
                // self.builder.build_return(
                //     Some(
                //         &self.get_type(proto_return).const_zero()
                //     )
                // );
            }
        };

        self.symbol_table = prev_env;
        // self.symbol_table.func_table = 

        return Some(body);
        // if function.verify(true) {}
        // unimplemented!();
    }

    fn visit_impl_decl(&mut self, name: &Token, trait_name: &Option<Token>, funcs: &Vec<Box<Decl>>)
    -> Option<inkwell::values::AnyValueEnum<'ctx>>  { 
        let prev_scope = self.current_scope.clone();
        self.current_scope = globals::Scope::Impl{name: name.value.clone(), trait_name: String::new()};
        if let Some(tok) = trait_name {
            self.current_scope = globals::Scope::Impl{name: name.value.clone(), trait_name: tok.value.clone()};
        }

        for func in funcs {
            self.visit_decl(func);
        }

        self.current_scope = prev_scope.clone();
        return None;
        unimplemented!();
    }

    fn visit_trait_decl(&mut self, name: &Token, funcs: &Vec<Box<Decl>>)
    -> Option<inkwell::values::AnyValueEnum<'ctx>> { 
        let prev_scope = self.current_scope.clone();
        self.current_scope = globals::Scope::Trait{name: name.value.clone()};

        for func in funcs {
            self.visit_decl(func);
        }

        self.current_scope = prev_scope.clone();
        return None;
        unimplemented!();
    }

    fn visit_if_stmt(&mut self, 
        if_token: &Token, 
        condition: &Box<Expr>, 
        then_block: &Box<Stmt>,
        else_block: &Option<Box<Stmt>>
    ) -> Option<inkwell::values::AnyValueEnum<'ctx>> { 
        let current_fn = self.curr_fn_value.unwrap();

        let cond = self.visit_expr(condition).into_int_value();

        let then_bb = self.context.append_basic_block(current_fn, "then");
        let else_bb = self.context.append_basic_block(current_fn, "else");
        let cont_bb = self.context.append_basic_block(current_fn, "ifcont");

        self.builder.build_conditional_branch(cond, then_bb, else_bb);

        // build then block
        self.builder.position_at_end(then_bb);
        let then_ret = self.visit_stmt(then_block)?;
        self.builder.build_unconditional_branch(cont_bb);

        let then_bb = self.builder.get_insert_block().unwrap();

        // build else block
        self.builder.position_at_end(else_bb);
        let mut else_ret: AnyValueEnum;
        if else_block.is_some() {
            else_ret = self.visit_stmt(else_block.as_ref().unwrap())?;
        } else {
            else_ret = then_ret.clone();
        }
        self.builder.build_unconditional_branch(cont_bb);
        let else_bb = self.builder.get_insert_block().unwrap();

        // emti merge block
        self.builder.position_at_end(cont_bb);
        println!("Codegen-IfStmt: Then_val: {:#?}", then_ret);
        println!("Codegen-IfStmt: Else_val: {:#?}", else_ret);
        match then_ret {
            AnyValueEnum::ArrayValue(a) => {
                let phi = self.builder.build_phi(a.get_type(), "ifphi");
                phi.add_incoming(&[
                    (&a, then_bb),
                    (&else_ret.into_array_value(), else_bb)
                ]);
                println!("Codegen-IfStmt: {:#?}", phi.print_to_string().to_str());
                return Some(AnyValueEnum::ArrayValue(phi.as_basic_value().into_array_value()));
            },
            AnyValueEnum::FloatValue(f) => {
                let phi = self.builder.build_phi(f.get_type(), "ifphi");
                phi.add_incoming(&[
                    (&f, then_bb),
                    (&else_ret.into_float_value(), else_bb)
                ]);
                println!("Codegen-IfStmt: {:#?}", phi.print_to_string().to_str());
                return Some(AnyValueEnum::FloatValue(phi.as_basic_value().into_float_value()));
            },
            AnyValueEnum::IntValue(i)     => {
                let phi = self.builder.build_phi(i.get_type(), "ifphi");
                phi.add_incoming(&[
                    (&i, then_bb),
                    (&else_ret.into_int_value(), else_bb)
                ]);
                
                println!("Codegen-IfStmt: {:#?}", phi.print_to_string().to_str());
                return Some(AnyValueEnum::IntValue(phi.as_basic_value().into_int_value()));
            },
            AnyValueEnum::PointerValue(p)   => {
                let phi = self.builder.build_phi(p.get_type(), "ifphi");
                phi.add_incoming(&[
                    (&p, then_bb),
                    (&else_ret.into_pointer_value(), else_bb)
                ]);
                println!("Codegen-IfStmt: {:#?}", phi.print_to_string().to_str());
                return Some(AnyValueEnum::PointerValue(phi.as_basic_value().into_pointer_value()));

            },
            AnyValueEnum::StructValue(s) => {
                let phi = self.builder.build_phi(s.get_type(), "ifphi");
                phi.add_incoming(&[
                    (&s, then_bb),
                    (&else_ret.into_struct_value(), else_bb)
                ]);
                println!("Codegen-IfStmt: {:#?}", phi.print_to_string().to_str());
                return Some(AnyValueEnum::StructValue(phi.as_basic_value().into_struct_value()));
            },
            AnyValueEnum::VectorValue(v) => {
                let phi = self.builder.build_phi(v.get_type(), "ifphi");
                phi.add_incoming(&[
                    (&v, then_bb),
                    (&else_ret.into_vector_value(), else_bb)
                ]);
                println!("Codegen-IfStmt: {:#?}", phi.print_to_string().to_str());
                return Some(AnyValueEnum::VectorValue(phi.as_basic_value().into_vector_value()));
            },
            AnyValueEnum::PhiValue(p) => {
            },
            AnyValueEnum::InstructionValue(i) => {
            },
            AnyValueEnum::FunctionValue(f) => {}
        }
        // let phi = self.builder.build_phi(then_ret.get_type(), "ifphi");
        
        // phi.add_incoming(&[(&then_ret, then_bb), (&else_ret, else_bb)]);

        return None;
        unimplemented!(); 
    }

    fn visit_block_stmt(&mut self, statements: &Vec<Box<Stmt>>)
    -> Option<inkwell::values::AnyValueEnum<'ctx>> { 
        let mut return_val = None;
        for stmt in statements {
            return_val = self.visit_stmt(stmt);
        }
        return return_val;
        // unimplemented!(); 
    }

    fn visit_return_stmt(&mut self, expr: &Box<Expr>)
    -> Option<inkwell::values::AnyValueEnum<'ctx>> { 
        let expr_val = self.visit_expr(expr);
        return Some(
            AnyValueEnum::InstructionValue(
                match expr_val {
                    BasicValueEnum::ArrayValue(a) => self.builder.build_return(Some(&a)),
                    BasicValueEnum::FloatValue(f) => self.builder.build_return(Some(&f)),
                    BasicValueEnum::IntValue(i) => self.builder.build_return(Some(&i)),
                    BasicValueEnum::PointerValue(p) => self.builder.build_return(Some(&p)),
                    BasicValueEnum::StructValue(s)  => self.builder.build_return(Some(&s)),
                    BasicValueEnum::VectorValue(v) => self.builder.build_return(Some(&v))
                }
            )
        );

        
        // return Some(expr_val.as_any_value_enum());

    }

    fn visit_for_stmt(&mut self, 
        for_token: &Token, 
        initialization: &Option<Box<Expr>>,
        condition: &Option<Box<Expr>>,
        updation: &Option<Box<Expr>>,
        block: &Box<Stmt>
    ) -> Option<inkwell::values::AnyValueEnum<'ctx>> { 
        let current_fn = self.curr_fn_value.unwrap();

        let prev_env = self.symbol_table.clone();
        
        if initialization.is_some() {
            self.visit_expr(initialization.as_ref().unwrap());
        }

        let cond_bb = self.context.append_basic_block(current_fn, "condbr");
        let loop_bb = self.context.append_basic_block(current_fn, "loopbr");
        let cont_bb = self.context.append_basic_block(current_fn, "contbr");

        // jumping to condbr 
        self.builder.build_unconditional_branch(cond_bb);
        self.builder.position_at_end(cond_bb);

        // emitting condition to condbr and jumping to loop_bb if true
        if(condition.is_some()) {
            let condres = self.visit_expr(condition.as_ref().unwrap()).into_int_value();
            self.builder.build_conditional_branch(condres, loop_bb, cont_bb);
        } else {
            // infinite loop
            self.builder.build_unconditional_branch(loop_bb);
        }
        let cond_bb = self.builder.get_insert_block().unwrap();
        
        // emit body and updation in loopbr
        self.builder.position_at_end(loop_bb);
        self.visit_stmt(block);
        if updation.is_some() {
            self.visit_expr(updation.as_ref().unwrap());
        }

        // go unconditionally to condbr from loopbr
        self.builder.build_unconditional_branch(cond_bb);
        let loop_bb = self.builder.get_insert_block().unwrap();

        // switching to cont_bb
        self.builder.position_at_end(cont_bb);
                
        return None;
        // unimplemented!(); 
    }

    fn visit_expression_stmt(&mut self, expr: &Box<Expr>)
    -> Option<inkwell::values::AnyValueEnum<'ctx>> { 
        // unimplemented!(); 
        return Some(self.visit_expr(expr).as_any_value_enum());
    }

    fn visit_decl_stmt(&mut self, decl: &Box<Decl>)
    -> Option<inkwell::values::AnyValueEnum<'ctx>> { 
        // unimplemented!(); 
        return self.visit_decl(decl);
    }

    fn visit_var_stmt(&mut self, name: &Token, datatype: &Option<Token>, initialization_value: &Option<Box<Expr>>)
    -> Option<inkwell::values::AnyValueEnum<'ctx>> {

        if datatype.is_some() && initialization_value.is_some() {
            
            let dtype = self.get_type(datatype.as_ref().unwrap());
            let val_ptr = self.builder.build_alloca(dtype, &name.value);
    
            self.symbol_table.variable_table.insert(
                name.value.clone(), 
                val_ptr
            );

            if datatype.as_ref().unwrap().tok_type == TokenType::IDENTIFIER {
                // if struct, creating a custom assignment expression.
                println!("Codegen-VarStmt: name: {:#?}, datatype: {:#?}, init_value: {:#?}", 
                    name, datatype, initialization_value);
                let dtype = self.get_type(datatype.as_ref().unwrap());
                let dtype_datatype = Datatype::get_tok_datatype(datatype.as_ref().unwrap());
                let var_expr = Expr::Variable{
                    name: name.clone(), 
                    datatype: dtype_datatype.clone(),
                    struct_name: Some(datatype.as_ref().unwrap().value.clone())
                };

                let assignment_expr = Expr::Assignment{
                    target: Box::new(var_expr.clone()),
                    operator: Token { 
                        tok_type: TokenType::EQUAL, 
                        value: "=".to_string(), 
                        line: usize::MAX, 
                        col: usize::MAX 
                    },
                    expr: initialization_value.as_ref().unwrap().clone(),
                    datatype: dtype_datatype.clone()
                };
                
                let val = self.visit_expr(&assignment_expr).as_any_value_enum();
                println!("Codegen-VarStmt: VarExpr: {:#?}", var_expr);
                println!("Codegen-VarStmt: AssignmentExpr: {:#?}", assignment_expr);
                println!("Codegen-VarStmt: Result: {:#?}", val);
                return Some(val);
            }

            let val = self.visit_expr(initialization_value.as_ref().unwrap());
            match val {
                BasicValueEnum::ArrayValue(v)      => self.builder.build_store(val_ptr, v),
                BasicValueEnum::FloatValue(v)      => self.builder.build_store(val_ptr, v),
                BasicValueEnum::IntValue(v)          => self.builder.build_store(val_ptr, v),
                BasicValueEnum::PointerValue(v)   => self.builder.build_store(val_ptr, v),
                BasicValueEnum::StructValue(v)     => self.builder.build_store(val_ptr, v),
                BasicValueEnum::VectorValue(v)     => self.builder.build_store(val_ptr, v),
            };
            return Some(AnyValueEnum::PointerValue(val_ptr));
        }

        if initialization_value.is_some() {
            let val = self.visit_expr(initialization_value.as_ref().unwrap());
            let dtype = val.get_type();
            let val_ptr = self.builder.build_alloca(dtype, &name.value);
    
            self.symbol_table.variable_table.insert(
                name.value.clone(), 
                val_ptr
            );


            if let BasicValueEnum::StructValue(s) = val {
                // if struct, create a custom assignment expr. 
                // This will evaluate `initialization_value` again.

                let var_expr = Box::new(Expr::Variable{
                    name: name.clone(),
                    datatype: Datatype::object{name:s.get_type().get_name()?.to_str().unwrap().to_string()},
                    struct_name: Some(s.get_type().get_name()?.to_str().unwrap().to_string()),
                });

                let assignment_expr = Expr::Assignment{
                    target: var_expr,
                    operator: Token { 
                        tok_type: TokenType::EQUAL,  
                        value: "=".to_string(), 
                        line: usize::MAX, 
                        col: usize::MAX
                    },
                    expr: initialization_value.as_ref().unwrap().clone(),
                    datatype: Datatype::object{name:s.get_type().get_name()?.to_str().unwrap().to_string()},
                };

                let val = self.visit_expr(&assignment_expr);
                return Some(val.as_any_value_enum());
            }

            // if not, store the val already calculated
            match val {
                BasicValueEnum::ArrayValue(v)      => self.builder.build_store(val_ptr, v),
                BasicValueEnum::FloatValue(v)      => self.builder.build_store(val_ptr, v),
                BasicValueEnum::IntValue(v)          => self.builder.build_store(val_ptr, v),
                BasicValueEnum::PointerValue(v)   => self.builder.build_store(val_ptr, v),
                BasicValueEnum::StructValue(v)     => self.builder.build_store(val_ptr, v),
                BasicValueEnum::VectorValue(v)     => self.builder.build_store(val_ptr, v),
            };
            return Some(AnyValueEnum::PointerValue(val_ptr));
        }
            
        // self.builder.build_store(val_ptr, val.into());
        if datatype.is_some() {
            let dtype = self.get_type(datatype.as_ref().unwrap());
            let val_ptr = self.builder.build_alloca(dtype, &name.value);
            self.symbol_table.variable_table.insert(
                name.value.clone(), 
                val_ptr
            );
            
            return Some(AnyValueEnum::PointerValue(val_ptr));
        }
        
        unimplemented!(); 
    }

    fn visit_variable_expr(&mut self, name: &Token, datatype: &Datatype, struct_name: &Option<String>)
    -> inkwell::values::BasicValueEnum<'ctx> { 
        
        if let Some(var_ptr) = self.symbol_table.variable_table.get(&name.value.clone()) {
            if self.is_parsing_lvalue {
                // return BasicValueEnum::PointerValue(var_ptr.into_pointer_value());
                return BasicValueEnum::PointerValue(*var_ptr);
            }
            println!("Codegen-VariableExpr: {:#?}", var_ptr);
            return self.builder.build_load(
                // var_ptr.into_pointer_value(), 
                *var_ptr,
                (name.value.clone()+".load").as_str()
            ).into();
        }

        return BasicValueEnum::IntValue(self.context.i8_type().const_zero());
        unimplemented!(); 
    }

    fn visit_literal_expr(&mut self, value: &Token, datatype: &Datatype) 
    -> inkwell::values::BasicValueEnum<'ctx> { 
        return match datatype {
            Datatype::bool 
                => BasicValueEnum::IntValue(self.context.bool_type().const_int(1, false)),
            Datatype::uint8 | Datatype::int8
                => BasicValueEnum::<'ctx>::IntValue(
                    self.context.i8_type().const_int_from_string(
                        &value.value, 
                        self.get_radix_type(&value.tok_type)).unwrap()
                ),
            Datatype::uint16 | Datatype::int16 
                => BasicValueEnum::<'ctx>::IntValue(
                    self.context.i16_type().const_int_from_string(
                        &value.value, 
                        self.get_radix_type(&value.tok_type)).unwrap()
                ),
            Datatype::uint32 | Datatype::int32 
                => BasicValueEnum::<'ctx>::IntValue(
                    self.context.i32_type().const_int_from_string(
                        &value.value, 
                        self.get_radix_type(&value.tok_type)
                    ).unwrap()
                ),
            Datatype::uint64 | Datatype::int64 
                => BasicValueEnum::<'ctx>::IntValue(
                    self.context.i64_type().const_int_from_string(
                        &value.value, 
                        self.get_radix_type(&value.tok_type)
                    ).unwrap()
                ),
            Datatype::float32 => BasicValueEnum::<'ctx>::FloatValue(
                self.context.f32_type().const_float_from_string(
                    &value.value
                ),
            ),
            Datatype::float64 => BasicValueEnum::<'ctx>::FloatValue(
                self.context.f64_type().const_float_from_string(&value.value)
            ),
            Datatype::object{..} => /* *self.symbol_table.variable_table.get(&value.value).unwrap() */
                    BasicValueEnum::PointerValue(*self.symbol_table.variable_table.get(&value.value).unwrap()),
            _ => BasicValueEnum::<'ctx>::IntValue(self.context.bool_type().const_zero()),
            
        };
        // unimplemented!(); 
    }

    fn visit_call_expr(&mut self, callee: &Box<Expr>, arguments: &Vec<Box<Expr>>, datatype: &Datatype)
    -> inkwell::values::BasicValueEnum<'ctx> {

        match &**callee {
            Expr::AttributeRef{name, object, object_dtype, datatype}
                => {

                let is_parsing_lvalue = self.is_parsing_lvalue;
                self.is_parsing_lvalue = true;
                let obj = self.visit_expr(object);
                self.is_parsing_lvalue = is_parsing_lvalue;

                if let Datatype::object{name: obj_name} = object_dtype {
                    let func_name = obj_name.clone()+"."+name.value.as_str();
                    return self.call_function(func_name, Some(obj), Some(obj_name.clone()), arguments);
                } 

            },
            Expr::Variable{name, datatype, struct_name}
                => {
                    if struct_name.is_some() { 
                        // print error and return
                    }

                    return self.call_function(name.value.clone(), None, None, arguments);
                
            },
            _ => ()
        }

        unimplemented!(); 
    }

    fn visit_attributeref_expr(&mut self, 
        object: &Box<Expr>, 
        name: &Token, 
        obj_dtype: &Datatype,
        datatype: &Datatype)
    -> inkwell::values::BasicValueEnum<'ctx> {
        let is_parsing_lvalue = self.is_parsing_lvalue;
        self.is_parsing_lvalue = true;
        let obj_ptr = self.visit_expr(object);
        self.is_parsing_lvalue = is_parsing_lvalue;

        let mut attr_index = 0;
        let mut found_attr = false;
        if let Datatype::object{name: obj_name} = obj_dtype {
            if let Decl::StructDecl{name:struct_name, fields}
             = self.symbol_table.struct_decls.get(&obj_name.clone()).unwrap() {

                for (field_name, field_type) in fields {
                    if field_name.value == name.value {
                        found_attr = true;
                        break;
                    } else {
                        attr_index+=1;
                    }
                }

                if found_attr {

                    let ptr = self.builder.build_struct_gep(
                        obj_ptr.into_pointer_value(), 
                        attr_index, 
                        (obj_name.clone()+"."+name.value.as_str()+"..ptr").as_str()
                    ).unwrap();
                    if self.is_parsing_lvalue {
                        return BasicValueEnum::PointerValue(ptr);
                    } else {
                        let val = self.builder.build_load(
                            ptr, 
                            (obj_name.clone()+"."+name.value.as_str()).as_str()
                        );
                        return val;
                    }

                }

            }

            

            let attr_name = name.clone();
            let impl_decls = self.symbol_table.impl_decls.get(obj_name).unwrap();
            for impl_decl in impl_decls {
                if let Decl::ImplDecl{name: struct_name, trait_name, funcs}
                    = &**impl_decl {

                    for func in funcs {

                        if let Decl::FuncDef{prototype, block} = &**func {

                            if let Decl::Prototype{name:func_name, parameters, returntype} = &**prototype {

                                if func_name.value == attr_name.value {
                                    // need to return functionvalue but basicvalueenum doesnt have it.
                                    self.module.get_function((obj_name.clone()+"."+func_name.value.as_str()).as_str()).unwrap();
                                }

                            }

                        }

                    }

                }
            }

        }

        // print some error.
        
        unimplemented!();
    }

    fn visit_binary_expr(&mut self, lhs: &Box<Expr>, rhs: &Box<Expr>, operator: &Token, datatype: &Datatype)
    -> inkwell::values::BasicValueEnum<'ctx> { 
        let mut lhs_val = self.visit_expr(lhs);
        let mut rhs_val = self.visit_expr(rhs);

        match operator.tok_type {
            TokenType::PLUS         =>  return self.add_(lhs_val, rhs_val),
            TokenType::MINUS        =>  return self.sub_(lhs_val, rhs_val),
            TokenType::ASTERISK     =>  return self.mul_(lhs_val, rhs_val),
            TokenType::SLASH        =>  return self.div_(lhs_val, rhs_val, datatype),
            TokenType::MOD          =>  return self.mod_(lhs_val, rhs_val, datatype),
            TokenType::BITWISE_AND  =>  return self.bitwise_and(lhs_val, rhs_val),
            TokenType::BITWISE_OR   =>  return self.bitwise_or(lhs_val, rhs_val),
            TokenType::BITWISE_XOR  =>  return self.bitwise_xor(lhs_val, rhs_val),
            TokenType::LEFT_SHIFT   =>  return self.left_shift(lhs_val, rhs_val),
            TokenType::RIGHT_SHIFT  =>  return self.right_shift(lhs_val, rhs_val),

            TokenType::EQUAL_EQUAL  =>  return self.equal_equal(lhs_val, rhs_val, datatype),
            TokenType::BANG_EQUAL   =>  return self.bang_equal(lhs_val, rhs_val, datatype),
            TokenType::LESS_THAN    =>  return self.less_than(lhs_val, rhs_val, datatype),
            TokenType::LESS_EQUAL   =>  return self.less_equal(lhs_val, rhs_val, datatype),
            TokenType::GREAT_THAN   =>  return self.greater_than(lhs_val, rhs_val, datatype),
            TokenType::GREAT_EQUAL  =>  return self.greater_equal(lhs_val, rhs_val, datatype),

            // operands are i1, result will be i1.
            TokenType::K_AND        =>  return self.bitwise_and(lhs_val, rhs_val),
            TokenType::K_OR         =>  return self.bitwise_or(lhs_val, rhs_val),
            _ => ()
        }
        unimplemented!(); 
    }

    fn visit_unary_expr(&mut self, operator: &Token, operand: &Box<Expr>, datatype: &Datatype)
    -> inkwell::values::BasicValueEnum<'ctx> { 
        let oprnd = self.visit_expr(operand);
        return match operator.tok_type {
            TokenType::BANG         => self.bitwise_not(oprnd, datatype),
            TokenType::MINUS        => self.unary_minus(oprnd, datatype),
            TokenType::BITWISE_NOT  => self.bitwise_not(oprnd, datatype),
            TokenType::PLUS         => oprnd,
            _                       => oprnd,
        };
        // unimplemented!(); 
    }

    fn visit_struct_expr(&mut self, struct_name: &Token, fields: &Vec<(Token, Box<Expr>)>, datatype: &Datatype)
    -> inkwell::values::BasicValueEnum<'ctx> {
        let struct_type = self.module.get_struct_type(struct_name.value.as_str()).unwrap();
        let struct_decl = self.symbol_table.struct_decls.get(&struct_name.value).unwrap();
        
        let mut field_name_index_map = HashMap::new();
        
        if let Decl::StructDecl{name, fields: field_decls} = struct_decl {
            for (i, (field_name, field_type)) in field_decls.into_iter().enumerate() {
                field_name_index_map.insert(field_name.value.clone(), i);
            }
        }

        let mut struct_field_vals = vec![];
        // println!("");
        struct_field_vals.reserve(field_name_index_map.len());
        for (field_name, field_expr) in fields {
            
            let field_val = self.visit_expr(field_expr);
            println!("COdegen-structexpr-Fieldval: {:#?}, clone: {:#?}", field_val, field_val.clone());
            // let field_v = match field_val {
            //     BasicValueEnum::ArrayValue(a) 
            //     => BasicValueEnum::ArrayValue(a),
            //     BasicValueEnum::FloatValue(f) 
            //     => BasicValueEnum::FloatValue(f)
            // };

            struct_field_vals.insert(
                *field_name_index_map.get(&field_name.value).unwrap(), 
                field_val.clone());
        }

        
        return BasicValueEnum::StructValue(struct_type.const_named_struct(&struct_field_vals));
        unimplemented!(); 
    }

    fn visit_assignment_expr(&mut self, target: &Box<Expr>, operator: &Token, expr: &Box<Expr>, datatype: &Datatype)
    -> inkwell::values::BasicValueEnum<'ctx> { 

        if let Datatype::object{..} = datatype {
            return self.visit_object_assignment_expr(target, operator, expr, datatype);
        }

        self.is_parsing_lvalue = true;
        let lhs_ptr = self.visit_expr(target); /* Should be pointer */
        self.is_parsing_lvalue = false;
        let rhs_val = self.visit_expr(expr);
        println!("Codegen-AssignmentExpr: lhs_ptr.is_ptr()={}", lhs_ptr.is_pointer_value());

        match operator.tok_type {
            TokenType::EQUAL => {
                println!("Codegen-AssignmentExpr: TokenType::EQUAL");
                self.builder.build_store(
                lhs_ptr.into_pointer_value(), 
                rhs_val
            );},
            TokenType::PLUS_EQUAL => {
                let lhs_val = self.builder.build_load(lhs_ptr.into_pointer_value(), "plus_eq");
                self.builder.build_store(
                    lhs_ptr.into_pointer_value(), 
                    self.add_(lhs_val, rhs_val)
                );
            },
            TokenType::MINUS_EQUAL => {
                let lhs_val = self.builder.build_load(lhs_ptr.into_pointer_value(), "minus_eq");
                self.builder.build_store(lhs_ptr.into_pointer_value(), self.sub_(lhs_val, rhs_val));
            },
            TokenType::ASTERISK_EQUAL => {
                let lhs_val = self.builder.build_load(lhs_ptr.into_pointer_value(), "mul_eq");
                self.builder.build_store(lhs_ptr.into_pointer_value(), self.mul_(lhs_val, rhs_val));
            },
            TokenType::SLASH_EQUAL => {
                let lhs_val = self.builder.build_load(lhs_ptr.into_pointer_value(), "div_eq");
                self.builder.build_store(
                    lhs_ptr.into_pointer_value(), 
                    self.div_(lhs_val, rhs_val, datatype)
                );
            },
            TokenType::MOD_EQUAL => {
                let lhs_val = self.builder.build_load(lhs_ptr.into_pointer_value(), "mod_eq");
                self.builder.build_store(
                    lhs_ptr.into_pointer_value(), 
                    self.mod_(lhs_val, rhs_val, datatype)
                );
            },
            TokenType::BITWISE_OR_EQUAL => {
                let lhs_val = self.builder.build_load(lhs_ptr.into_pointer_value(), "bit_or_eq");
                self.builder.build_store(
                    lhs_ptr.into_pointer_value(), 
                    self.bitwise_or(lhs_val, rhs_val)
                );
            },
            TokenType::BITWISE_AND_EQUAL => {
                let lhs_val = self.builder.build_load(lhs_ptr.into_pointer_value(), "bit_and_eq");
                self.builder.build_store(
                    lhs_ptr.into_pointer_value(), 
                    self.bitwise_and(lhs_val, rhs_val)
                );
            },
            TokenType::BITWISE_XOR_EQUAL => {
                let lhs_val = self.builder.build_load(lhs_ptr.into_pointer_value(), "bit_xor_eq");
                self.builder.build_store(
                    lhs_ptr.into_pointer_value(), 
                    self.bitwise_xor(lhs_val, rhs_val)
                );
            },
            TokenType::LEFT_SHIFT_EQUAL => {
                let lhs_val = self.builder.build_load(lhs_ptr.into_pointer_value(), "leftsh_eq");
                self.builder.build_store(
                    lhs_ptr.into_pointer_value(), 
                    self.left_shift(lhs_val, rhs_val)
                );
            },
            TokenType::RIGHT_SHIFT_EQUAL => {
                let lhs_val = self.builder.build_load(lhs_ptr.into_pointer_value(), "rightsh_eq");
                self.builder.build_store(
                    lhs_ptr.into_pointer_value(), 
                    self.right_shift(lhs_val, rhs_val)
                );
            },
            _ => ()
        }

        return rhs_val;
        // unimplemented!(); 
    }

    fn visit_grouping_expr(&mut self, expr: &Box<Expr>, datatype: &Datatype) 
    -> inkwell::values::BasicValueEnum<'ctx> { 
        return self.visit_expr(expr);
        // unimplemented!(); 
    }

    fn visit_cast_expr(&mut self, 
        variable: &Box<Expr>, 
        cast_type: &Token, 
        from_dtype: &Datatype, 
        to_dtype: &Datatype)
    -> inkwell::values::BasicValueEnum<'ctx> { 
        let var_value = self.visit_expr(variable);
        
        if var_value.is_int_value() {
            if /*Datatype::is_int(from_dtype) &&*/ Datatype::is_float(to_dtype) {
                return self.cast_int_to_float(var_value, from_dtype, to_dtype);
            }
            else if Datatype::is_int(to_dtype) {
                return self.cast_int_to_int(var_value, from_dtype, to_dtype);
            } else {
                unimplemented!();
            }
        } else if var_value.is_float_value() {
            if Datatype::is_int(to_dtype) {
                return self.cast_float_to_int(var_value, from_dtype, to_dtype);
            } else if Datatype::is_float(to_dtype) {
                return self.cast_float_to_float(var_value, from_dtype, to_dtype);
            } else {
                unimplemented!();
            }
        } else {
            unimplemented!(); 
        }
    }

    fn visit_exprlist_expr(&mut self, expr_list: &Vec<Box<Expr>>, datatype: &Datatype)
    -> inkwell::values::BasicValueEnum<'ctx> { 
        if expr_list.len() == 0 {
            unimplemented!();
        }

        let reply = self.visit_expr(&expr_list[0]);
        let mut i = 1;
        while i < expr_list.len() {
            self.visit_expr(&expr_list[i]);
            i += 1;
        }

        return reply;
        unimplemented!(); 
    }
}