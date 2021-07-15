#ifndef __BAASHA_AST_CPP
#define __BAASHA_AST_CPP

#include "globals.hpp"
#include "lexer.cpp"

namespace Baasha {

// template<class Derived>
class Expr {
    public:

        class AssignExpr;
        class LiteralExpr;
        class BinaryExpr;
        class UnaryExpr;
        class CallExpr;

        template<class T=llvm::Value*>
        class Visitor {
            public:
            virtual T visit(const Expr&)              = 0;
            virtual T visit(const Expr::AssignExpr&)  = 0;
            virtual T visit(const Expr::LiteralExpr&) = 0;
            virtual T visit(const Expr::BinaryExpr&)  = 0;
            virtual T visit(const Expr::UnaryExpr&)   = 0;
            virtual T visit(const Expr::CallExpr&)    = 0;
        };

        virtual ~Expr() {};

        virtual std::string typeName() const { return "Expr"; }

        template<class T>
        T accept(Expr::Visitor<T>& visitor) const {
            return visitor.visit((*this));
        }
};

class Expr::AssignExpr : public Expr {
    public:
    std::unique_ptr<Token> name;
    std::unique_ptr<Expr> value;

    AssignExpr(std::unique_ptr<Token> name, std::unique_ptr<Expr> value)
    :   name(std::move(name)), value(std::move(value))
    {}

    template<class T>
    T accept(Visitor<T>& visitor) const  {
        return visitor.visit(*this);
    }

    std::string typeName() const override { return "AssignExpr"; }
};

class Expr::LiteralExpr : public Expr {
    public:
    std::unique_ptr<Token> token;
    // Value value; 

    LiteralExpr(std::unique_ptr<Token> token)
    :   token(std::move(token)) 
    {
        // std::cout<<"LiteralExpr() constructor\n";
    }

    template<class T>
    T accept(Visitor<T>& visitor) const  {
        visitor.visit(*this);
        return;
    }

    std::string typeName() const override { return "LiteralExpr"; }

};

class Expr::BinaryExpr : public Expr {
    public:
    std::unique_ptr<Token> oprtr;
    std::unique_ptr<Expr> left, right;

    BinaryExpr(std::unique_ptr<Expr> left, std::unique_ptr<Token> oprtr, std::unique_ptr<Expr> right)
    :   left(std::move(left)), oprtr(std::move(oprtr)), right(std::move(right))
    {}

    template<class T>
    T accept(Visitor<T>& visitor) const {
        return visitor.visit(*this);
    }

    std::string typeName() const override { return "BinaryExpr"; }
};

class Expr::UnaryExpr : public Expr {
    public:
    std::unique_ptr<Token> oprtr;
    std::unique_ptr<Expr> oprnd;

    UnaryExpr(std::unique_ptr<Token> oprtr, std::unique_ptr<Expr> oprnd) 
    :   oprtr(std::move(oprtr)), oprnd(std::move(oprnd))
    {}

    template<class T>
    T accept(Visitor<T>& visitor) const  {
        return visitor.visit(*this);
    }

    std::string typeName() const override { return "UnaryExpr"; }
};

class Expr::CallExpr : public Expr {
    public:
    std::unique_ptr<Expr> callee;
    std::unique_ptr<Token> paren;
    std::vector<std::unique_ptr<Expr>> arguments;

    CallExpr(std::unique_ptr<Expr> callee, std::unique_ptr<Token> paren, std::vector<std::unique_ptr<Expr>> arguments)
    :   callee(std::move(callee)), paren(std::move(paren)), arguments(std::move(arguments))
    {}

    template<class T>
    T accept(Visitor<T>& visitor) const  {
        return visitor.visit(*this);
    }

    std::string typeName() const override { return "CallExpr"; }
};



class Stmt {
    public:
        class VarStmt;
        class ExprStmt;
        class IfStmt;
        class LoopStmt;
        class ReturnStmt;

