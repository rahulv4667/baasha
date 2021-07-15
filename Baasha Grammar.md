## Baasha Grammar

<!--
Borrowed from Lox and Golang
-->
### Lexical Grammar
```
NUMBER -> 0 [HEX_LIT | OCTAL_LIT] | DIGIT+ (FLOAT | DIGIT*)?
STRING -> "\"" <any char except "\"">* "\"";
IDENTIFIER -> ALPHA ( DIGIT | ALPHA)*
ALPHA -> "a"....."z" | "A"...."Z" | "_"
DIGIT -> "0"..."9"

```

### Syntax Grammar
```
program   -> statement* EOF

stmt -> varStmt | classStmt | exprStmt | ifStmt | returnStmt | loopStmt | block;

varStmt -> "var" type? IDENTIFIER ( "=" expression)? ("," type? IDENTIFIER ( "=" expression)?)*

classStmt -> "class" IDENTIFIER "{" classBlock "}"
classBlock -> ("struct" "{" varStmt* "}")? ("impl" "{" function* "}")

exprStmt -> expression ";"

ifStmt -> "if" expression "{" statement "}" ("else" "{" statement "}")?
loopStmt -> "loop" expression "{" statement "}"

returnStmt -> "return" expression* ";"

expression -> assignment;

assignment -> IDENTIFIER "=" assignment | logic_or;

logic_or -> logic_and ("or" logic_and)* ;
logic_and -> equality ("and" equality)

type -> "uint8" | "uint16" | "uint32" | "uint64" | "int8" | "int16" | "int32" | "int64" | "bool";
```

### AST nodes

```
Stmt
varStmt -> Token nameToken, Token typeToken, Expr initializer
classStmt -> Token nameToken, Stmt superclass, 
exprStmt -> Expr expression
ifStmt -> Expr Condition, Stmt thenBlock, Stmt elseBlock
loopStmt -> Expr condition, Stmt loopBlock
returnStmt -> Expr returnVal;


Expr 
AssignExpr -> Variable lvalue, Expr rvalue

```