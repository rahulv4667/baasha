#ifndef __BAASHA_LEXER_CPP
#define __BAASHA_LEXER_CPP
#include "globals.hpp"

namespace Baasha {

    enum class TokenType {

        TOKEN_ERROR,

        // keywords
        K_VAR, K_STRUCT, K_IMPL, K_FUNC, K_OR, K_AND, K_IF, K_ELSE, K_FOR,
        K_RETURN,/* K_SELF,*/

        // datatypes
        K_INT32, K_INT64, K_INT16, K_INT8,
        K_UINT32, K_UINT64, K_UINT16, K_UINT8,
        K_FLOAT32, K_FLOAT64, K_BOOL, OBJECT_TYPE,
        K_NULL,

        K_TRUE, K_FALSE,

        IDENTIFIER,
        
        INT_LITERAL, FLOAT_LITERAL, HEX_LITERAL, OCTAL_LITERAL,
        STRING_LITERAL, SEMICOLON, COMMA,

        // operators
        PLUS, PLUS_EQUAL, 
        MINUS, MINUS_EQUAL, 
        ASTERISK, ASTERISK_EQUAL, 
        SLASH, SLASH_EQUAL, 
        MOD, MOD_EQUAL, 
        EQUAL, EQUAL_EQUAL, 
        BANG, BANG_EQUAL,
        CURLY_OPEN, CURLY_CLOSE, 
        BRACKET_OPEN, BRACKET_CLOSE, 
        SQUARE_OPEN, SQUARE_CLOSE,
        LESS_THAN, LESS_EQUAL, 
        GREAT_THAN, GREAT_EQUAL, 
        BITWISE_XOR, BITWISE_XOR_EQUAL, 
        BITWISE_OR, BITWISE_OR_EQUAL,
        BITWISE_AND, BITWISE_AND_EQUAL,
        DOLLAR, HASH,

        

        
        FILE_EOF
    };

    TokenType keywordEnumVal(std::string kword) {
        if(kword == "int8") return TokenType::K_INT8;
        else if(kword == "int16")               return TokenType::K_INT16;
        else if(kword == "int32")               return TokenType::K_INT32;
        else if(kword == "int64")               return TokenType::K_INT64;
        else if(kword == "uint8")               return TokenType::K_UINT8;
        else if(kword == "uint16")              return TokenType::K_UINT16;
        else if(kword == "uint32")              return TokenType::K_UINT32;
        else if(kword == "uint64")              return TokenType::K_UINT64;
        else if(kword == "float32")             return TokenType::K_FLOAT32;
        else if(kword == "float64")             return TokenType::K_FLOAT64;
        else if(kword == "bool")                return TokenType::K_BOOL;
        else if(kword == "var")                 return TokenType::K_VAR;
        else if(kword == "struct")              return TokenType::K_STRUCT;
        // else if(kword == "self")                return TokenType::K_SELF;
        else if(kword == "and")                 return TokenType::K_AND;
        else if(kword == "or")                  return TokenType::K_OR;
        else if(kword == "if")                  return TokenType::K_IF;
        else if(kword == "else")                return TokenType::K_ELSE;
        else if(kword == "for")                 return TokenType::K_FOR;
        else                                    return TokenType::OBJECT_TYPE;  // not keyword but helpful for sturct types.
    }