        template<class T=void>
        class Visitor {
            public:
            virtual T visit(const Stmt&)        = 0;
            virtual T visit(const VarStmt&)     = 0;
            virtual T visit(const ExprStmt&)    = 0;
            virtual T visit(const IfStmt&)      = 0;
            virtual T visit(const LoopStmt&)    = 0;
            virtual T visit(const ReturnStmt&)  = 0;
        };

        virtual ~Stmt() {}

        template<class T>
        T accept(Stmt::Visitor<T>& visitor) const {
            return visitor.visit(*this);
        }


        virtual std::string typeName() const { return "Stmt"; }
};

class Stmt::VarStmt : public Stmt {
    public:
    std::string name;
    std::string type;   
    // made it string so that we can print user-defined types instead of generic 'object'. They call it reflections , I think.
    std::unique_ptr<Expr> initializer;

    VarStmt(const std::string& name, const std::string& type, std::unique_ptr<Expr> initializer)
    : name(name), type(type), initializer(std::move(initializer)) 
    {}

    template<class T>
    T accept(Stmt::Visitor<T>& visitor) const  {
        return visitor.visit(*this);
    }

    std::string typeName() const override { return "VarStmt"; }
};

class Stmt::ExprStmt : public Stmt {
    public:
    std::unique_ptr<Expr> expression;

    ExprStmt(std::unique_ptr<Expr> expression)
    : expression(std::move(expression)) 
    {}

    template<class T>
    T accept(Visitor<T>& visitor) const  {
        return visitor.visit(*this);
    }

    std::string typeName() const override { return "ExprStmt"; }
};

class Stmt::IfStmt : public Stmt {
    public:
    std::unique_ptr<Expr> condition;
    std::unique_ptr<Stmt> thenBody, elseBody;

    IfStmt(std::unique_ptr<Expr> condition, std::unique_ptr<Stmt> thenBody, std::unique_ptr<Stmt> elseBody) 
    : condition(std::move(condition)), thenBody(std::move(thenBody)), elseBody(std::move(elseBody))
    {}

    template<class T>
    T accept(Visitor<T>& visitor) const  {
        return visitor.visit(*this);
    }

    std::string typeName() const override { return "IfStmt"; }
};

class Stmt::LoopStmt : public Stmt {
    public:
    std::unique_ptr<Expr> condition;
    std::unique_ptr<Stmt> body;

    LoopStmt(std::unique_ptr<Expr> condition, std::unique_ptr<Stmt> body) 
    :   condition(std::move(condition)), body(std::move(body))
    {}

    template<class T>
    T accept(Visitor<T>& visitor) const  {
        return visitor.visit(*this);
    }

    std::string typeName() const override { return "LoopStmt"; }
};

class Stmt::ReturnStmt : public Stmt {
    public:
    std::unique_ptr<Expr> retExpr;

    ReturnStmt(std::unique_ptr<Expr> retExpr)
    :   retExpr(std::move(retExpr))
    {}

    template<class T>
    T accept(Visitor<T>& visitor) const  {
        return visitor.visit(*this);
    }

    std::string typeName() const override { return "ReturnStmt"; }
};





template<class StmtRetType=void, class ExprRetType=llvm::Value*>
class CodegenVisitor : public Expr::Visitor<ExprRetType>, public Stmt::Visitor<StmtRetType> {

    StmtRetType visit(const Stmt& stmt) override {
        if(stmt.typeName() == "Stmt") {
            return StmtRetType();
        } else if(stmt.typeName() == "VarStmt") {
            return this->visit(static_cast<const Stmt::VarStmt&>(stmt));
        } else if(stmt.typeName() == "IfStmt") {
            return this->visit(static_cast<const Stmt::IfStmt&>(stmt));
        } else if(stmt.typeName() == "LoopStmt") {
            return this->visit(static_cast<const Stmt::LoopStmt&>(stmt));
        } else if(stmt.typeName() == "ReturnStmt") {
            return this->visit(static_cast<const Stmt::ReturnStmt&>(stmt));
        } else if(stmt.typeName() == "ExprStmt") {
            return this->visit(static_cast<const Stmt::ExprStmt&>(stmt));
        } else {
            return StmtRetType();
        }
    }

