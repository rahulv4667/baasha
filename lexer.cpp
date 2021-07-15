#ifndef __BAASHA_LEXER_CPP
#define __BAASHA_LEXER_CPP
#include "globals.hpp"

namespace Baasha {

    enum class TokenType {

        TOKEN_ERROR,

        // keywords
        K_VAR, K_CLASS, K_FUNC, K_OR, K_AND, K_IF, K_ELSE, K_FOR,
        K_RETURN,

        // datatypes
        K_INT32, K_INT64, K_INT16, K_INT8,
        K_UINT32, K_UINT64, K_UINT16, K_UINT8,
        K_FLOAT32, K_FLOAT64, K_BOOL, OBJECT,
        K_NULL,

        K_TRUE, K_FALSE,

        IDENTIFIER,
        
        INT_LITERAL, FLOAT_LITERAL, HEX_LITERAL, OCTAL_LITERAL,
        STRING_LITERAL, SEMICOLON,

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
        else if(kword == "class")               return TokenType::K_CLASS;
        else if(kword == "and")                 return TokenType::K_AND;
        else if(kword == "or")                  return TokenType::K_OR;
        else if(kword == "if")                  return TokenType::K_IF;
        else if(kword == "else")                return TokenType::K_ELSE;
        else if(kword == "for")                 return TokenType::K_FOR;
        else                                    return TokenType::TOKEN_ERROR;
    }

    std::string enumStringVal(TokenType type) {
        switch(type) {
            case TokenType::ASTERISK:                   return "ASTERISK";
            case TokenType::ASTERISK_EQUAL:             return "ASTERISK_EQUAL";
            case TokenType::BANG:                       return "BANG";
            case TokenType::BANG_EQUAL:                 return "BANG_EQUAL";
            case TokenType::BITWISE_AND:                return "BITWISE_AND";
            case TokenType::BITWISE_AND_EQUAL:          return "BITWISE_AND_EQUAL";
            case TokenType::BITWISE_OR:                 return "BITWISE_OR";
            case TokenType::BITWISE_OR_EQUAL:           return "BITWISE_OR_EQUAL";
            case TokenType::BITWISE_XOR:                return "BITWISE_XOR";
            case TokenType::BITWISE_XOR_EQUAL:          return "BITWISE_XOR_EQUAL";
            case TokenType::BRACKET_CLOSE:              return "BRACKET_CLOSE";
            case TokenType::BRACKET_OPEN:               return "BRACKET_OPEN";
            case TokenType::CURLY_CLOSE:                return "CURLY_CLOSE";
            case TokenType::CURLY_OPEN:                 return "CURLY_OPEN";
            case TokenType::DOLLAR:                     return "DOLLAR";
            case TokenType::EQUAL:                      return "EQUAL";
            case TokenType::EQUAL_EQUAL:                return "EQUAL_EQUAL";
            case TokenType::FILE_EOF:                   return "FILE_EOF";
            case TokenType::FLOAT_LITERAL:              return "FLOAT_LITERAL";
            case TokenType::GREAT_EQUAL:                return "GREAT_EQUAL";
            case TokenType::GREAT_THAN:                 return "GREAT_THAN";
            case TokenType::HASH:                       return "HASH";
            case TokenType::HEX_LITERAL:                return "HEX_LITERAL";
            case TokenType::IDENTIFIER:                 return "IDENTIFIER";
            case TokenType::INT_LITERAL:                return "INT_LITERAL";
            case TokenType::K_AND:                      return "K_AND";
            case TokenType::K_BOOL:                     return "K_BOOL";
            case TokenType::K_CLASS:                    return "K_CLASS";
            case TokenType::K_ELSE:                     return "K_ELSE";
            case TokenType::K_FALSE:                    return "K_FALSE";
            case TokenType::K_FLOAT32:                  return "K_FLOAT32";
            case TokenType::K_FLOAT64:                  return "K_FLOAT64";
            case TokenType::K_FOR:                      return "K_FOR";
            default:                                    return "SOMETHING";
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
                case 'c': APPEND_IF_MATCH(TokenType::K_CLASS, std::string("class"));
                case 'e': APPEND_IF_MATCH(TokenType::K_ELSE, std::string("else"));
                case 'f': 
                        APPEND_IF_MATCH(TokenType::K_FOR, std::string("for"));
                        APPEND_IF_MATCH(TokenType::K_FUNC, std::string("func"));
                        APPEND_IF_MATCH(TokenType::K_FALSE, std::string("false"));
                        APPEND_IF_MATCH(TokenType::K_FLOAT32, std::string("float32"));
                        APPEND_IF_MATCH(TokenType::K_FLOAT64, std::string("float64"));
                case 'i':
                        APPEND_IF_MATCH(TokenType::K_IF, std::string("if"));
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

            tokens.emplace_back(std::make_shared<Token>(TokenType::IDENTIFIER, std::make_shared<Scanner>(scanner)));

            #undef CHECK_AND_APPEND
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