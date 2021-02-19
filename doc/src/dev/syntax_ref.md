# Syntax reference

```
KExpr ::= ID '=' Expr

# Expressions
ArrayLitExpr ::= '[' (Expr ',')* Expr? ']'
MapLitExpr ::= '{' (KExpr ',')* KExpr? '}'

FArg ::= KExpr | Expr
FArgs ::= (FArg ',')* FArg?

Primary ::= ArrayLitExpr
          | MapLitExpr
          | NumLit
          | ID
          | StrLit

StrLit ::= "'.*?'" # Remember escapes
         | "'''.*?'''"

Factor ::= Primary
Term ::= Factor (('*' | '/' | '%') Factor)*

Expr ::= ArrayLitExpr
       | MapLitExpr
       | NumLit
       | ID
       | Expr '(' FArgs ')'
       | Expr '[' Expr ']'
       | UnaryOp Expr
       | Expr InfixOp1 Expr
       | Expr InfixOp2 Expr
```