    std::string enumStringVal(TokenType type) {
        #define ENUM_STR_VAL(type) case type: return #type
        switch(type) {
            ENUM_STR_VAL(TokenType::ASTERISK);
            ENUM_STR_VAL(TokenType::ASTERISK_EQUAL);
            ENUM_STR_VAL(TokenType::BANG);
            ENUM_STR_VAL(TokenType::BANG_EQUAL);
            ENUM_STR_VAL(TokenType::BITWISE_AND);
            ENUM_STR_VAL(TokenType::BITWISE_AND_EQUAL);
            ENUM_STR_VAL(TokenType::BITWISE_OR);
            ENUM_STR_VAL(TokenType::BITWISE_OR_EQUAL);
            ENUM_STR_VAL(TokenType::BITWISE_XOR);
            ENUM_STR_VAL(TokenType::BITWISE_XOR_EQUAL);
            ENUM_STR_VAL(TokenType::BRACKET_CLOSE);
            ENUM_STR_VAL(TokenType::BRACKET_OPEN);
            ENUM_STR_VAL(TokenType::CURLY_CLOSE);
            ENUM_STR_VAL(TokenType::CURLY_OPEN);
            ENUM_STR_VAL(TokenType::DOLLAR);
            ENUM_STR_VAL(TokenType::EQUAL);
            ENUM_STR_VAL(TokenType::EQUAL_EQUAL);
            ENUM_STR_VAL(TokenType::FILE_EOF);
            ENUM_STR_VAL(TokenType::FLOAT_LITERAL);
            ENUM_STR_VAL(TokenType::GREAT_EQUAL);
            ENUM_STR_VAL(TokenType::GREAT_THAN);
            ENUM_STR_VAL(TokenType::HASH);
            ENUM_STR_VAL(TokenType::HEX_LITERAL);
            ENUM_STR_VAL(TokenType::IDENTIFIER);
            ENUM_STR_VAL(TokenType::INT_LITERAL);
            ENUM_STR_VAL(TokenType::K_AND);
            ENUM_STR_VAL(TokenType::K_BOOL);
            ENUM_STR_VAL(TokenType::K_ELSE);
            ENUM_STR_VAL(TokenType::K_FALSE);
            ENUM_STR_VAL(TokenType::K_FLOAT32);
            ENUM_STR_VAL(TokenType::K_FLOAT64);
            ENUM_STR_VAL(TokenType::K_FOR);
            ENUM_STR_VAL(TokenType::K_FUNC);
            ENUM_STR_VAL(TokenType::K_IF);
            ENUM_STR_VAL(TokenType::K_IMPL);
            ENUM_STR_VAL(TokenType::K_INT16);
            ENUM_STR_VAL(TokenType::K_INT32);
            ENUM_STR_VAL(TokenType::K_INT64);
            ENUM_STR_VAL(TokenType::K_INT8);
            ENUM_STR_VAL(TokenType::K_UINT16);
            ENUM_STR_VAL(TokenType::K_UINT32);
            ENUM_STR_VAL(TokenType::K_UINT64);
            ENUM_STR_VAL(TokenType::K_UINT8);
            ENUM_STR_VAL(TokenType::K_NULL);
            ENUM_STR_VAL(TokenType::K_OR);
            ENUM_STR_VAL(TokenType::K_RETURN);
            // ENUM_STR_VAL(TokenType::K_SELF);
            ENUM_STR_VAL(TokenType::K_STRUCT);
            ENUM_STR_VAL(TokenType::K_TRUE);
            ENUM_STR_VAL(TokenType::K_VAR);
            ENUM_STR_VAL(TokenType::LESS_EQUAL);
            ENUM_STR_VAL(TokenType::LESS_THAN);
            ENUM_STR_VAL(TokenType::MINUS);
            ENUM_STR_VAL(TokenType::MINUS_EQUAL);
            ENUM_STR_VAL(TokenType::MOD);
            ENUM_STR_VAL(TokenType::MOD_EQUAL);
            ENUM_STR_VAL(TokenType::OBJECT_TYPE);
            ENUM_STR_VAL(TokenType::OCTAL_LITERAL);
            ENUM_STR_VAL(TokenType::PLUS);
            ENUM_STR_VAL(TokenType::PLUS_EQUAL);
            ENUM_STR_VAL(TokenType::SEMICOLON);
            ENUM_STR_VAL(TokenType::SLASH);
            ENUM_STR_VAL(TokenType::SLASH_EQUAL);
            ENUM_STR_VAL(TokenType::SQUARE_CLOSE);
            ENUM_STR_VAL(TokenType::SQUARE_OPEN);
            ENUM_STR_VAL(TokenType::STRING_LITERAL);
            ENUM_STR_VAL(TokenType::TOKEN_ERROR);
            default:                                    return "SOMETHING";
        }
        #undef ENUM_STR_VAL
    }

    llvm::Type* getLLVMTypeRaw(TokenType type, const std::string& type_str = "") {
        switch(type) {
            case TokenType::K_UINT8:
            case TokenType::K_INT8:
                return llvm::IntegerType::get(*the_context, 8);

            case TokenType::K_INT16:
            case TokenType::K_UINT16:
                return llvm::IntegerType::get(*the_context, 16);

            case TokenType::K_UINT32:
            case TokenType::K_INT32:
                return llvm::IntegerType::get(*the_context, 32);

            case TokenType::K_INT64:
            case TokenType::K_UINT64:
                return llvm::IntegerType::get(*the_context, 64);
                    
            case TokenType::K_FLOAT32:
                return llvm::Type::getFloatTy(*the_context);

            case TokenType::K_FLOAT64:
                return llvm::Type::getDoubleTy(*the_context);
            case TokenType::OBJECT_TYPE:
                return getStructure(type_str);
        }
    }


