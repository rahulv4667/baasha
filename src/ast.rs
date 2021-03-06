use crate::lexer::Token;
use crate::globals::TokenType;

#[allow(non_camel_case_types,dead_code)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Datatype {
    int8, int16, int32, int64,
    uint8, uint16, uint32, uint64,
    float32, float64, bool, 
    object{name: String},
    function{name: String, obj_name: Option<String>, returntype: Box<Datatype>, param_types: Vec<Box<Datatype>>},
    string, yet_to_infer
}

impl Datatype {

    #[allow(dead_code)]
    pub fn get_tok_datatype(tok: &Token) -> Self {
        if let Datatype::object{..} = Datatype::get_datatype(&tok.tok_type) {
            return Datatype::object{name: tok.value.clone()};
        }
        return Datatype::get_datatype(&tok.tok_type);
    }

    #[allow(dead_code)]
    pub fn get_datatype(tok_type: &TokenType) -> Self {
        return match tok_type {
            TokenType::K_INT8 => Datatype::int8,
            TokenType::K_INT16 => Datatype::int16,
            TokenType::K_INT32 => Datatype::int32,
            TokenType::K_INT64 => Datatype::int64,
            TokenType::K_UINT8 => Datatype::uint8,
            TokenType::K_UINT16 => Datatype::uint16,
            TokenType::K_UINT32 => Datatype::uint32,
            TokenType::K_UINT64 => Datatype::uint64,
            TokenType::K_FLOAT32 => Datatype::float32,
            TokenType::K_FLOAT64 => Datatype::float64,
            TokenType::K_BOOL => Datatype::bool,
            TokenType::INT_LITERAL => Datatype::int64,
            TokenType::HEX_LITERAL => Datatype::int64,
            TokenType::OCTAL_LITERAL => Datatype::int64,
            TokenType::STRING_LITERAL => Datatype::string,
            TokenType::FLOAT_LITERAL => Datatype::float64,
            TokenType::IDENTIFIER => Datatype::object{name: String::new()},
            _ => Datatype::yet_to_infer
        };
    }

    #[allow(dead_code)]
    pub fn get_int_types() -> Vec<Self> {
        vec![
            Datatype::int8,     Datatype::uint8,
            Datatype::int16,    Datatype::uint16,
            Datatype::int32,    Datatype::uint32,
            Datatype::int64,    Datatype::uint64 
        ]
    }

    #[allow(dead_code)]
    pub fn get_float_types() -> Vec<Self> {
        vec![Datatype::float32, Datatype::float64]
    }

    #[allow(dead_code)]
    pub fn get_signed_types() -> Vec<Self> {
        vec![Datatype::int8, Datatype::int16, Datatype::int32, Datatype::int64]
    }

    #[allow(dead_code)]
    pub fn get_unsigned_types() -> Vec<Self> {
        vec![Datatype::uint8, Datatype::uint16, Datatype::uint32, Datatype::uint64]
    }

    #[allow(dead_code)]
    pub fn is_unsigned_int(dtype: &Datatype) -> bool {
        match dtype {
            Datatype::uint8|Datatype::uint16|Datatype::uint32|Datatype::uint64 
                => true,
            _   => false
        }
    }

    #[allow(dead_code)]
    pub fn is_signed_int(dtype: &Datatype) -> bool {
        match dtype {
            Datatype::int8|Datatype::int16|Datatype::int32|Datatype::int64 
                => true,
            _   => false,
        }
    }

    #[allow(dead_code)]
    pub fn is_int(dtype: &Datatype) -> bool {
        Datatype::is_signed_int(dtype) || Datatype::is_unsigned_int(dtype) //|| *dtype == Datatype::bool
    }

    #[allow(dead_code)]
    pub fn is_float(dtype: &Datatype) -> bool {
        match dtype {
            Datatype::float32 | Datatype::float64
                => true,
            _   => false
        }
    }

    #[allow(dead_code)]
    pub fn is_object(dtype: &Datatype) -> bool {
        match dtype {
            Datatype::object{..}    => true,
            _   => false
        }
    }

    #[allow(dead_code)]
    pub fn is_width_64(dtype: &Datatype) -> bool {
        match dtype {
            Datatype::int64|Datatype::uint64|Datatype::float64  => true,
            _   => false
        }
    }

