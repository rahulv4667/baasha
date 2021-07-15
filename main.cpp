#include "globals.hpp"
#include "lexer.cpp"
#include "parser.cpp"
#include "bridge.cpp"

namespace Baasha {

    const char* readFile(const char* file_path) {
        DEBUG_LOG("readFile visiting");

        FILE *file = fopen(file_path, "rb");

        if(file == NULL) {
            logger->logMessage(LogLevel::CRASH, "Couldn't open file: %s\n", file_path);
        }

        fseek(file, 0L, SEEK_END);
        size_t file_size = ftell(file);
        rewind(file);

        char* buffer = (char*)malloc(file_size + 1);

        if(buffer == NULL) {
            logger->logMessage(LogLevel::CRASH, "Not enough memory to read %s\n", file_path);
        }

        size_t bytes_read = fread(buffer, sizeof(char), file_size, file);
        if(bytes_read < file_size) {
            logger->logMessage(LogLevel::ERROR, "Couldn't read the whole file\n");
        }

        buffer[bytes_read] = '\0';

        fclose(file);
        return buffer;

    }

    void compileFile(const char* file_path) {
        DEBUG_LOG("compileFile visited");

        initializeModuleAndPassManager();

        const char* buffer = readFile(file_path);

        auto lexer = std::make_unique<Lexer>(buffer);
        free((void*)buffer);

        std::vector<std::shared_ptr<Token>> tokens = lexer->tokenize();
        lexer->printTokens(tokens);

        auto parser = std::make_unique<Parser>(tokens);
        // std::cout<<"Parser object created. About to parse\n";
        std::vector<std::unique_ptr<Stmt>> statements = parser->parse();
        // std::cout<<"Succesfully parsed\n";
        CodegenVisitor<void, llvm::Value*> visitor;
        // std::cout<<"Total number of statements:"<<statements.size()<<"\n";
        // std::cout<<statements[0]->typeName()<<"\n";
        // std::cout<<(statements[1]==nullptr)<<"\n";
        for(auto &statement: statements) {
                // std::cout<<statement->typeName()<<"\n";
                statement->accept(visitor);
        }

        

        // ir_builder->Insert(llvm::ConstantInt::get(llvm::IntegerType::get(*the_context, 64), 42, false));
        the_module->print(llvm::errs(), nullptr);
        // if(!the_module->global_empty()) {
        //     auto i = the_module->global_begin();
        //     while(i != the_module->global_end()) {
        //         i->print(llvm::outs());
        //         std::cout<<"\n";
        //         i++;
        //     }
        // } else {
        //     std::cout<<"No global variables\n";
        // }

        // the_module->print(llvm::errs(), nullptr);
        

    }
}


int main(int argc, const char** argv) {
    DEBUG_LOG("main func");

    if(argc == 1) {
        std::cout<<"Format: ./baasha [filename]"<<std::endl;
    }

    Baasha::compileFile(argv[1]);

    return 0;
}