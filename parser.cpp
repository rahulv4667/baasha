#ifndef __BAASHA_PARSER_CPP
#define __BAASHA_PARSER_CPP

#include "globals.hpp"
#include "lexer.cpp"
#include "ast.cpp"

namespace Baasha {

    class Parser {

        std::vector<std::shared_ptr<Token>> tokens;
        std::vector<std::unique_ptr<Stmt>> statements;
        Scanner scanner;

        bool match(TokenType type) {
            if(scanner.current >= tokens.size()) return false;
            if(peek()->type == type) return advance()->type == type;
            return false;
        }

        bool match(std::vector<TokenType> types) {
            // try {
                // std::cout<<"vec match\n";
                if(scanner.current >= tokens.size()) return false;

                // auto i=0;
                for(auto type: types) {
                    // std::cout<<"Type: "<<i<<"\n";
                    // i++;
                    if(peek()->type == type) return advance()->type == type; 
                }
                return false;
            // }
            // catch(std::exception e) {
                // std::cout<<"Exception: "<<e.what()<<"\n";
                // return false;
            // }
            
        }

        bool isEnd() {
            return scanner.current >= tokens.size() || peek()->type == TokenType::FILE_EOF;
            // return scanner.current >= tokens.size() ;
            // || tokens[scanner.current]->type == TokenType::FILE_EOF;
            // || (scanner.current > 0 && tokens[scanner.current-1]->type == TokenType::FILE_EOF);
            // || (scanner.current < tokens.size()-1 && tokens[scanner.current+1]->type == TokenType::FILE_EOF);
        }

        const std::shared_ptr<Token>& advance() {
            // std::cout<<"In advance():\n";
            // if(isEnd()) {
            //     logger->logMessage(LogLevel::CRASH, "Reached EOF before completing parsing. Probable incomplete expression.");
            //     return nullptr;
            // }
            if(!isEnd()) scanner.current++;
            return tokens[scanner.current - 1];
        }

        const std::shared_ptr<Token>& peek() {
            // std::cout<<"In peek():\n";
            // if(isEnd()) {
            //     logger->logMessage(LogLevel::ERROR, "Reached EOF before completing parsing. Probable incomplete expression.");
            //     return nullptr;
            // }
            return tokens[scanner.current];
        }

        const std::shared_ptr<Token>& peekNext() {
            // std::cout<<"In peekNext():\n";
            if(isEnd()  || tokens[scanner.current+1]->type == TokenType::FILE_EOF) {
                logger->logMessage(LogLevel::ERROR, "Reached EOF before completing parsing. Probable incomplete expression.");
                return nullptr;
            }
            return tokens[scanner.current + 1];
        }

        const std::shared_ptr<Token>& curr() {
            // std::cout<<"In curr():\n";
            if(isEnd()) {
                logger->logMessage(LogLevel::CRASH, "Reached EOF before completing parsing. Probable incomplete expression.");
                return nullptr;
            }
            return tokens[scanner.current - 1];
        }

        std::unique_ptr<Token> consume(TokenType type, std::string message) {
            // std::cout<<"In consume(): expected:"<<enumStringVal(type)<<
                // ", actual:"<<enumStringVal(peek()->type)<<"\n";
            // std::cout<<"In consume():"<<scanner.current<<"\n";

            if(match(type)) 
                return std::make_unique<Token>(*curr());
            
            logger->logMessage(LogLevel::ERROR, message);
            return nullptr;
        }

        std::unique_ptr<Token> consume(std::vector<TokenType> types, std::string message) {

            if(match(types)) {
                // std::cout<<"Returning from vec consume\n";
                auto tok = std::make_unique<Token>(*curr());
                // std::cout<<"Token creating in consume\n";
                return std::move(std::make_unique<Token>(*curr()));
            }
            


            if(message.length() >= 1)        // sometimes, we just consume for optional parts. So, we can skip error printing.
                logger->logMessage(LogLevel::ERROR, message);
            return nullptr;
        }

