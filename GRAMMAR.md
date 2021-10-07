## Baasha

A modern programming language inspired by Rust and Golang. 

**Features in first iteration:**
- Rust-like structs, impl and traits.
- goroutine-like green threads
- Every other basic language features which make it turing complete.

Features in future iteration:
- Templates
- Rust-like enums and Pattern matching
- Closures


**Syntax Grammar:**

```
program         -> declaration* EOF

declaration     -> funcDef | structDecl | implDecl | traitDecl
prototype       -> "func" IDENTIFIER "(" parameters? ")" returntypes?
funcDef         -> prototype block 
funcDecl        -> prototype ";"
structDecl      -> "struct" IDENTIFIER "{" ( IDENTIFIER ("," IDENTIFIER)* ":" DATATYPE  )* "}"
implDecl        -> "impl" IDENTIFIER ("for" IDENTIFIER)? "{" funcDef* "}"
traitDecl       -> "trait" IDENTIFIER "{" (funcDecl | funcDef)* "}"


block           -> "{" statement* "}"
statement       -> varStmt | ifStmt | returnStmt | forStmt | loopStmt | declStmt | exprStmt
varStmt         -> "var" IDENTIFIER ((":" types) | ("=" expression) | (":" types) ("=" expression)) ";"
ifStmt          -> "if" expression block ("else" block|ifStmt)?
returnStmt      -> "return" expr_list ";"
loopStmt        -> "loop" expression block
forStmt         -> "for"    assignment ";" expression ";" expr_list block
whileStmt       -> "while" expression block

exprStmt        -> (expression)? ";"
expression      -> assignment-expr | expression "," assignment-expr         -> assignment-expr ("," assignment-expr)*


assignment-expr -> /*conditional-expr*/ logORexpr | unaryExpr assignment-op assignment-expr
                        -> logORexpr (assignment-op assignment-expr)*
assignment-op   -> "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" | "<<=" | ">>="


target          -> identifier | attributeref | /* subscription | slicing */


logORexpr        -> logANDexpr | logORexpr "or" logANDexpr          -> logANDexpr ("or" logANDexpr)*
logANDexpr       -> inclORexpr | logANDexpr "and" inclORexpr        -> inclORexpr ("and" inclORexpr)*
inclORexpr       -> exclORexpr | inclORexpr "|" exclORexpr          -> exclORexpr ("|" exclORexpr)*
exclORexpr       -> ANDexpr | exclORexpr "^" ANDexpr                -> ANDexpr    ("^" ANDexpr)*
ANDexpr          -> equality-expr | ANDexpr "&" equality-expr       -> equality-expr ("&" equality-expr)*
equality-expr    -> relational-expr | equality-expr ("==" | "!=") relational-expr   -> relational-expr (("=="|"!=") relational-expr)*
relational-expr  -> shift-expr | relational-expr relational-op shift-expr     -> shift-expr (relational-op shift-expr)*
shift-expr       -> additive-expr | shift-expr ("<<"|">>") additive-expr      -> additive-expr (("<<"|">>") additive-expr)*
additive-expr    -> mul-expr | additive-expr ("+"|"-") mul-expr               -> mul-expr (("+"|"-") mul-expr)*
mul-expr         -> unary | mul-expr ("*"|"/"|"%") unary                      -> unary (("*"|"/"|"%") unary)*
unary            -> unary-op unary | primary                                  -> unary-op* primary

unary-op         -> "~" | "!" | "-" | "+"
relational-op    -> "<"|">"|"<="|">="

primary     -> atom | attributeref | /* subscription | slicing */ | call | cast
atom        -> identifier | literal | grouping | structExpr
grouping    -> "(" expression ")"
structExpr  -> identifier "{" (identifier ":" expression)* ""
attributeref-> primary "." identifier
call        -> primary "(" expr_list ")" 
cast        -> primary "as" DATATYPE



DATATYPE    -> "int8"|"int16"|"int32"|"int64"|"uint8"|"uint16"|"uint32"|"uint64"|"float32"|"float64"|"bool"|IDENTIFIER
type_list   -> DATATYPE ("," DATATYPE)*
```
 
**Lexical Grammar:**
```
INTEGER     -> HEX_LIT | OCTAL_LIT | DIGIT+
DIGIT       -> "0"..."9"
ALPHA       -> "a"..."z"|"A"..."Z"
HEX_ALPHA   -> "a"..."f"|"A..."F"

HEX_LIT     -> "0" ("x"|"X") (DIGIT|HEX_ALPHA)+
OCTAL_LIT   -> "0" ("o"|"O") ("0"..."7")+

FLOAT       -> DIGIT+ ( "."| (("e"|"E") ("+"|"-")) ) DIGIT+

IDENTIFIER  -> ("_" | ALPHA) ALPHA+
```


**Operator Precedene and Associativity**

