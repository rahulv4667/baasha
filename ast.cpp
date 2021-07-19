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
        class PrototypeStmt;
        class FunctionStmt;
        class BlockStmt;
        class ReturnStmt;

        template<class T=void>
        class Visitor {
            public:
            virtual T visit(const Stmt&)            = 0;
            virtual T visit(const VarStmt&)         = 0;
            virtual T visit(const ExprStmt&)        = 0;
            virtual T visit(const IfStmt&)          = 0;
            virtual T visit(const LoopStmt&)        = 0;
            virtual T visit(const PrototypeStmt&)   = 0;
            virtual T visit(const FunctionStmt&)    = 0;
            virtual T visit(const BlockStmt&)       = 0;
            virtual T visit(const ReturnStmt&)      = 0;
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

class Stmt::PrototypeStmt : public Stmt {
    public:
    std::string name;
    bool isDecl;
    std::map<std::string, llvm::Type*> params;
    std::vector<TokenType> returnTypes;

    PrototypeStmt(
        const std::string& name, 
        std::map<std::string, llvm::Type*> params, 
        std::vector<TokenType> returnTypes, 
        bool isDecl
    ) 
    :   name(std::move(name)), params(std::move(params)), returnTypes(std::move(returnTypes)), isDecl(isDecl)
    {}

    template<class T>
    T accept(Visitor<T>& visitor) const {
        return visitor.visit(*this);
    } 

    std::string typeName() const override { return "PrototypeStmt"; }
};

class Stmt::FunctionStmt : public Stmt {
    public:
    std::unique_ptr<Stmt::PrototypeStmt> prototype;
    std::unique_ptr<Stmt::BlockStmt> body;

    FunctionStmt(std::unique_ptr<Stmt::PrototypeStmt> prototype, std::unique_ptr<Stmt::BlockStmt> body)
    :  prototype(std::move(prototype)), body(std::move(body))
    {}

    template<class T>
    T accept(Visitor<T>& visitor) const {
        return visitor.visit(*this);
    } 

    std::string typeName() const override { return "FunctionStmt"; }
};

class Stmt::BlockStmt : public Stmt {
    public: 
    std::vector<std::unique_ptr<Stmt>> statements;

    BlockStmt(std::vector<std::unique_ptr<Stmt>> statements)
    :   statements(std::move(statements))
    {}

    template<class T>
    T accept(Visitor<T>& visitor) const {
        return visitor.visit(*this);
    }

    std::string typeName() const override { return "BlockStmt"; }
};

class Stmt::ReturnStmt : public Stmt {
    public:
    std::vector<std::unique_ptr<Expr>> retExpr;

    ReturnStmt(std::vector<std::unique_ptr<Expr>> retExpr)
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
        } else if(stmt.typeName() == "FunctionStmt") {
            return this->visit(static_cast<const Stmt::FunctionStmt&>(stmt));
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

        if(scope == Scope::FUNCTION) {
            llvm::AllocaInst* alloca;
            if(!stmt.type.empty()) {
                TokenType type = keywordEnumVal(stmt.type);
                alloca = createEntryBlockAlloca(
                    ir_builder->GetInsertBlock()->getParent(),
                    getLLVMTypeRaw(type),
                    stmt.name
                );

                named_values[stmt.name] = alloca;
                return StmtRetType();
            }
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
                    // llvm::Type* intTy = llvm::TypeBuilder
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

    StmtRetType visit(const Stmt::PrototypeStmt& stmt) override {
        // TODO: change to support non-primitive types
        llvm::Type* retType;
        // Assign the correct type
        std::vector<llvm::Type*> retElems;
        for(auto type : stmt.returnTypes) {
            retElems.emplace_back(getLLVMTypeRaw(type));
        }
        retType = llvm::StructType::get(*the_context, retElems, false);

        std::vector<llvm::Type*> args;
        // fill up the args with types
        for(auto arg_itr=stmt.params.begin(); arg_itr != stmt.params.end(); arg_itr++) {
            args.push_back((*arg_itr).second);
        }

        llvm::Function* func = llvm::Function::Create(
            llvm::FunctionType::get(retType, args, false /* isVarArg -- change when supporting varargs */),
            llvm::Function::LinkageTypes::CommonLinkage,
            stmt.name,
            the_module.get()
        );

        // setting names for parameters
        auto itr = stmt.params.begin();
        for(auto &arg: func->args()) {
            arg.setName((*itr).first);
            itr++;
        }
    }

    StmtRetType visit(const Stmt::FunctionStmt& stmt) override {
        auto *func = the_module->getFunction(stmt.prototype->name);
        if(func == nullptr) {
            this->visit(*stmt.prototype);
            func = the_module->getFunction(stmt.prototype->name);
        }

        if(stmt.body == nullptr || stmt.prototype->isDecl) {
            return StmtRetType();
        } else {
            llvm::BasicBlock *func_block = llvm::BasicBlock::Create(*the_context, "entry", func);
            ir_builder->SetInsertPoint(func_block);

            scope = Scope::FUNCTION;
            auto old_values = named_values;

            // updating named values. skipping it for now.
            for(uint32_t i=0; i<func->arg_size(); i++) {
                llvm::Argument* arg = func->getArg(i);
                if(!arg->hasName()) {
                    logger->logMessage(LogLevel::WARNING, "Function argument not having a name");
                    continue;
                }
                named_values[std::string(arg->getName().data())] = arg;
            }

            // going into generation of body
            this->visit(*stmt.body);

            // setting default return type
            // ir_builder->CreateRet(llvm::Value::get)

            llvm::verifyFunction(*func);

            named_values = old_values;
        }

    }

    StmtRetType visit(const Stmt::BlockStmt& stmt) override {
        for(uint32_t i=0; i<stmt.statements.size(); i++) {
            stmt.statements[i]->accept(*this);
        }
    }

    StmtRetType visit(const Stmt::ReturnStmt& stmt) override {
        llvm::StructType* ret_type = (llvm::StructType*)ir_builder->GetInsertBlock()->getParent()->getReturnType();
        // llvm::Twine ret_name = llvm::Twine(ir_builder->GetInsertBlock()->getParent()->getName()) + "$return";
        llvm::AllocaInst* ret_value = createEntryBlockAlloca(ir_builder->GetInsertBlock()->getParent(), ret_type);
        std::cout<<"ReturnSTMT info:\n";
        // std::string s = std::string(ret_type->getStructName().data());
        // std::cout<<"Structure name:"<<s<<"\n";
        uint32_t num = ret_type->getStructNumElements();
        std::cout<<"Structure num of elements:"<<num<<"\n";
        for(uint32_t i=0; i<num; i++) {
            std::cout<<i<<"th element:";
            std::string type_str;
            llvm::raw_string_ostream rso(type_str);
            ret_type->getStructElementType(i)->print(rso, true);
            std::cout<<rso.str()<<"\n";
        }

        
        //  {llvm::APInt(32, 0, false),}
        for(uint32_t i=0; i < stmt.retExpr.size(); i++) {

            std::vector<llvm::Value*> idx_list;
            idx_list.push_back(llvm::ConstantInt::get(*the_context, llvm::APInt(32, 0, false)));
            idx_list.push_back(llvm::ConstantInt::get(*the_context, llvm::APInt(32, i, false)));

            // first, evaluate each expression. then assign its result to return sturct at appropriate index.
            llvm::Value *expr_res = stmt.retExpr[i]->accept(*this);
            std::cout<<"Getting "<<i<<"th element in return structure\n";
            llvm::Value *struct_idx = ir_builder->CreateGEP(ret_type, ret_value, idx_list);
            std::cout<<"Got "<<i<<"th element in return structure. Storing now\n";
            ir_builder->CreateStore(expr_res, struct_idx);
            std::cout<<"Stored in "<<i<<"th element in return structure\n";
        }

        ir_builder->CreateRet(ret_value);
    }

    StmtRetType visit(const Stmt::ExprStmt& stmt)  override {
        if(stmt.expression == nullptr) 
            return StmtRetType();
        else {
            stmt.expression->accept(*this);
        }
    }

    StmtRetType visit(const Stmt::IfStmt& stmt)  override {
        return StmtRetType();
    }

    StmtRetType visit(const Stmt::LoopStmt& stmt)  override {
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
        if(named_values.count(expr.name->getTokenString(source_code)) <= 0) {
            logger->logMessage(LogLevel::ERROR, "Can't assign to variable that doesn't exist");
            return ExprRetType();
        } 

        llvm::Value* var = named_values[expr.name->getTokenString(source_code)];
        llvm::Value* evalutated_value = expr.value->accept(*this); 
        if(var->getType()->isFunctionTy()) {
            // std::cout<<"it is function type\n";
            return ir_builder->CreateStore(evalutated_value, var);
        } else {
            // std::cout<<"it is local variable\n";
            return ir_builder->CreateStore(evalutated_value, var);
        }
    } 

    ExprRetType visit(const Expr::UnaryExpr& expr)  override {
        return ExprRetType();
    }

    ExprRetType visit(const Expr::BinaryExpr& expr)  override {
        std::cout<<"Visited binary expr:"<<enumStringVal(expr.oprtr->type)<<"\n";
        switch(expr.oprtr->type) {
            case TokenType::PLUS: {
                    llvm::Value* lhs = expr.left->accept(*this);
                    llvm::Value* rhs = expr.right->accept(*this);
                    if(lhs->getType()->getTypeID() == rhs->getType()->getTypeID()) {
                        std::cout<<"Types are equal\n";
                    } else  {
                        std::cout<<"Types are NOT equal\n";
                    }
                    return ir_builder->CreateAdd(lhs, rhs);
            }

            case TokenType::MINUS: {
                    llvm::Value* lhs = expr.left->accept(*this);
                    llvm::Value* rhs = expr.right->accept(*this);
                    return ir_builder->CreateSub(lhs, rhs);
            }
            
            case TokenType::ASTERISK: {
                    llvm::Value* lhs = expr.left->accept(*this);
                    llvm::Value* rhs = expr.right->accept(*this);
                    return ir_builder->CreateMul(lhs, rhs);
            }

            case TokenType::SLASH: {
                    llvm::Value* lhs = expr.left->accept(*this);
                    llvm::Value* rhs = expr.right->accept(*this);
                    
                    if(lhs->getType()->getTypeID() == llvm::Type::TypeID::IntegerTyID 
                        && rhs->getType()->getTypeID() == lhs->getType()->getTypeID())
                        return ir_builder->CreateUDiv(lhs, rhs);

                    if(lhs->getType()->getTypeID() == llvm::Type::TypeID::FloatTyID 
                        && rhs->getType()->getTypeID() == lhs->getType()->getTypeID())
                        return ir_builder->CreateFDiv(lhs, rhs);

            }

                    
        }
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
            case TokenType::IDENTIFIER: {
                auto var_name = expr.token->getTokenString(source_code);
                if(named_values.count(var_name) > 0) 
                    return named_values[expr.token->getTokenString(source_code)];
                else {
                    logger->logMessage(LogLevel::WARNING, "Trying to access undeclared variable "+var_name);
                    return ExprRetType();
                }
            }
            default: return llvm::ConstantFP::get(*the_context, llvm::APFloat(0.0f));
        }
    }

};


};

#endif // __BAASHA_AST_CPP