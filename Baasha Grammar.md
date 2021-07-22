## Baasha Grammar


// TODO
--- change returntypes from struct to some other thing.
--- add resolution for structure names.
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

stmt -> varStmt | funcStmt | structStmt | implStmt | exprStmt | ifStmt | returnStmt | loopStmt | block;

varStmt -> "var" IDENTIFIER type? ( "=" expression)? ("," IDENTIFIER type? ( "=" expression)?)*


funcStmt -> prototype (";" | block) ;
prototype -> "func" IDENTIFIER "(" parameters? ")" ("->" "(" parameters? ")")? 
block -> "{" stmt* "}" ;
returnStmt -> "return" arguments ";"

<!-- funcDefStmt -> "func" IDENTIFIER "(" parameters? ")" ("->" "(" parameters? ")")? block; -->
<!-- funcDecStmt -> "func" IDENTIFIER "(" parameters? ")" ("->" "(" parameters? ")")? ";"; -->

parameters -> (type parameter ("," parameter )* ) ("," type parameter ("," parameter )* )* ;
parameter -> IDENTIFIER ("=" expression)?
arguments -> expression ("," expression)* ;


structStmt -> "struct" IDENTIFIER "{" varStmt* "}" 


implStmt -> "impl" IDENTIFIER "{" (prototype block)* "}"


exprStmt -> expression ";"

ifStmt -> "if" expression "{" statement "}" ("else" "{" statement "}")?
loopStmt -> "loop" expression "{" statement "}"

returnStmt -> "return" expression* ";"




expression -> assignment;

assignment -> ( call "." )? IDENTIFIER "=" assignment | logic_or;

logic_or -> logic_and ("or" logic_and)* ;
logic_and -> equality ("and" equality)* ;
equality -> comparison (("!=" | "==") comparison)* ;
comparison -> term ((">" | ">=" | "<" | "<=") term)* ;
term -> factor (("-" | "+") factor)* ;
factor -> unary (("/" | "*") unary)* ;

unary -> ("!"|"-") unary | call;
call -> primary ( "(" arguments? ")" | "." IDENTIFIER)* ;

primary -> INTEGER | "true" | "false" | FLOAT | IDENTIFIER ;
type -> "uint8" | "uint16" | "uint32" | "uint64" | "int8" | "int16" | "int32" | "int64" | "bool" | "float32" | "float64";
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