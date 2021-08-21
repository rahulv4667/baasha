use std::collections::HashMap;

pub struct SymbolTable {
    variable_table: HashMap<String, Token>,
    struct_decls: HashMap<String, Decl>
}