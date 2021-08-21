use crate::lexer::Token;

#[allow(non_camel_case_types,dead_code)]
#[derive(Debug)]
pub enum Datatype {
    int8, int16, int32, int64,
    uint8, uint16, uint32, uint64,
    float32, float64, bool, object,
    string, yet_to_infer
}

#[allow(non_camel_case_types,dead_code)]
#[derive(Debug)]
pub enum Primary_Type {
    Hex_literal,
    Octal_literal,
    Float_literal,
    Int_literal,
    String_literal,
    Bool_literal,
    Identifier,
    Error
}



#[allow(dead_code)]
#[derive(Debug)]
pub enum Decl {
    Prototype   {name:Token, parameters: Vec<(Token/*name*/, Token/*datatype*/)>, returntypes: Vec<Token>},
    FuncDef     {prototype: Box<Decl>/*Prototype*/, block: Box<Stmt>/*Block*/},
    // FuncDecl    {prototype}
    StructDecl  {name: Token, fields: Vec<(Token/*name*/, Token/*datatype*/)>},
    ImplDecl    {name: Token, trait_name: Option<Token>, funcs: Vec<Box<Decl>>/*FuncDef*/},
    TraitDecl   {name: Token, funcs: Vec<Box<Decl>>/*FuncDef/Prototype*/},
    // Program     {decls: Vec<Box<Decl>>}
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Stmt {
    If      {condition: Box<Expr>, 
                then_block: Box<Stmt>, /* Should be Block */
                else_block: Option<Box<Stmt>> /*Else block could be an if statement*/},
    Block   {statements: Vec<Box<Stmt>>},
    Return  {expr_list: Vec<Box<Expr>>},
    While   {condition: Box<Expr>, block: Box<Stmt>},
    For     {initialization: Box<Expr> /*should be Assignment */, 
                condition: Box<Expr>, 
                updation: Vec<Box<Expr>>, 
                block: Box<Stmt>},  
    Expression  {expr: Box<Expr>},
    Decl    { decl: Box<Decl> },
    Var     { name: Token, datatype: Option<Token>, initialization_value: Option<Box<Expr>>}
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Expr {
    Variable  {name: Token, datatype: Datatype, struct_name: Option<String>/* Valid only if datatype is obj */},
    Literal   {value: Token, datatype: Datatype},
    Call        {callee: Box<Expr>/*Expr=Identifier*/, arguments: Vec<Box<Expr>>, datatype: Datatype},
    AttributeRef{object: Box<Expr>, name: Token, datatype: Datatype},
    Binary      {lhs: Box<Expr>, rhs: Box<Expr>, operator: Token, datatype: Datatype},
    Unary       {operator: Token, operand: Box<Expr>, datatype: Datatype},
    StructExpr  {struct_name: Token, fields: /*Vec<Expr>*/Vec<(Token, Box<Expr>)>, datatype: Datatype},
    Assignment  {target_list: Vec<Box<Expr>>, /*Expr=Expr::Identifier */
        expr_list: Vec<Box<Expr>>, datatype: Datatype},  
    Grouping    { expr: Box<Expr> },
    Cast        { variable: Box<Expr>, cast_type: Token}
}