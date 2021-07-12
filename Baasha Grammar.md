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
program   -> declaration* EOF

declaration -> varDecl | classDecl | funDecl | statement;

varDecl  -> VISIBILITY? "var" IDENTIFIER TYPE ("=" expression)? ";"
            | IDENTIFIER ":=" expression ";" ;

expression -> assignment;

assignment -> IDENTIFIER "=" assignment | logic_or;

logic_or -> logic_and ("or" logic_and)* ;
logic_and -> equality ("and" equality)
```

### AST nodes

```
Stmt
varStmt -> Token nameToken, Token typeToken, Expr initializer
classStmt -> Token nameToken, Token 

Expr 
AssignExpr -> Variable lvalue, Expr rvalue

```