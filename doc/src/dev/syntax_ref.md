# Syntax reference

```
KExpr ::= ID '=' Expr

# Expressions
ArrayLitExpr ::= '[' (Expr ',')* Expr? ']'
MapLitExpr ::= '{' (KExpr ',')* KExpr? '}'
TupleExpr ::= '(' (Expr ',')* Expr? ')'

NumLit ::= "([1-9][0-9]*|0x[0-9a-fA-F]+|0b[01]+|0[0-7]+|0)[uU]?[lL]?"
StrLit ::= "'(\\['nt\\]|[^'\\])+'"
         | "'''[.\n]*?'''"
BoolLit ::= 'true' | 'false'
ID ::= "[a-zA-Z_][a-zA-Z0-9_]*"

FArg ::= KExpr | Expr
FCallArgs ::= '(' (FArg ',')* FArg? ')'
IndexExprBraces ::= '[' Expr ']'

Primary ::= ArrayLitExpr
          | MapLitExpr
          | NumLit
          | ID
          | StrLit
          | BoolLit
          | TupleExpr
          | Conditional



UnaryExprOp ::= '-' | '+'

Factor ::= UnaryExprOp Factor
         | Primary
Term ::= Factor (('*' | '/' | '%') Factor)*

Precedence1Expr ::= Primary (FCallParens | IndexExprBraces)*
Precedence2Expr ::= ('-' | '+') Precedence2Expr
                  | Precedence1Expr
Precedence3Expr ::= Precedence2Expr (('*' | '/' | '%') Precedence2Expr)*
Precedence4Expr ::= Precedence3Expr (('+' | '-') Precedence3Expr)*
Precedence5Expr ::= Precedence4Expr (('<' | '>' | '<=' | '>=') Precedence4Expr)*
Precedence6Expr ::= Precedence5Expr (('==' | '!=') Precedence5Expr)*
Precedence7Expr ::= Precedence6Expr ('and' Precedence6Expr)*
Precedence8Expr ::= Precedence7Expr ('or' Precedence7Expr)*

Expr ::= Precedence8Expr

# Control flow
Conditional ::= 'if' Expr Block ('else' 'if' Expr Block)* ('else' Block)?

# Statements
Block ::= '{' Statement* '}'
ExprEvalStatement ::= Expr '\n'

Continue ::= 'continue' '\n'
Break ::= 'break' Expr? '\n'
Return ::= 'return' Expr? '\n'

ForeachStatement ::= 'foreach' ForeachInExpr Block

StructDecl ::= 'struct' ID StructFieldList
StructFieldList ::= '{' StructField* '}'
StructField ::= ID ':' TypeRef

# Types
TypeRef ::= ID GenericParams?
GenericParams ::= '<' (TypeRef ',')* TypeRef? '>'
```