    typedef struct Scanner {
        size_t start;
        size_t current;
        size_t line_num;
        size_t col;

        Scanner(const Scanner& scanner)
        :   start(scanner.start),
            current(scanner.current),
            line_num(scanner.line_num),
            col(scanner.col)
        {}

        Scanner()
        :   start(0), current(0), line_num(1), col(0) 
        {}
    } Scanner;
    

    struct Token {
        TokenType type;
        std::shared_ptr<Scanner> scan_point;
        Token(TokenType type, std::shared_ptr<Scanner> scan_point)
        :   type(type), scan_point(std::move(scan_point))
        {}

        Token(TokenType type)
        :   type(type), scan_point(std::move(std::make_shared<Scanner>()))
        {}

        Token(const Token& token)
        :   type(token.type), scan_point(token.scan_point)
        {}

        std::string getTokenString(const std::string& code) {
            DEBUG_LOG("Scanner.getTokenString visited");
            // std::cout<<"getTokenString():"<<scan_point->current<<"\t"<<scan_point->start<<"\n";
            // if(scan_point->current<=scan_point->start || scan_point->current - scan_point->start <= 2) 
                // return std::string();
            return code.substr(scan_point->start, scan_point->current-scan_point->start);
        }
    };

    typedef struct Token Token;

    

    class Lexer {
        std::string item;
        std::vector<std::shared_ptr<Token>> tokens;
        Scanner scanner;

        // static std::shared_ptr<Lexer> lexer_obj;

        // Lexer() = private;
        // ~Lexer() = private;
        

        void skipNonCode() {
            DEBUG_LOG("skipNonCode visited");
            while(true) {
                char c = peek();

                switch(c) {
                    case ' ':
                        scanner.current++;
                        scanner.col++;
                        break;
                    case '\t':
                        scanner.current++;
                        scanner.col+=TAB_SPACE_COLS;
                        break;
                    case '\r':
                        advance();
                        break;

                    case '\n':
                        scanner.current++;
                        scanner.col = 0;
                        scanner.line_num++;
                        break;

                    // Taking care of comments
                    case '/':
                        if(peekNext() == '/') {
                            while(peek() != '\n' && source_code[scanner.current] != '\0') advance();
                        } else {
                            return;
                        }
                        break;

                    default: return;
                }
            }

        }


        void incrementLine() {
            scanner.line_num++;
            scanner.col = 0;
        }

        char curr() {
            DEBUG_LOG("curr() visited");
            return source_code[scanner.current-1];
        }

        char peek() {
            DEBUG_LOG("peek() visited");
            if(scanner.current >= source_code.length()) return '\0';
            return source_code[scanner.current];
        }

        char peekNext() {
            DEBUG_LOG("peekNext() visited");
            if(scanner.current >= source_code.length()-1) return '\0';
            return source_code[scanner.current+1];
        }

        char advance() {
            DEBUG_LOG("advance visited");
            scanner.current++;
            scanner.col++;
            return source_code[scanner.current - 1];
        }

        bool isEnd() {
            return curr() == '\0';
        }

        bool match(char c) {
            DEBUG_LOG("match visited");
            if(peek() == c) return advance() == c;
            return false;
        }

        bool ishex(char c) {
            DEBUG_LOG("ishex visited");
            return isdigit(c) || (c>='a' && c<='f') || (c>='A' && c<='F');
        }

        bool isoctal(char c) {
            DEBUG_LOG("isoctal visited");
            return c >= '0' && c <= '9';
        }

        void number() {
            DEBUG_LOG("number visited");
            char c = peek();

            // checking if hex
            if(curr() == '0' && (c == 'x' || c == 'X')) {
                advance();
                while(ishex(peek()) && !isEnd()) advance();
                if(isEnd()) logger->logMessage(LogLevel::ERROR, "Unterminated hex literal");
                tokens.emplace_back(std::make_shared<Token>(TokenType::HEX_LITERAL, std::make_shared<Scanner>(scanner)));
                return;
            }

            // checking if octal
            if(curr() == '0' && (c == 'o' || c=='O')) {
                advance();
                while(isoctal(peek()) && !isEnd()) advance();
                if(isEnd()) logger->logMessage(LogLevel::ERROR, "Unterminated octal literal");
                tokens.emplace_back(std::make_shared<Token>(TokenType::OCTAL_LITERAL, std::make_shared<Scanner>(scanner)));
                return;
            }

            // differentiaing float and integer. 
            bool is_int = true;
            while(isdigit(peek()) && !isEnd()) advance();
            if(isEnd()) logger->logMessage(LogLevel::ERROR, "Unterminated integer/float literal");

            if(match('.')) {
                is_int = false;
            }
                
            while(isdigit(peek()) && !isEnd()) advance();
            if(isEnd()) logger->logMessage(LogLevel::ERROR, "Unterminated integer/float literal");
                
            if(match('e') || match('E')) {
                is_int = false;
                match('+') || match('-');
            }

            while(isdigit(peek()) && !isEnd()) advance();
            if(isEnd()) logger->logMessage(LogLevel::ERROR, "Unterminated integer/float literal");

            if(is_int)  tokens.emplace_back(std::make_shared<Token>(TokenType::INT_LITERAL, std::make_shared<Scanner>(scanner)));
            else        tokens.emplace_back(std::make_shared<Token>(TokenType::FLOAT_LITERAL, std::make_shared<Scanner>(scanner)));

            return;
            
        }

