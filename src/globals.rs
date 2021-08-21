#[allow(non_camel_case_types,dead_code)]
#[derive(Debug,PartialEq,Clone)]
pub enum TokenType {
    ERROR,

    // keywords
    K_VAR, K_STRUCT, K_IMPL, K_TRAIT, K_FUNC, K_OR, K_AND, K_IF, K_ELSE, K_FOR, K_WHILE,
    K_RETURN, K_AS,/* K_SELF */

    // datatypes
    K_INT8, K_INT16, K_INT32, K_INT64,
    K_UINT8, K_UINT16, K_UINT32, K_UINT64,
    K_FLOAT32, K_FLOAT64, K_BOOL, /*OBJECT_TYPE,*/
    K_NULL,

    K_TRUE, K_FALSE,

    IDENTIFIER, 

    INT_LITERAL, HEX_LITERAL, OCTAL_LITERAL, STRING_LITERAL, FLOAT_LITERAL,
    SEMICOLON, COLON, COMMA, DOT, UNDERSCORE,

    // operators
    PLUS, PLUS_EQUAL,
    MINUS, MINUS_EQUAL,
    ASTERISK, ASTERISK_EQUAL,
    SLASH, SLASH_EQUAL,
    MOD, MOD_EQUAL,
    EQUAL, EQUAL_EQUAL,
    BANG, BANG_EQUAL,
    
    LESS_THAN, LESS_EQUAL,
    GREAT_THAN, GREAT_EQUAL,
    
    BITWISE_XOR, BITWISE_XOR_EQUAL,
    BITWISE_AND, BITWISE_AND_EQUAL,
    BITWISE_OR, BITWISE_OR_EQUAL,
    BITWISE_NOT,
    
    LEFT_SHIFT, LEFT_SHIFT_EQUAL,
    RIGHT_SHIFT, RIGHT_SHIFT_EQUAL,
    
    RIGHT_ARROW, LEFT_ARROW,
    DOLLAR, HASH,

    CURLY_OPEN, CURLY_CLOSE,
    BRACKET_OPEN, BRACKET_CLOSE,
    SQUARE_OPEN, SQUARE_CLOSE,

    FILE_EOF
}


impl TokenType {
    pub fn get_datatypes() -> Vec<TokenType> {
        vec![
            TokenType::K_INT8,      TokenType::K_UINT8,
            TokenType::K_INT16,     TokenType::K_UINT16,
            TokenType::K_INT32,     TokenType::K_UINT32,
            TokenType::K_INT64,     TokenType::K_UINT64,
            TokenType::K_FLOAT32,   TokenType::K_FLOAT64,
            TokenType::K_BOOL,      TokenType::IDENTIFIER
        ]
    }

    pub fn get_assignment_ops() -> Vec<TokenType> {
        vec![
            TokenType::EQUAL, 
            TokenType::PLUS_EQUAL, 
            TokenType::MINUS_EQUAL,
            TokenType::ASTERISK_EQUAL,
            TokenType::SLASH_EQUAL,
            TokenType::MOD_EQUAL,
            TokenType::BITWISE_AND_EQUAL,
            TokenType::BITWISE_OR_EQUAL,
            TokenType::BITWISE_XOR_EQUAL,
            TokenType::LEFT_SHIFT_EQUAL,
            TokenType::RIGHT_SHIFT_EQUAL
        ]
    }

    pub fn get_unary_ops() -> Vec<TokenType> {
        vec![
            TokenType::BANG,
            TokenType::BITWISE_XOR,
            TokenType::MINUS,
            TokenType::PLUS
        ]
    }

    pub fn get_equality_ops() -> Vec<TokenType> {
        vec![
            TokenType::EQUAL_EQUAL,
            TokenType::BANG_EQUAL
        ]
    }

    pub fn get_relational_ops() -> Vec<TokenType> {
        vec![
            TokenType::GREAT_THAN,
            TokenType::GREAT_EQUAL,
            TokenType::LESS_THAN,
            TokenType::LESS_EQUAL
        ]
    }

    pub fn get_shift_ops() -> Vec<TokenType> {
        vec![
            TokenType::LEFT_SHIFT,
            TokenType::RIGHT_SHIFT
        ]
    }

    pub fn get_additive_ops() -> Vec<TokenType> {
        vec![
            TokenType::PLUS,
            TokenType::MINUS
        ]
    }

    pub fn get_multiplicative_ops() -> Vec<TokenType> {
        vec![
            TokenType::ASTERISK,
            TokenType::SLASH,
            TokenType::MOD
        ]
    }

    pub fn get_literal_types() -> Vec<TokenType> {
        vec![
            TokenType::HEX_LITERAL,
            TokenType::INT_LITERAL,
            TokenType::FLOAT_LITERAL,
            TokenType::OCTAL_LITERAL,
            TokenType::STRING_LITERAL,
            TokenType::K_TRUE,
            TokenType::K_FALSE
        ]
    }
}