        bool check(TokenType type) {
            if(isEnd()) return false;
            return peek()->type == type;
        }

        

        std::unique_ptr<Stmt> statement() {
            
            // std::cout<<"In statement():"<<scanner.current<<"\n";
            // std::cout<<"In statement(): "<<peek()->getTokenString(source_code)<<"\n";
            if(match(TokenType::K_VAR)) {
                auto stmt = varStmt();
                // std::cout<<"Returned from varStmt\n";
                return std::move(stmt);
            }

            // if not any of the above cases. Eg: SEMICOLON
            // advance();
            scanner.current++;
            return nullptr;       
        }

        std::unique_ptr<Stmt::VarStmt> varStmt() {
            // std::cout<<"in varStmt()\n";
            std::unique_ptr<Token> name = consume(TokenType::IDENTIFIER, "Expected variable name after 'var'");
            // std::cout<<"Came back to varStmt\n";
            std::vector<TokenType> datatypes = {
                TokenType::K_INT32, TokenType::K_INT64, TokenType::K_INT16, TokenType::K_INT8,
                TokenType::K_UINT32, TokenType::K_UINT64, TokenType::K_UINT16, TokenType::K_UINT8,
                TokenType::K_FLOAT32, TokenType::K_FLOAT64, TokenType::K_BOOL 
            };

            std::unique_ptr<Token> datatype = consume(std::move(datatypes), "");
            // std::cout<<"Got the datatypes too\n";
            std::unique_ptr<Expr> initializer = nullptr;

            if(match(TokenType::EQUAL)) {
                initializer = expression();
                // std::cout<<"Got back from expression\n";
            } else if(datatype == nullptr) {
                // user has neither given datatype nor initializer
                logger->logMessage(LogLevel::ERROR, "Variable declaration needs either initializer or datatype\n");
                return nullptr;
            }

            // std::cout<<"before name->getTokenString\n";
            // std::cout<<source_code<<"\n";
            // std::cout<<name->getTokenString(source_code)<<"\n";
            // std::cout<<((datatype != nullptr)?datatype->getTokenString(source_code):"")<<"\n";
            // std::cout<<"after name->getTokenString\n";
            

            return std::make_unique<Stmt::VarStmt>(
                (name != nullptr)?name->getTokenString(source_code):"", 
                (datatype != nullptr)?datatype->getTokenString(source_code):"", 
                std::move(initializer));
        }

        std::unique_ptr<Expr> expression() {
            // std::cout<<"In expression():"<<scanner.current<<"\n";
            return std::move(primary());
        }

        std::unique_ptr<Expr> primary() {
            // std::cout<<"In primary(): "<<scanner.current<<"\n";
            std::vector<TokenType> literal_types = {
                TokenType::HEX_LITERAL, TokenType::OCTAL_LITERAL, 
                TokenType::INT_LITERAL, TokenType::FLOAT_LITERAL,
                TokenType::STRING_LITERAL
            };
            std::unique_ptr<Token> tok = consume(literal_types, "Expected an expression after '='");
            // std::cout<<"Got back token from consume() in primary(). LiteralType is :"<<enumStringVal(tok->type)<<"\n";
            // std::cout<<tok->getTokenString(source_code)<<"\n";
            return std::make_unique<Expr::LiteralExpr>(std::move(tok));
        }

        public:

            Parser(const std::vector<std::shared_ptr<Token>>& tokens) 
            :   tokens(tokens),
                statements(std::vector<std::unique_ptr<Stmt>>()),
                scanner(Scanner())
            {}

            std::vector<std::unique_ptr<Stmt>> parse() {
                // std::cout<<"Tokens size:"<<tokens.size()<<"\n";
                while(!isEnd() && peek()->type != TokenType::FILE_EOF) {
                    auto stmt = statement();
                    if(stmt != nullptr)
                        statements.emplace_back(std::move(stmt));
                }
                return std::move(statements);
            }
    };
}

#endif // __BAASHA_PARSER_CPP