        void string() {
            while(peek() != '"' && curr() != '\0') {
                if(peek() == '\n') incrementLine();
                advance();
            }

            if(curr() == '\0') {
                logger->logMessage(LogLevel::ERROR, "[line: %u, col: %u] Unterminated string", scanner.line_num, scanner.col);
            }

            // the closing quote
            advance();
            tokens.emplace_back(std::make_shared<Token>(TokenType::STRING_LITERAL, std::make_unique<Scanner>(scanner)));
        }

        void identifier() {
            DEBUG_LOG("identifier visited");
            while((isdigit(peek()) || isalpha(peek()) || peek() == '_') && !isEnd()) advance();
            if(isEnd()) logger->logMessage(LogLevel::ERROR, "Unterminated identifier");
            return recognizeIdentifierType();
        }


        void recognizeIdentifierType() {
            DEBUG_LOG("recognizeIdentifierType visited");
            auto str = source_code.substr(scanner.start, scanner.current-scanner.start);

            #define APPEND_IF_MATCH(type, type_str) \
                if(str == type_str) {\
                    tokens.emplace_back(std::make_shared<Token>(type, std::make_shared<Scanner>(scanner))); \
                    return; \
                }


            switch(source_code[scanner.start]) {
                case 'a': APPEND_IF_MATCH(TokenType::K_AND, std::string("and"));
                case 'b': APPEND_IF_MATCH(TokenType::K_BOOL, std::string("bool"));
                case 'e': APPEND_IF_MATCH(TokenType::K_ELSE, std::string("else"));
                case 'f': 
                        APPEND_IF_MATCH(TokenType::K_FOR, std::string("for"));
                        APPEND_IF_MATCH(TokenType::K_FUNC, std::string("func"));
                        APPEND_IF_MATCH(TokenType::K_FALSE, std::string("false"));
                        APPEND_IF_MATCH(TokenType::K_FLOAT32, std::string("float32"));
                        APPEND_IF_MATCH(TokenType::K_FLOAT64, std::string("float64"));
                case 'i':
                        APPEND_IF_MATCH(TokenType::K_IF, std::string("if"));
                        APPEND_IF_MATCH(TokenType::K_IMPL, std::string("impl"));
                        APPEND_IF_MATCH(TokenType::K_INT8, std::string("int8"));
                        APPEND_IF_MATCH(TokenType::K_INT16, std::string("int16"));
                        APPEND_IF_MATCH(TokenType::K_INT32, std::string("int32"));
                        APPEND_IF_MATCH(TokenType::K_INT64, std::string("int64"));

                case 'n':
                        APPEND_IF_MATCH(TokenType::K_NULL, std::string("null"));
                case 'o':
                        APPEND_IF_MATCH(TokenType::K_OR, std::string("or"));
                case 'r':
                        APPEND_IF_MATCH(TokenType::K_RETURN, std::string("return"));
                case 's':
                        // APPEND_IF_MATCH(TokenType::K_SELF, std::string("self"));
                        APPEND_IF_MATCH(TokenType::K_STRUCT, std::string("struct"));
                case 't':
                        APPEND_IF_MATCH(TokenType::K_TRUE, std::string("true"));
                case 'u':
                        APPEND_IF_MATCH(TokenType::K_UINT8, std::string("uint8"));
                        APPEND_IF_MATCH(TokenType::K_UINT16, std::string("uint16"));
                        APPEND_IF_MATCH(TokenType::K_UINT32, std::string("uint32"));
                        APPEND_IF_MATCH(TokenType::K_UINT64, std::string("uint64"));
                case 'v':
                        APPEND_IF_MATCH(TokenType::K_VAR, std::string("var"));
                        
            }

            if(
                tokens.back()->type == TokenType::K_STRUCT 
                || tokens.back()->type == TokenType::K_IMPL
                || tokens.back()->type == TokenType::IDENTIFIER) {
                tokens.emplace_back(std::make_shared<Token>(TokenType::OBJECT_TYPE, std::make_shared<Scanner>(scanner)));
                return;
            }
            tokens.emplace_back(std::make_shared<Token>(TokenType::IDENTIFIER, std::make_shared<Scanner>(scanner)));

            #undef APPEND_IF_MATCH
        }