    #[allow(dead_code)]
    pub fn is_width_32(dtype: &Datatype) -> bool {
        match dtype {
            Datatype::int32|Datatype::uint32|Datatype::float32  => true,
            _   => false
        }
    }

    #[allow(dead_code)]
    pub fn is_width_16(dtype: &Datatype) -> bool {
        match dtype {
            Datatype::int16|Datatype::uint16  => true,
            _   => false
        }
    }

    #[allow(dead_code)]
    pub fn is_width_8(dtype: &Datatype) -> bool {
        match dtype {
            Datatype::int8|Datatype::uint8  => true,
            _   => false
        }
    }

    /// Checks if width of `l_type` is more than `r_type`
    #[allow(dead_code)]
    pub fn lhs_has_more_width(l_type: &Datatype, r_type: &Datatype) -> bool {
        if Datatype::is_width_64(l_type) && !Datatype::is_width_64(r_type)      { true }
        else if Datatype::is_width_32(l_type) && !Datatype::is_width_32(r_type) { true }
        else if Datatype::is_width_16(l_type) && !Datatype::is_width_16(r_type) { true }
        else if Datatype::is_width_8(l_type) && !Datatype::is_width_8(r_type)   { true }
        else { false }
        // unimplemented!();
    }
}


#[allow(non_camel_case_types,dead_code)]
#[derive(Debug,Clone, Copy)]
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
#[derive(Debug,Clone)]
pub enum Decl {
    Prototype   {name:Token, parameters: Vec<(Token/*name*/, Token/*datatype*/)>, returntype: /*Vec<Token>*/Token},
    FuncDef     {prototype: Box<Decl>/*Prototype*/, block: Box<Stmt>/*Block*/},
    // FuncDecl    {prototype}
    StructDecl  {name: Token, fields: Vec<(Token/*name*/, Token/*datatype*/)>},
    ImplDecl    {name: Token, trait_name: Option<Token>, funcs: Vec<Box<Decl>>/*FuncDef*/},
    TraitDecl   {name: Token, funcs: Vec<Box<Decl>>/*FuncDef/Prototype*/},
    // Program     {decls: Vec<Box<Decl>>}
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub enum Stmt {
    If      {   if_token: Token,
                condition: Box<Expr>, 
                then_block: Box<Stmt>, /* Should be Block */
                else_block: Option<Box<Stmt>> /*Else block could be an if statement*/},
    Block   {statements: Vec<Box<Stmt>>},
    Return  {expr: Box<Expr>},
    While   {condition: Box<Expr>, block: Box<Stmt>},
    For     {   for_token: Token,
                initialization: Option<Box<Expr>> /*should be Assignment */, 
                condition: Option<Box<Expr>>, 
                updation: Option<Box<Expr>>, 
                block: Box<Stmt>},  
    Expression  {expr: Box<Expr>},
    Decl    { decl: Box<Decl> },
    Var     { name: Token, datatype: Option<Token>, initialization_value: Option<Box<Expr>>}
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expr {
    Variable  {name: Token, datatype: Datatype, struct_name: Option<String>/* Valid only if datatype is obj */},
    Literal   {value: Token, datatype: Datatype},
    Call        {callee: Box<Expr>/*Expr=Identifier*/, arguments: Vec<Box<Expr>>/*arguments: Box<Expr>*//*Expr= ExprList*/, 
                    datatype: Datatype},
    AttributeRef{object: Box<Expr>, name: Token, object_dtype: Datatype, datatype: Datatype},
    Binary      {lhs: Box<Expr>, rhs: Box<Expr>, operator: Token, datatype: Datatype},
    Unary       {operator: Token, operand: Box<Expr>, datatype: Datatype},
    StructExpr  {struct_name: Token, fields: /*Vec<Expr>*/Vec<(Token, Box<Expr>)>, datatype: Datatype},
    // Assignment  {target_list: Vec<Box<Expr>>, /*Expr=Expr::Identifier */
    //     expr_list: Vec<Box<Expr>>, datatype: Datatype},
    Assignment  {target: Box<Expr>, operator: Token, expr: Box<Expr>, datatype: Datatype},
    Grouping    { expr: Box<Expr>, datatype: Datatype },
    Cast        { variable: Box<Expr>, cast_type: Token, from_dtype: Datatype, to_dtype: Datatype},
    ExprList    { expr_list: Vec<Box<Expr>>, datatype: Datatype}
}