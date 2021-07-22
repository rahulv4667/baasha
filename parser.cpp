#ifndef __BAASHA_PARSER_CPP
#define __BAASHA_PARSER_CPP

#include "globals.hpp"
#include "lexer.cpp"
#include "ast.cpp"

namespace Baasha {

    class Parser;

    enum class Precedence {
        PREC_NONE,
        PREC_ASSIGNMENT,    // =
        PREC_OR,            // or
        PREC_AND,           // and
        PREC_EQUALITY,      // == !=
        PREC_COMPARISON,    // <, >, <=, >=
        PREC_TERM,          // + -
        PREC_FACTOR,        // * /
        PREC_UNARY,         // ! -
        PREC_CALL,          // . ()
        PREC_PRIMARY
    };

    Precedence& nextHigher(Precedence& prec) {
        Precedence ret_prec;
        switch(prec) {
            case Precedence::PREC_NONE:             ret_prec = Precedence::PREC_ASSIGNMENT; break;
            case Precedence::PREC_ASSIGNMENT:       ret_prec = Precedence::PREC_OR; break;
            case Precedence::PREC_OR:               ret_prec = Precedence::PREC_AND; break;
            case Precedence::PREC_AND:              ret_prec = Precedence::PREC_EQUALITY; break;
            case Precedence::PREC_EQUALITY:         ret_prec = Precedence::PREC_COMPARISON; break;
            case Precedence::PREC_COMPARISON:       ret_prec = Precedence::PREC_TERM; break;
            case Precedence::PREC_TERM:             ret_prec = Precedence::PREC_FACTOR; break;
            case Precedence::PREC_FACTOR:           ret_prec = Precedence::PREC_UNARY; break;
            case Precedence::PREC_UNARY:            ret_prec = Precedence::PREC_CALL; break;
            case Precedence::PREC_CALL:
            case Precedence::PREC_PRIMARY:          ret_prec = Precedence::PREC_PRIMARY; break;
        }
        return ret_prec;
    }

    // typedef std::function<std::unique_ptr<Expr>()> PrefixFn;
    // typedef std::function<std::unique_ptr<Expr>(std::unique_ptr<Expr>)> InfixFn;
    typedef std::unique_ptr<Expr> (Parser::*PrefixFn)(bool);
    typedef std::unique_ptr<Expr> (Parser::*InfixFn)(std::unique_ptr<Expr>);
    
    struct ParseRule {
        PrefixFn prefixFunc;
        InfixFn infixFunc;
        Precedence precedence;

        ParseRule(PrefixFn prefixFunc,
            InfixFn infixFunc,
            Precedence precedence
        )   : prefixFunc(prefixFunc), infixFunc(infixFunc), precedence(precedence)
        {}

        ParseRule(const ParseRule& rule)
        : prefixFunc(rule.prefixFunc), infixFunc(rule.infixFunc), precedence(rule.precedence)
        {}

        ParseRule& operator=(const ParseRule& rule) {
            this->infixFunc = rule.infixFunc;
            this->precedence = rule.precedence;
            this->prefixFunc = rule.prefixFunc;
        }

        // ParseRule(ParseRule&& rule)
        // : prefixFunc(rule.prefixFunc), infixFunc(rule.infixFunc), precedence(rule.precedence)
        // {}
    };
    typedef struct ParseRule ParseRule;

    
    

    class Parser {

        public:

            Parser(const std::vector<std::shared_ptr<Token>>& tokens) 
            :   tokens(tokens),
                statements(std::vector<std::unique_ptr<Stmt>>()),
                scanner(Scanner())
            {
                initializeParseRules();
            }

            ~Parser() {
                destroyParseRules();
            }

            std::vector<std::unique_ptr<Stmt>> parse() {
                // std::cout<<"Tokens size:"<<tokens.size()<<"\n";
                while(!isEnd() && peek()->type != TokenType::FILE_EOF) {
                    auto stmt = statement();
                    if(stmt != nullptr)
                        statements.emplace_back(std::move(stmt));
                }
                return std::move(statements);
            }
        
        private:

