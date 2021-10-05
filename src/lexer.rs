use unicode_segmentation::UnicodeSegmentation;
use crate::globals::TokenType;
use crate::logger;
use std::fmt;

#[allow(dead_code)]
// #[derive(Debug,Clone)]
#[derive(Clone)]
pub struct Token {
    pub tok_type: TokenType,
    pub value: String,
    pub line: usize,
    pub col: usize
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter)   -> fmt::Result {
        return write!(f, "Token{{ tok_type: {:?}, value: {:?} }}", self.tok_type, self.value);
    }
}

#[allow(dead_code)]
pub struct Lexer {
    col: usize,
    line_num: usize,
    current: usize,
    start: usize,
    program: String,
    str_vec: Vec<String>,
    tokens: Vec<Token>
}



#[allow(dead_code)]
impl Lexer {
    pub fn new() -> Self {
        Lexer {
            col: 0,
            line_num: 1,
            current: 0,
            start: 0,
            program: String::new(),
            str_vec: Vec::new(),
            tokens: Vec::new()
        }
    }


    fn peek(&self) -> String {
        if self.current >= self.str_vec.len() {
            return String::from("\0");
        }
        return self.str_vec[self.current].clone();
    }


    fn peek_next(&self) -> String {
        if self.current >= self.str_vec.len()-1 {
            return String::from("\0");
        }
        return self.str_vec[self.current+1].clone();
    }



    fn advance(&mut self) -> String {
        self.current += 1;
        self.col += 1;
        return self.str_vec[self.current-1].clone();
    } 



    fn curr(&self) -> String {
        return self.str_vec[self.current - 1].clone();
    }


    fn is_end(&self) -> bool {
        return self.curr() == "\0";
    }


    fn match_(&mut self, c: &str) -> bool {
        if self.peek() == c {
            return self.advance() == c;
        }
        return false;
    }


    fn increment_line(&mut self) {
        self.line_num += 1;
        self.col = 0;
    }


    fn is_alpha(c: String) -> bool {
        return match c.as_str() {
            "A"|"B"|"C"|"D"|"E"|"F"|"G"|"H"|"I"|"J"|"K"|"L"|"M"|
            "N"|"O"|"P"|"Q"|"R"|"S"|"T"|"U"|"V"|"W"|"X"|"Y"|"Z"     => true,
            "a"|"b"|"c"|"d"|"e"|"f"|"g"|"h"|"i"|"j"|"k"|"l"|"m"|
            "n"|"o"|"p"|"q"|"r"|"s"|"t"|"u"|"v"|"w"|"x"|"y"|"z"     => true,
            _ => false
        }
    }

    fn is_digit(c: String) -> bool {
        return match c.as_str() {
            "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9" => true,
            _ => false
        }
    }


    fn is_hex(c: String) -> bool {
        return match c.as_str() {
            "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9"|"A"|"B"|"C"|"D"|"E"|"F"|"a"|"b"|"c"|"d"|"e"|"f" => true,
            _ => false
        }
    }

    fn is_octal(c: String) -> bool {
        return match c.as_str() {
            "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7" => true,
            _ => false
        }
    }


    fn number(&mut self) {
        let c: String = self.peek();

        // checking if hex
        if self.curr() == "0" && (c == "x" || c == "X") {
            self.advance();
            
            while Self::is_hex(self.peek()) && !self.is_end() { self.advance(); }
            self.tokens.push(Token{
                tok_type: TokenType::HEX_LITERAL,
                value: (&self.str_vec[self.start..self.current]).join(""),
                line: self.line_num,
                col: self.col
            });

            return;
        }

        // checking if octal
        if self.curr() == "0" && (c == "o" || c == "O") {
            self.advance();

            while Self::is_octal(self.peek()) && !self.is_end() { self.advance(); }

            self.tokens.push(Token{
                tok_type: TokenType::OCTAL_LITERAL,
                value: (&self.str_vec[self.start..self.current]).join(""),
                line: self.line_num,
                col: self.col
            });

            return;
        }

        // differentiating float and integer
        let mut is_int: bool;
        while Self::is_digit(self.peek()) && !self.is_end() { self.advance(); }
        if self.is_end() {
            logger::log_message(logger::LogLevel::ERROR, self.col, 
                self.line_num, "Unterminated integer/float literal".to_string());
        }

        is_int = !self.match_(".");

        while Self::is_digit(self.peek()) && !self.is_end() { self.advance(); }
        if self.is_end() {
            logger::log_message(logger::LogLevel::ERROR, self.col, 
                self.line_num, "Unterminated integer/float literal".to_string());
        }

        #[allow(unused_must_use)]
        if self.match_("e") || self.match_("E") {
            is_int = false;
            self.match_("+") || self.match_("-");
        }

        while Self::is_digit(self.peek()) && !self.is_end() { self.advance(); }
        if self.is_end() {
            logger::log_message(logger::LogLevel::ERROR, self.col, 
                self.line_num, "Unterminated integer/float literal".to_string());
        }

        self.tokens.push(Token{
            tok_type: if is_int {TokenType::INT_LITERAL} else {TokenType::FLOAT_LITERAL},
            value: (&self.str_vec[self.start..self.current]).join(""),
            col: self.col,
            line: self.line_num
        });

        return;
    }