        void scanToken() {
            DEBUG_LOG("scanToken visited");
            skipNonCode();
            scanner.start = scanner.current;
            if(scanner.current > 0 && curr() == '\0')  {
                tokens.emplace_back(std::make_shared<Token>(TokenType::FILE_EOF, std::make_shared<Scanner>(scanner)));
                return;
            }

            char c = advance();

            if(isdigit(c))                 return number();
            if(isalpha(c) || c== '_')      return identifier();

            #define APPEND_TYPE(type) \
                { tokens.emplace_back(std::make_shared<Token>(type, std::make_shared<Scanner>(scanner))); \
                return;  }

            switch(c) {
                case '(':   APPEND_TYPE(TokenType::BRACKET_OPEN);
                case ')':   APPEND_TYPE(TokenType::BRACKET_CLOSE);
                case '{':   APPEND_TYPE(TokenType::CURLY_OPEN);
                case '}':   APPEND_TYPE(TokenType::CURLY_CLOSE);
                case '[':   APPEND_TYPE(TokenType::SQUARE_OPEN);
                case ']':   APPEND_TYPE(TokenType::SQUARE_CLOSE);
                case '=':   if(match('=')){APPEND_TYPE(TokenType::EQUAL_EQUAL);}else{APPEND_TYPE(TokenType::EQUAL);}
                case '*':   if(match('=')){APPEND_TYPE(TokenType::ASTERISK_EQUAL);}else{APPEND_TYPE(TokenType::ASTERISK);}
                case '/':   if(match('=')){APPEND_TYPE(TokenType::SLASH_EQUAL);}else{APPEND_TYPE(TokenType::SLASH);}
                case '%':   if(match('=')){APPEND_TYPE(TokenType::MOD_EQUAL);}else{APPEND_TYPE(TokenType::MOD);}
                case '+':   if(match('=')){APPEND_TYPE(TokenType::PLUS_EQUAL);}else{APPEND_TYPE(TokenType::PLUS);}
                case '-':   if(match('=')){APPEND_TYPE(TokenType::MINUS_EQUAL);}else{APPEND_TYPE(TokenType::MINUS);}
                case '|':   if(match('=')){APPEND_TYPE(TokenType::BITWISE_OR_EQUAL);}else{APPEND_TYPE(TokenType::BITWISE_OR);}
                case '&':   if(match('=')){APPEND_TYPE(TokenType::BITWISE_AND_EQUAL);}else{APPEND_TYPE(TokenType::BITWISE_AND);}
                case '^':   if(match('=')){APPEND_TYPE(TokenType::BITWISE_XOR_EQUAL);}else{APPEND_TYPE(TokenType::BITWISE_XOR);}
                case '$':   APPEND_TYPE(TokenType::DOLLAR);
                case '#':   APPEND_TYPE(TokenType::HASH);
                case ';':   APPEND_TYPE(TokenType::SEMICOLON);
                case ',':   APPEND_TYPE(TokenType::COMMA);
                case '"':   return string();
                return;
            }

            #undef APPEND_TYPE
        }



        public:
            Lexer(const char* src_code): scanner(Scanner()) {
                tokens = std::vector<std::shared_ptr<Token>>();
                source_code = std::string(src_code);

                scanner = Scanner();
                scanner.current = 0;
                scanner.start = 0;
                scanner.line_num = 1;
                scanner.col = 0;

                if(source_code.length() == 0) {
                    logger->logMessage(LogLevel::ERROR, "File empty. Didn't find source code.");
                }
            }

            std::vector<std::shared_ptr<Token>> tokenize() {
                DEBUG_LOG("tokenize visited");

                while(scanner.current < source_code.length())
                    scanToken();

                tokens.emplace_back(std::make_shared<Token>(TokenType::FILE_EOF, std::make_shared<Scanner>(scanner)));
                return std::move(tokens);
                
            }

            void printTokens(std::vector<std::shared_ptr<Token>> tokens) {
                DEBUG_LOG("printTokens visited");
                int i=0;
                for(auto &token: tokens) {
                    std::cout<<i<<"\t"<<token->getTokenString(source_code)<<"\t"<<enumStringVal(token->type)<<std::endl;
                    i++;
                }
            }
    };

}


#endif // __BAASHA_LEXER_CPP