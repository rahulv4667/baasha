#include "globals.hpp"
#include "lexer.cpp"
#include "ast.cpp"

namespace Baasha {

    class Parser {

        std::vector<std::shared_ptr<Token>> tokens;
        Scanner scanner;

        bool match(TokenType type) {
            if(scanner.current >= tokens.size()) return false;
            if(peek()->type == type) return advance()->type == type;
            return false;
        }

        bool isEnd() {
            return scanner.current >= tokens.size() || tokens[scanner.current]->type == TokenType::FILE_EOF;
        }

        const std::shared_ptr<Token>& advance() {
            if(isEnd()) return nullptr;
            scanner.current++;
            return tokens[scanner.current - 1];
        }

        const std::shared_ptr<Token>& peek() {
            if(isEnd()) return nullptr;
            return tokens[scanner.current];
        }

        const std::shared_ptr<Token>& peekNext() {
            if(isEnd()  || tokens[scanner.current+1]->type == TokenType::FILE_EOF) return nullptr;
            return tokens[scanner.current + 1];
        }

        const std::shared_ptr<Token>& curr() {
            if(scanner.current >= tokens.size()) return nullptr;
            return tokens[scanner.current - 1];
        }

        void declaration() {
            
        }

        public:

            Parser(const std::vector<std::shared_ptr<Token>>& tokens) : tokens(tokens)
            {
                scanner.current = 0;
                scanner.start = 0;
                scanner.col = 0;
                scanner.line_num = 1;
            }

            void parse() {

            }
    };
}