    fn string_(&mut self) {
        while self.peek() != "\"" && !self.is_end() {
            if self.peek() == "\n" { self.increment_line(); }
            self.advance();
        }

        if self.curr() == "\0" {
            logger::log_message(logger::LogLevel::ERROR, self.col, self.line_num, "Unterminated string".to_string());
        }

        // closing quote
        self.advance();
        self.tokens.push(Token { 
            tok_type: TokenType::STRING_LITERAL, 
            value: (&self.str_vec[self.start..self.current]).join(""), 
            line: self.line_num, 
            col: self.col 
        });
    }



    fn identifier(&mut self) {
        while Self::is_digit(self.peek()) || Self::is_alpha(self.peek()) || self.peek() == "_" && !self.is_end() {
            self.advance();
        }

        if self.is_end() {
            logger::log_message(logger::LogLevel::ERROR, self.col, self.line_num, "Unterminated identifier".to_string());
        }

        return self.recognize_identifier();
    }


    fn recognize_identifier(&mut self) {
        let idntfr: String = (&self.str_vec[self.start..self.current]).join("");

        let tok_type: TokenType = match idntfr.as_str() {
            "and"           => TokenType::K_AND,
            "as"            => TokenType::K_AS,
            
            "bool"          => TokenType::K_BOOL,
            
            "else"          => TokenType::K_ELSE,
            
            "for"           => TokenType::K_FOR,
            "func"          => TokenType::K_FUNC,
            "false"         => TokenType::K_FALSE,
            "float32"       => TokenType::K_FLOAT32,
            "float64"       => TokenType::K_FLOAT64,

            "if"            => TokenType::K_IF,
            "impl"          => TokenType::K_IMPL,
            "int8"          => TokenType::K_INT8,
            "int16"         => TokenType::K_INT16,
            "int32"         => TokenType::K_INT32,
            "int64"         => TokenType::K_INT64,

            "null"          => TokenType::K_NULL,

            "or"            => TokenType::K_OR,

            "return"        => TokenType::K_RETURN,

            "struct"        => TokenType::K_STRUCT,

            "true"          => TokenType::K_TRUE,
            "trait"         => TokenType::K_TRAIT,

            "uint8"         => TokenType::K_UINT8,
            "uint16"        => TokenType::K_UINT16,
            "uint32"        => TokenType::K_UINT32,
            "uint64"        => TokenType::K_UINT64,

            "var"           => TokenType::K_VAR,

            "while"         => TokenType::K_WHILE,

            // _               => {
            //     // if identifier, check if it is a struct or impl name.
            //     let last_tok: Option<&Token> = self.tokens.last();
            //     let ttype: TokenType = match last_tok {
            //         Some(Token{tok_type: TokenType::K_STRUCT, ..}) |
            //         Some(Token{tok_type: TokenType::K_IMPL,   ..}) |
            //         Some(Token{tok_type: TokenType::K_TRAIT,  ..}) |
            //         Some(Token{tok_type: TokenType::K_FOR,    ..}) |
            //         Some(Token{tok_type: TokenType::IDENTIFIER, ..}) => TokenType::OBJECT_TYPE,
            //         _   =>  TokenType::IDENTIFIER
            //     };
            //     ttype
            // }
            _               => TokenType::IDENTIFIER,
        };


        self.tokens.push(Token{
            tok_type,
            value: idntfr,
            col: self.col,
            line: self.line_num
        });
       
        
    }


