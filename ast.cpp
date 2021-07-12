#include "globals.hpp"
#include "lexer.cpp"

namespace Baasha {
class Visitor {
    public:
        virtual void visit(const Expr&) = 0;
        virtual void visit(const Stmt&) = 0;
};

class DerivedVisitor : public Visitor {

};

class Expr {
    public:
        virtual ~Expr() {};
        virtual void accept(Visitor *visitor) const;
};

class AssignExpr : public Expr {
    Token name;
    Expr value;

    void accept(Visitor *visitor) const override {
        return visitor->visit(*this);
    }
};

class NumberExpr : public Expr {
    std::unique_ptr<Token> token;
    Expr value;

    void accept(Visitor *visitor) const override {
        return visitor->visit(*this);
    }

};

class BinaryExpr : public Expr {
    std::unique_ptr<Token> oprtr;
    Expr left;
    Expr right;

    void accept(Visitor *visitor) const override {
        return visitor->visit(*this);
    }
};

class CallExpr : public Expr {
    Expr callee;
    std::unique_ptr<Token> paren;
    std::vector<Expr> arguments;

    void accept(Visitor *visitor) const override {
        return visitor->visit(*this);
    }
};

class LiteralExpr : public Expr {
    std::unique_ptr<Token> token;
    typedef union Value {
        uint32_t    uint32;
        uint64_t    uint64;
        uint16_t    uint16;
        uint8_t     uint8;
        int32_t     int32;
        int64_t     int64;
        int16_t     int16;
        int8_t      int8;
        float       float32;
        double      float64;
        void*       ptr; 
    } Value;

    void accept(Visitor *visitor) const override {
        return visitor->visit(*this);
    }
};

class Stmt {
    public:
        virtual ~Stmt() {}
        virtual void accept(Visitor *visitor) const = 0;
};

class ExprStmt : public Stmt {
    Expr expression;

    void accept(Visitor *visitor) const override {
        return visitor->visit(*this);
    }
};

class ForStmt : public Stmt {

};

class IfStmt : public Stmt {
    
};

}