    StmtRetType visit(const Stmt::VarStmt& stmt)  override {
        // std::cout<<"Entered varstmt visit\n";
        llvm::Value* value = (llvm::Constant*)nullptr;
        if(stmt.initializer != nullptr) {
            value = (llvm::Value*)stmt.initializer->accept(*this);
        }

        llvm::GlobalVariable *gl_var = nullptr;

        if(!stmt.type.empty()) {
            // std::cout<<"Creating a var stmt:"<<stmt.type<<"\n";
            switch(keywordEnumVal(stmt.type)) {

                case TokenType::K_UINT8: 
                case TokenType::K_INT8:
                    the_module->getOrInsertGlobal(stmt.name, ir_builder->getInt8Ty());
                    gl_var = the_module->getNamedGlobal(stmt.name);
                    gl_var->setInitializer((llvm::Constant*)value);
                    gl_var->setLinkage(llvm::GlobalValue::LinkageTypes::PrivateLinkage);
                    break;
                    
                
                case TokenType::K_UINT16:
                case TokenType::K_INT16:
                    the_module->getOrInsertGlobal(stmt.name, ir_builder->getInt16Ty());
                    gl_var = the_module->getNamedGlobal(stmt.name);
                    gl_var->setInitializer((llvm::Constant*)value);
                    gl_var->setLinkage(llvm::GlobalValue::LinkageTypes::PrivateLinkage);
                    break;
                
                case TokenType::K_UINT32:
                case TokenType::K_INT32:
                    the_module->getOrInsertGlobal(stmt.name, ir_builder->getInt32Ty());
                    gl_var = the_module->getNamedGlobal(stmt.name);
                    if(value != nullptr) gl_var->setInitializer((llvm::Constant*)value);
                    gl_var->setLinkage(llvm::GlobalValue::LinkageTypes::PrivateLinkage);
                    break;
                
                case TokenType::K_UINT64:
                case TokenType::K_INT64:
                    the_module->getOrInsertGlobal(stmt.name, ir_builder->getInt64Ty());
                    gl_var = the_module->getNamedGlobal(stmt.name);
                    if(value != nullptr) gl_var->setInitializer((llvm::Constant*)value);
                    gl_var->setLinkage(llvm::GlobalValue::LinkageTypes::PrivateLinkage);
                    break;

                case TokenType::K_FLOAT32:
                    the_module->getOrInsertGlobal(stmt.name, ir_builder->getFloatTy());
                    gl_var = the_module->getNamedGlobal(stmt.name);
                    if(value != nullptr) gl_var->setInitializer((llvm::Constant*)value);
                    gl_var->setLinkage(llvm::GlobalValue::LinkageTypes::PrivateLinkage);
                    break;

                case TokenType::K_FLOAT64:
                    the_module->getOrInsertGlobal(stmt.name, ir_builder->getDoubleTy());
                    gl_var = the_module->getNamedGlobal(stmt.name);
                    if(value != nullptr) gl_var->setInitializer((llvm::Constant*)value);
                    gl_var->setLinkage(llvm::GlobalValue::LinkageTypes::PrivateLinkage);
                    break;
                    
            }

            named_values[stmt.name] = gl_var;
            
        } else {
            if(value == nullptr) {
                // user has neither provided initializer nor datatype. 
                // Ideally, this case is already handled by parser. This error gen is just in case.
                logger->logMessage(LogLevel::ERROR, "Variable declaration needs either initializer or datatype\n");
                return StmtRetType();
            }
            switch(value->getType()->getTypeID()) {
                case llvm::Type::TypeID::IntegerTyID:
                    // std::cout<<"Initializer is int\n";
                    the_module->getOrInsertGlobal(stmt.name, ir_builder->getInt64Ty());
                    gl_var = the_module->getNamedGlobal(stmt.name);
                    if(value != nullptr) gl_var->setInitializer((llvm::Constant*)value);
                    gl_var->setLinkage(llvm::GlobalValue::LinkageTypes::PrivateLinkage);
                    break;

                case llvm::Type::TypeID::FloatTyID:
                    // std::cout<<"Initializer is float\n";
                    the_module->getOrInsertGlobal(stmt.name, ir_builder->getFloatTy());
                    gl_var = the_module->getNamedGlobal(stmt.name);
                    gl_var->setInitializer((llvm::Constant*)value);
                    gl_var->setLinkage(llvm::GlobalValue::LinkageTypes::PrivateLinkage);
                    break;

                case llvm::Type::TypeID::DoubleTyID:
                    // std::cout<<"Initializer is double\n";
                    the_module->getOrInsertGlobal(stmt.name, ir_builder->getDoubleTy());
                    gl_var = the_module->getNamedGlobal(stmt.name);
                    gl_var->setInitializer((llvm::Constant*)value);
                    gl_var->setLinkage(llvm::GlobalValue::LinkageTypes::PrivateLinkage);
                    break;
            }

            named_values[stmt.name] = gl_var;
        }
    }