    fn skip_non_code(&mut self) {
        loop {
            let s: String = self.peek();

            match s.as_str() {
                
                " " => {
                    self.current += 1;
                    self.col += 1;
                }
                
                "\t" => {
                    self.current+=1;
                    self.col += 1;
                }
                
                "\r" => {
                    self.advance();
                }
                
                "\n" => {
                    self.current += 1;
                    self.col = 0;
                    self.line_num += 1;
                }

                "/" => {
                    if self.peek_next() == "/" {
                        while self.peek() != "\n" && self.str_vec[self.current] != "\0" {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }

                _ => { return; }
            }
        }
    }


    fn scan_token(&mut self) {
        self.skip_non_code();
        self.start = self.current;

        if self.current > 0 && self.curr() == "\0" {
            self.tokens.push(Token{ tok_type: TokenType::FILE_EOF, value: String::new(), col: self.col, line: self.line_num});
            return;
        }

        let c: String = self.advance();

        if Self::is_digit(c.clone())                        { return self.number(); }
        if Self::is_alpha(c.clone()) || c == "_"    { return self.identifier(); }

        let tok_type = match c.as_str() {
            "("         => TokenType::BRACKET_OPEN,
            ")"         => TokenType::BRACKET_CLOSE,
            "{"         => TokenType::CURLY_OPEN,
            "}"         => TokenType::CURLY_CLOSE,
            "["         => TokenType::SQUARE_OPEN,
            "]"         => TokenType::SQUARE_CLOSE,
            "="         => if self.match_("=")  {TokenType::EQUAL_EQUAL}        else {TokenType::EQUAL},
            "*"         => if self.match_("=")  {TokenType::ASTERISK_EQUAL}     else {TokenType::ASTERISK},
            "/"         => if self.match_("=")  {TokenType::SLASH_EQUAL}        else {TokenType::SLASH},
            "+"         => if self.match_("=")  {TokenType::PLUS_EQUAL}         else {TokenType::PLUS},
            "-"         => if self.match_("=")  {TokenType::MINUS_EQUAL}        
                            else if self.match_(">") {TokenType::RIGHT_ARROW}
                            else  {TokenType::MINUS},
            "|"         => if self.match_("=")  {TokenType::BITWISE_OR_EQUAL}   else {TokenType::BITWISE_OR},
            "&"         => if self.match_("=")  {TokenType::BITWISE_AND_EQUAL}  else {TokenType::BITWISE_AND},
            "^"         => if self.match_("=")  {TokenType::BITWISE_XOR_EQUAL}  else {TokenType::BITWISE_XOR},
            "!"         => if self.match_("=")  {TokenType::BANG_EQUAL}         else {TokenType::BANG},
            "<"         => if self.match_("=")  {TokenType::LESS_EQUAL}         
                            else if self.match_("<") {
                                if self.peek() == "=" {TokenType::LEFT_SHIFT_EQUAL}
                                else {TokenType::LEFT_SHIFT}
                            } else if self.match_("-") {
                                TokenType::LEFT_ARROW
                            }
                            else {TokenType::LESS_THAN},
            ">"         => if self.match_("=")  {TokenType::GREAT_EQUAL}
                            else if self.match_(">")    {
                                // not peek next coz already advanced in match_
                                if self.peek() == "=" {TokenType::RIGHT_SHIFT_EQUAL}
                                else {TokenType::RIGHT_SHIFT}
                            }
                            else {TokenType::GREAT_THAN},
            "~"         => TokenType::BITWISE_NOT,
            "$"         => TokenType::DOLLAR,
            "#"         => TokenType::HASH,
            ";"         => TokenType::SEMICOLON,
            ":"         => TokenType::COLON,
            ","         => TokenType::COMMA,
            "."         => TokenType::DOT,
            "_"         => TokenType::UNDERSCORE,
            "\""        => { self.string_(); TokenType::ERROR }
            _           => TokenType::ERROR 
        };

        if tok_type != TokenType::ERROR {
            self.tokens.push(Token { 
                tok_type: tok_type, 
                value: (&self.str_vec[self.start..self.current]).join(""), 
                line: self.line_num, 
                col: self.col
            });
        }
        
    }

    
    pub fn tokenize(&mut self, program: String) -> Vec<Token> {
        self.program = program;
        // let char_vec = self.program.graphemes(true).collect::<Vec<&str>>();
        self.str_vec = Vec::new();
        
        for chr in self.program.graphemes(true) {
            self.str_vec.push(String::from(chr));
        }
        

        while self.current < self.str_vec.len() {
            self.scan_token();
        }

        self.tokens.push(Token { tok_type: TokenType::FILE_EOF, value: String::new(), line: usize::MAX, col: usize::MAX });
        
    
        return self.tokens.clone();
    }

    pub fn print_tokens(&self) {
        for token in &self.tokens {
            println!("{:?}", token);
        }
    }
}

