use std::collections::HashMap;
use crate::lexer::Token;
use crate::ast::*;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub variable_table: HashMap<String, Token>, /* key: variable name, value: Datatype string */
    pub struct_decls: HashMap<String, Decl>, /* key: struct name, value: Declaration */
    pub impl_decls: HashMap<String, Decl>,      /* key: struct name, value: Declaration */
    pub func_table: HashMap<String, Decl> /* key: Function name, value: return type list */
}