    StmtRetType visit(const Stmt::ExprStmt& stmt)  override {
        return StmtRetType();
    }

    StmtRetType visit(const Stmt::IfStmt& stmt)  override {
        return StmtRetType();
    }

    StmtRetType visit(const Stmt::LoopStmt& stmt)  override {
        return StmtRetType();
    }

    StmtRetType visit(const Stmt::ReturnStmt& stmt)  override {
        return StmtRetType();
    }


    ExprRetType visit(const Expr& expr) override {
        if(expr.typeName() == "Expr") {
                return ExprRetType();
            } else if(expr.typeName() == "AssignExpr") {
                return this->visit(static_cast<const Expr::AssignExpr&>(expr));
            } else if(expr.typeName() == "LiteralExpr") {
                return this->visit(static_cast<const Expr::LiteralExpr&>(expr));
            } else if(expr.typeName() == "BinaryExpr") {
                return this->visit(static_cast<const Expr::BinaryExpr&>(expr));
            } else if(expr.typeName() == "UnaryExpr") {
                return this->visit(static_cast<const Expr::UnaryExpr&>(expr));
            } else if(expr.typeName() == "CallExpr") {
                return this->visit(static_cast<const Expr::CallExpr&>(expr));
            } else {
                return ExprRetType();
            }
            
    }

    ExprRetType visit(const Expr::AssignExpr& expr)  override {
        return ExprRetType();
    } 

    ExprRetType visit(const Expr::UnaryExpr& expr)  override {
        return ExprRetType();
    }

    ExprRetType visit(const Expr::BinaryExpr& expr)  override {
        return ExprRetType();
    }

    ExprRetType visit(const Expr::CallExpr& expr)  override {
        return ExprRetType();
    }

    ExprRetType visit(const Expr::LiteralExpr& expr)  override {
        std::string literal_str = expr.token->getTokenString(source_code);

        switch(expr.token->type) {
            case TokenType::INT_LITERAL:
                return llvm::ConstantInt::get(*the_context, llvm::APInt(64, literal_str.c_str(), 10));
            case TokenType::HEX_LITERAL:
                literal_str = literal_str.substr(2, literal_str.length()-2);
                return llvm::ConstantInt::get(*the_context, llvm::APInt(64, literal_str.c_str(), 16));
            case TokenType::OCTAL_LITERAL:
                literal_str = literal_str.substr(2, literal_str.length()-2);
                return llvm::ConstantInt::get(*the_context, llvm::APInt(64, literal_str.c_str(), 8));
            case TokenType::FLOAT_LITERAL: {
                double float64 = strtod(literal_str.c_str(), NULL);
                return llvm::ConstantFP::get(*the_context, llvm::APFloat(float64));
            }
            default: return llvm::ConstantFP::get(*the_context, llvm::APFloat(0.0f));
        }
    }

};


};

#endif // __BAASHA_AST_CPP