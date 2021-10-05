use std::collections::HashMap;
use inkwell::values::PointerValue;

use crate::ast::*;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    // pub variable_table: HashMap<String, Token>, /* key: variable name, value: Datatype string */
    pub variable_table: HashMap<String, Datatype>, /* key: variable name, value: Datatype string */
    pub struct_decls: HashMap<String, Decl>, /* key: struct name, value: Declaration */
    pub impl_decls: HashMap<String, Vec<Box<Decl>>>,      /* key: struct name, value: Declaration */
    pub trait_decls: HashMap<String, Vec<Box<Decl>>>,
    pub func_table: HashMap<String, Decl> /* key: Function name, value: return type list */
}

#[derive(Debug, Clone)]
pub struct IRSymbolTable<'ctx> {
    // pub variable_table: HashMap<String, BasicValueEnum<'ctx>>,
    pub variable_table: HashMap<String, PointerValue<'ctx>>,
    // pub functions_table: HashMap<String, inkwell::values::FunctionValue<'ctx>>,
    pub struct_decls: HashMap<String, Decl>,
    pub impl_decls: HashMap<String, Vec<Box<Decl>>>,
    pub trait_decls: HashMap<String, Vec<Box<Decl>>>,
    pub func_table: HashMap<String, Decl>
}