        std::vector<std::shared_ptr<Token>> tokens;
        std::vector<std::unique_ptr<Stmt>> statements;
        std::map<TokenType, ParseRule*> rules;
        Scanner scanner;

        void initializeParseRules() {
            rules[TokenType::BRACKET_OPEN]      = new ParseRule(&Parser::grouping,  &Parser::call,  Precedence::PREC_NONE);
            rules[TokenType::BRACKET_CLOSE]     = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::CURLY_OPEN]        = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::CURLY_CLOSE]       = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::COMMA]             = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            // rules[TokenType::DOT]               = ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::MINUS]             = new ParseRule(&Parser::unary,    &Parser::binary, Precedence::PREC_TERM);
            rules[TokenType::MINUS_EQUAL]       = new ParseRule(NULL,            NULL,       Precedence::PREC_ASSIGNMENT);
            rules[TokenType::PLUS]              = new ParseRule(NULL,    &Parser::binary,     Precedence::PREC_TERM);
            rules[TokenType::PLUS_EQUAL]        = new ParseRule(NULL,            NULL,       Precedence::PREC_ASSIGNMENT);
            rules[TokenType::SEMICOLON]         = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::SLASH]             = new ParseRule(NULL,    &Parser::binary,     Precedence::PREC_FACTOR);
            rules[TokenType::SLASH_EQUAL]       = new ParseRule(NULL,    &Parser::binary,     Precedence::PREC_ASSIGNMENT);
            rules[TokenType::ASTERISK]          = new ParseRule(NULL,    &Parser::binary,     Precedence::PREC_FACTOR);
            rules[TokenType::ASTERISK_EQUAL]    = new ParseRule(NULL,    &Parser::binary,     Precedence::PREC_ASSIGNMENT);
            rules[TokenType::BANG]              = new ParseRule(&Parser::unary,           NULL,       Precedence::PREC_NONE);
            rules[TokenType::BANG_EQUAL]        = new ParseRule(NULL,    &Parser::binary,     Precedence::PREC_EQUALITY);
            rules[TokenType::EQUAL]             = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::EQUAL_EQUAL]       = new ParseRule(NULL,    &Parser::binary,     Precedence::PREC_EQUALITY);
            rules[TokenType::GREAT_THAN]        = new ParseRule(NULL,    &Parser::binary,     Precedence::PREC_COMPARISON);
            rules[TokenType::GREAT_EQUAL]       = new ParseRule(NULL,    &Parser::binary,     Precedence::PREC_COMPARISON);
            rules[TokenType::LESS_THAN]         = new ParseRule(NULL,    &Parser::binary,     Precedence::PREC_COMPARISON);
            rules[TokenType::LESS_EQUAL]        = new ParseRule(NULL,    &Parser::binary,     Precedence::PREC_COMPARISON);
            rules[TokenType::IDENTIFIER]        = new ParseRule(&Parser::variable,        NULL,       Precedence::PREC_NONE);
            rules[TokenType::STRING_LITERAL]    = new ParseRule(&Parser::primary/*string*/,          NULL,       Precedence::PREC_NONE);
            rules[TokenType::INT_LITERAL]       = new ParseRule(&Parser::primary/*number*/,          NULL,       Precedence::PREC_NONE);
            rules[TokenType::HEX_LITERAL]       = new ParseRule(&Parser::primary/*number*/,          NULL,       Precedence::PREC_NONE);
            rules[TokenType::OCTAL_LITERAL]     = new ParseRule(&Parser::primary/*number*/,          NULL,       Precedence::PREC_NONE);
            rules[TokenType::FLOAT_LITERAL]     = new ParseRule(&Parser::primary/*number*/,          NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_AND]             = new ParseRule(NULL,     &Parser::binary/*_and*/,       Precedence::PREC_AND);
            rules[TokenType::K_OR]              = new ParseRule(NULL,     &Parser::binary/*_or*/,        Precedence::PREC_OR);
            rules[TokenType::K_FALSE]           = new ParseRule(&Parser::primary/*literal*/,         NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_TRUE]            = new ParseRule(&Parser::primary/*literal*/,         NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_ELSE]            = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_IF]              = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_BOOL]            = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_INT8]            = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_INT16]           = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_INT32]           = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_INT64]           = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_UINT8]           = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_UINT16]          = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_UINT32]          = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_UINT64]          = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_VAR]             = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_FUNC]            = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_FLOAT32]         = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_FLOAT64]         = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_STRUCT]          = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_IMPL]            = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_FOR]             = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_NULL]            = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            rules[TokenType::K_RETURN]          = new ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
        }


        void destroyParseRules() {
            delete rules[TokenType::BRACKET_OPEN];
            delete rules[TokenType::BRACKET_CLOSE];
            delete rules[TokenType::CURLY_OPEN];
            delete rules[TokenType::CURLY_CLOSE];
            delete rules[TokenType::COMMA];
            // rules[TokenType::DOT]               = ParseRule(NULL,            NULL,       Precedence::PREC_NONE);
            delete rules[TokenType::MINUS];
            delete rules[TokenType::MINUS_EQUAL];
            delete rules[TokenType::PLUS];
            delete rules[TokenType::PLUS_EQUAL];
            delete rules[TokenType::SEMICOLON];
            delete rules[TokenType::SLASH];
            delete rules[TokenType::SLASH_EQUAL];
            delete rules[TokenType::ASTERISK];
            delete rules[TokenType::ASTERISK_EQUAL];
            delete rules[TokenType::BANG];
            delete rules[TokenType::BANG_EQUAL];
            delete rules[TokenType::EQUAL];
            delete rules[TokenType::EQUAL_EQUAL];
            delete rules[TokenType::GREAT_THAN];
            delete rules[TokenType::GREAT_EQUAL];
            delete rules[TokenType::LESS_THAN];
            delete rules[TokenType::LESS_EQUAL];
            delete rules[TokenType::IDENTIFIER];
            delete rules[TokenType::STRING_LITERAL];
            delete rules[TokenType::INT_LITERAL];
            delete rules[TokenType::HEX_LITERAL];
            delete rules[TokenType::OCTAL_LITERAL];
            delete rules[TokenType::FLOAT_LITERAL];
            delete rules[TokenType::K_AND];
            delete rules[TokenType::K_OR];
            delete rules[TokenType::K_FALSE];
            delete rules[TokenType::K_TRUE];
            delete rules[TokenType::K_ELSE];
            delete rules[TokenType::K_IF];
            delete rules[TokenType::K_BOOL];
            delete rules[TokenType::K_INT8];
            delete rules[TokenType::K_INT16];
            delete rules[TokenType::K_INT32];
            delete rules[TokenType::K_INT64];
            delete rules[TokenType::K_UINT8];
            delete rules[TokenType::K_UINT16];
            delete rules[TokenType::K_UINT32];
            delete rules[TokenType::K_UINT64];
            delete rules[TokenType::K_VAR];
            delete rules[TokenType::K_FUNC];
            delete rules[TokenType::K_FLOAT32];
            delete rules[TokenType::K_FLOAT64];
            delete rules[TokenType::K_STRUCT];
            delete rules[TokenType::K_IMPL];
            delete rules[TokenType::K_FOR];
            delete rules[TokenType::K_NULL];
            delete rules[TokenType::K_RETURN];
        }



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
        }

        const std::shared_ptr<Token>& advance() {
            if(!isEnd()) scanner.current++;
            return tokens[scanner.current - 1];
        }

        const std::shared_ptr<Token>& peek() {
            return tokens[scanner.current];
        }

        const std::shared_ptr<Token>& peekNext() {
            if(isEnd()  || tokens[scanner.current+1]->type == TokenType::FILE_EOF) {
                logger->logMessage(LogLevel::CRASH, "Reached EOF before completing parsing. Probable incomplete expression.");
                return nullptr;
            }
            return tokens[scanner.current + 1];
        }

        const std::shared_ptr<Token>& curr() {
            if(isEnd()) {
                
                logger->logMessage(LogLevel::CRASH, "Reached EOF before completing parsing. Probable incomplete expression.");
                return nullptr;
            }
            return tokens[scanner.current - 1];
        }

        std::unique_ptr<Token> consume(TokenType type, std::string message, bool show_error = true) {
            if(match(type)) 
                return std::make_unique<Token>(*curr());
            
            if(show_error) {
                std::stringstream ss;
                ss<<"[Line: "<<tokens[scanner.current]->scan_point->line_num<<", ";
                ss<<"Col: "<<tokens[scanner.current]->scan_point->col<<"] "<<message;
                logger->logMessage(LogLevel::ERROR, ss.str());
            }
            return nullptr;
        }

        std::unique_ptr<Token> consume(std::vector<TokenType> types, std::string message, bool show_error = true) {

            if(match(types)) {
                return std::move(std::make_unique<Token>(*curr()));
            }
            
            if(message.length() >= 1 && show_error)  {  // sometimes, we just consume for optional parts. So, we can skip error printing.
                std::stringstream ss;
                ss<<"[Line: "<<tokens[scanner.current]->scan_point->line_num<<", ";
                ss<<"Col: "<<tokens[scanner.current]->scan_point->col<<"] "<<message;
                logger->logMessage(LogLevel::ERROR, ss.str());
            }
            return nullptr;
        }

        bool check(TokenType type) {
            if(isEnd()) return false;
            return peek()->type == type;
        }
        

        std::unique_ptr<Stmt> statement() {
            std::cout<<"Statement: "<<enumStringVal(peek()->type)<<"\n";
            if(match(TokenType::K_VAR))                 return varStmt();
            else if(match(TokenType::K_FUNC))           return funcStmt();
            else if(match(TokenType::K_RETURN))         return returnStmt();
            else if(match(TokenType::K_STRUCT))         return structStmt();
            else if(match(TokenType::K_IMPL))           return implStmt();
            else if(match(TokenType::SEMICOLON)) {
                return nullptr;
            }
            else {
                return std::make_unique<Stmt::ExprStmt>(std::move(expression()));
            }

            // if not any of the above cases. Eg: SEMICOLON
            // advance();
            scanner.current++;
            return nullptr;       
        }


        std::unique_ptr<Stmt::StructStmt> structStmt() {
            std::unique_ptr<Token> name;
            std::vector<std::unique_ptr<Stmt::VarStmt>> members;

            name = consume(TokenType::OBJECT_TYPE, "Expected identifier after 'struct' keyword", true);
            
            consume(TokenType::CURLY_OPEN, "Expected '{' after identifier for struct", true);

            // Scope prev_scope = scope;
            // scope = Scope::STRUCT;

            while(!match(TokenType::CURLY_CLOSE)) {

                if(match(TokenType::K_VAR)) {
                    std::unique_ptr<Stmt::VarStmt> var = varStmt(true);
                    if(var != nullptr)
                        members.emplace_back(std::move(var));
                }
                else if(match(TokenType::SEMICOLON)) continue; // required so that we go past semicolon

            }

            // scope = prev_scope;
            return std::make_unique<Stmt::StructStmt>(std::move(name), std::move(members));
        }


        std::unique_ptr<Stmt::ImplStmt> implStmt() {
            std::unique_ptr<Token> name;
            std::vector<std::unique_ptr<Stmt::FunctionStmt>> funcs;

            name = consume(TokenType::OBJECT_TYPE, "Expected struct name after 'impl' keyword", true);
            consume(TokenType::CURLY_OPEN, "Expected '{' after struct name for impl", true);

            std::string struct_name = name->getTokenString(source_code);
            // Scope prev_scope = scope;
            // scope = Scope::IMPL;

            // if(user_defined_types.count(struct_name) == 0) {
            //     llvm::StructType* stype = llvm::StructType::get(*the_context);
            //     stype->setName(struct_name);
            //     user_defined_types[struct_name] =  stype;
            // }
 
            while(!match(TokenType::CURLY_CLOSE)) {
                if(match(TokenType::K_FUNC)) {
                    
                    std::unique_ptr<Stmt::FunctionStmt> func = funcStmt(true);
                    if(func != nullptr) { 
                        func->prototype->struct_or_trait_name = struct_name;
                        funcs.emplace_back(std::move(func));    
                    }
                    
                }
                // else if(match(TokenType))
            }

            // scope = prev_scope;
            return std::make_unique<Stmt::ImplStmt>(std::move(name), std::move(funcs));
        }


        std::unique_ptr<Stmt::ReturnStmt> returnStmt() {
            std::vector<std::unique_ptr<Expr>> retExprs;

            while(!match(TokenType::SEMICOLON)) {
                retExprs.emplace_back(std::move(expression()));
                std::cout<<"Returning "<<retExprs.back()->typeName()<<"\n";

                if(match(TokenType::COMMA)) continue;
                else if(match(TokenType::SEMICOLON)) break;
                else {
                    logger->logMessage(LogLevel::ERROR, "There should be either ',' or ';' after return expression");
                    return nullptr;
                }

            }

            return std::make_unique<Stmt::ReturnStmt>(std::move(retExprs));
        }

        std::unique_ptr<Stmt::FunctionStmt> funcStmt(bool is_inside_impl=false) {

            // TODO: Right now, it works only for primitive types. find a way to make match for custom types also.
            // TODO: Also handle default values for function parameters.

            std::unique_ptr<Token> name = consume(TokenType::IDENTIFIER, "Expected function name after 'func'", true);
            consume(TokenType::BRACKET_OPEN, "Expected '(' after function name", true);

            std::vector<TokenType> datatypes = {
                TokenType::K_INT32, TokenType::K_INT64, TokenType::K_INT16, TokenType::K_INT8,
                TokenType::K_UINT32, TokenType::K_UINT64, TokenType::K_UINT16, TokenType::K_UINT8,
                TokenType::K_FLOAT32, TokenType::K_FLOAT64, TokenType::K_BOOL
            };

            // reading parameters list
            std::map<std::string, llvm::Type*> params;

            // if(!struct_or_trait_name.empty()) {
            //     llvm::StructType* stype = (llvm::StructType*)user_defined_types[struct_or_trait_name];
            //     llvm::Type* dtype = llvm::PointerType::get(stype, 0);
            //     params["self"] = dtype;
            // }

            while(!match(TokenType::BRACKET_CLOSE)) {
                // read parameters
                
                while(match(datatypes)) {
                    auto tok = curr();
                    llvm::Type* dtype = getLLVMTypeRaw(tok->type);

                    while(match(TokenType::IDENTIFIER)) {
                        auto idntfr = curr();
                        params[idntfr->getTokenString(source_code)] = dtype;
                        if(match(TokenType::COMMA)) continue;
                        else break;
                    }
                }
            }

            // reading return types
            std::vector<TokenType> returns;
            consume(TokenType::BRACKET_OPEN, "Expected '(' after parameter list to get return types", true);

            while(!match(TokenType::BRACKET_CLOSE)) {
                while(match(datatypes)) {
                    auto tok = curr();
                    returns.push_back(curr()->type);
                    if(match(TokenType::COMMA)) continue;
                    else break;
                }
            }

            
            if(match(TokenType::SEMICOLON)) {

                if(is_inside_impl) {
                    logger->logMessage(LogLevel::ERROR, "Implementations can't have function declarations. Only definitions allowed");
                    return nullptr;
                }

                std::unique_ptr<Stmt::PrototypeStmt> proto =
                        std::make_unique<Stmt::PrototypeStmt>(name->getTokenString(source_code), params, returns, true);
                return std::make_unique<Stmt::FunctionStmt>(std::move(proto), nullptr);
            
            } else if(match(TokenType::CURLY_OPEN)){
        
                std::unique_ptr<Stmt::PrototypeStmt> proto = 
                        std::make_unique<Stmt::PrototypeStmt>(name->getTokenString(source_code), params, returns, false);
                std::unique_ptr<Stmt::BlockStmt> body = blockStmt();
                return std::make_unique<Stmt::FunctionStmt>(std::move(proto), std::move(body));
            } else {
                logger->logMessage(LogLevel::ERROR, "Expected either ';' or '{' after function signature");
                return nullptr;
            }

        }

        std::unique_ptr<Stmt::BlockStmt> blockStmt() {


            std::vector<std::unique_ptr<Stmt>> stmts;
            while(!match(TokenType::CURLY_CLOSE)) {
                auto stmt = statement();
                if(stmt != nullptr) {
                    stmts.emplace_back(std::move(stmt));
                }
            }

            return std::make_unique<Stmt::BlockStmt>(std::move(stmts));
        }

        std::unique_ptr<Stmt::VarStmt> varStmt(bool is_inside_struct=false) {

            std::unique_ptr<Token> name = consume(TokenType::IDENTIFIER, "Expected variable name after 'var'");

            std::vector<TokenType> datatypes = {
                TokenType::K_INT32, TokenType::K_INT64, TokenType::K_INT16, TokenType::K_INT8,
                TokenType::K_UINT32, TokenType::K_UINT64, TokenType::K_UINT16, TokenType::K_UINT8,
                TokenType::K_FLOAT32, TokenType::K_FLOAT64, TokenType::K_BOOL, TokenType::OBJECT_TYPE 
            };

            
            std::unique_ptr<Token> datatype = consume(std::move(datatypes), "", false);
            if(datatype == nullptr && is_inside_struct) {
                logger->logMessage(LogLevel::ERROR, "Variable declaration in a struct requires datatype");
                return nullptr;
            }
            std::unique_ptr<Expr> initializer = nullptr;

            if(match(TokenType::EQUAL)) {
                if(is_inside_struct) {
                    logger->logMessage(LogLevel::WARNING, "Initialization in struct is illegal. It will be ignored.");
                }
                initializer = expression();
            } else if(datatype == nullptr) {
                // user has neither given datatype nor initializer
                logger->logMessage(LogLevel::ERROR, "Variable declaration needs either initializer or datatype\n");
                return nullptr;
            }

            return std::make_unique<Stmt::VarStmt>(
                (name != nullptr)?name->getTokenString(source_code):"", 
                (datatype != nullptr)?datatype->getTokenString(source_code):"", 
                std::move(initializer));
        }

        std::unique_ptr<Expr> parsePrecedence(Precedence precedence) {
            // auto tok = advance();
            // std::cout<<"Precedence: "<<enumStringVal(peek()->type)<<"\n";
            PrefixFn prefix_rule = rules[peek()->type]->prefixFunc;
            if(prefix_rule == NULL) {
                // std::stringstream ss;
                // ss<<"[Line: "<<peek()->scan_point->line_num<<", ";
                // ss<<"Col: "<<peek()->scan_point->col<<"] ";
                // ss<<peek()->getTokenString(source_code)<<" ";
                // ss<<enumStringVal(peek()->type)<<"\t";
                // logger->logMessage(LogLevel::ERROR, ss.str()+" Expected expression");
                return nullptr;
            }

            auto tok = advance();

            bool can_assign = precedence <= Precedence::PREC_ASSIGNMENT;
            std::unique_ptr<Expr> expr = (this->*prefix_rule)(can_assign);

            
            while(precedence <= (rules[peek()->type]->precedence)) {
                auto tok = advance();
                InfixFn infix_rule = rules[tok->type]->infixFunc;
                if(infix_rule == NULL) return expr;
                expr = (this->*infix_rule)(std::move(expr));
            }

            if(can_assign && match(TokenType::EQUAL)) {
                logger->logMessage(LogLevel::ERROR, "Invalid assignment target");
            }

            return std::move(expr);
        }

        std::unique_ptr<Expr> expression() {
            // std::cout<<"Expression: "<<enumStringVal(peek()->type)<<"\n";
            return std::move(parsePrecedence(Precedence::PREC_ASSIGNMENT));
            // return std::move(primary());
        }

        std::unique_ptr<Expr> variable(bool can_assign) {
            std::cout<<enumStringVal(peek()->type)<<" "<<peek()->getTokenString(source_code)<<"\n";
            // auto tok = consume(TokenType::IDENTIFIER, "Expected identifier");
            auto tok = curr();
            
            if(can_assign && match(TokenType::EQUAL)) {
                // std::cout<<"Match = \n";
                return std::make_unique<Expr::AssignExpr>(std::make_unique<Token>(*tok), std::move(expression()));
            } else {
                // std::cout<<"Not match \n";
                return std::make_unique<Expr::LiteralExpr>(std::make_unique<Token>(*tok));
            }
        }

        std::unique_ptr<Expr> primary(bool can_assign) {

            // std::vector<TokenType> literal_types = {
            //     TokenType::HEX_LITERAL, TokenType::OCTAL_LITERAL, 
            //     TokenType::INT_LITERAL, TokenType::FLOAT_LITERAL,
            //     TokenType::STRING_LITERAL
            // };


            std::cout<<enumStringVal(peek()->type)<<" "<<peek()->getTokenString(source_code)<<"\n";
            // std::unique_ptr<Token> tok = consume(literal_types, "Expected an expression after '='", false);
            auto tok = curr();
            if(tok != nullptr)
                return std::make_unique<Expr::LiteralExpr>(std::make_unique<Token>(*tok));
            else return nullptr;
        }

        // std::unique_ptr<Expr> literal() {
        //     return nullptr;
        // }

        // std::unique_ptr<Expr> number() {
        //     return nullptr;
        // }

        // std::unique_ptr<Expr> variable() {
        //     return nullptr;
        // }

        // std::unique_ptr<Expr> string() {
        //     return nullptr;
        // }

        // std::unique_ptr<Expr> assignment() {
        //     return nullptr;
        // }

        std::unique_ptr<Expr> grouping(bool can_assign) {
            auto expr = expression();
            consume(TokenType::BRACKET_CLOSE, "Expected ')' after expression");
            return std::move(expr);
        }

        std::unique_ptr<Expr> call(std::unique_ptr<Expr> left) {
            return nullptr;
        }

        // std::unique_ptr<Expr> _and(std::unique_ptr<Expr> left) {
        //     std::unique_ptr<Token> oprtr = std::make_unique<Token>(*curr());
        //     ParseRule& rule = rules[oprtr->type];

        //     // first, parse all that have higher precedence
        //     std::unique_ptr<Expr> right = parsePrecedence(nextHigher(rule.precedence));

        //     // now make the binary expression
        //     return std::make_unique<Expr::BinaryExpr>(left, oprtr, right);
        // }

        // std::unique_ptr<Expr> _or(std::unique_ptr<Expr> left) {
        //     return nullptr;
        // }

        

        std::unique_ptr<Expr> unary(bool can_assign) {
            std::cout<<enumStringVal(peek()->type)<<" "<<peek()->getTokenString(source_code)<<"\n";
            std::unique_ptr<Token> oprtr = std::make_unique<Token>(*curr());
            std::cout<<enumStringVal(peek()->type)<<" "<<peek()->getTokenString(source_code)<<"\n";
            std::unique_ptr<Expr> oprnd = parsePrecedence(Precedence::PREC_UNARY);

            return std::make_unique<Expr::UnaryExpr>(std::move(oprtr), std::move(oprnd));
        }

        std::unique_ptr<Expr> binary(std::unique_ptr<Expr> left) {
            std::cout<<enumStringVal(peek()->type)<<" "<<peek()->getTokenString(source_code)<<"\n";
            std::unique_ptr<Token> oprtr = std::make_unique<Token>(*curr());
            ParseRule& rule = *rules[oprtr->type];

            std::cout<<enumStringVal(peek()->type)<<" "<<peek()->getTokenString(source_code)<<"\n";
            // first, parse all that have higher precedence
            std::unique_ptr<Expr> right = parsePrecedence(nextHigher(rule.precedence));

            // now make the binary expression
            return std::make_unique<Expr::BinaryExpr>(std::move(left), std::move(oprtr), std::move(right));
        }

        
    };
}

#endif // __BAASHA_PARSER_CPP