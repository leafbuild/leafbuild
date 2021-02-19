//! Holds [`SyntaxKind`]
/// The syntax kind; see rowan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    // tokens
    /// `(`
    L_PAREN = 0,
    /// `)`
    R_PAREN,

    /// `[`
    L_BRACKET,
    /// `]`
    R_BRACKET,

    /// `{`
    L_BRACE,
    /// `}`
    R_BRACE,

    /// `+=`
    PLUS_EQ,
    /// `-=`
    MINUS_EQ,
    /// `*=`
    MUL_EQ,
    /// `/=`
    DIV_EQ,
    /// `%=`
    MOD_EQ,

    /// `+`
    PLUS,
    /// `-`
    MINUS,
    /// `*`
    ASTERISK,
    /// `/`
    SLASH,
    /// `%`
    PERCENT,
    /// `==`
    EQ_EQ,
    /// `>=`
    GREATER_EQ,
    /// `>`
    GREATER,
    /// `<=`
    LESS_EQ,
    /// `<`
    LESS,
    /// `!=`
    NOT_EQ,
    /// `=`
    EQ,
    /// `<<`
    SHIFT_LEFT,
    /// `>>`
    SHIFT_RIGHT,
    /// `.`
    DOT,
    /// `:`
    COLON,
    /// `?`
    QMARK,
    /// `;`
    SEMICOLON,
    /// `,`
    COMMA,
    /// `~`
    TILDE,
    /// `and`
    AND_KW,
    /// `or`
    OR_KW,
    /// `not`
    NOT_KW,
    /// `in`
    IN_KW,
    /// `let`
    LET_KW,
    /// `if`
    IF_KW,
    /// `else`
    ELSE_KW,
    /// `foreach`
    FOREACH_KW,
    /// `continue`
    CONTINUE_KW,
    /// `break`
    BREAK_KW,
    /// `return`
    RETURN_KW,
    /// `true`
    TRUE_KW,
    /// `false`
    FALSE_KW,
    /// `fn`
    FN_KW,
    /// Number
    NUMBER,
    /// Identifier
    ID,
    /// String
    STRING,
    /// String, but multiline
    MULTILINE_STRING,
    /// Line comment
    LINE_COMMENT,
    /// Block comment
    BLOCK_COMMENT,
    /// Whitespace
    WHITESPACE,

    /// Newline
    NEWLINE,

    /// Error
    ERROR,

    // composites
    /// `[1,2,3]`
    ArrayLitExpr,
    /// `{a:1,b:2}`
    MapLitExpr,

    /// Number / Bool / Str / Id / ArrayLit / MapLit
    AtomExpr,

    /// `a <op> b`
    BinOpExpr,

    /// `f()`
    FuncCallExpr,

    /// `a.f()`
    MethodCallExpr,

    /// `a.b`
    PropertyAccessExpr,

    /// `( <expr> )`
    TupleExpr,

    /// `a[b]`
    IndexedExpr,

    /// Arguments to function call
    /// `(Expr Comma)* Expr?`
    FuncCallArgs,

    /// Positional arg
    /// `expr`
    PositionalArg,

    /// Default arg
    /// `name = expr`
    DefaultArg,

    /// `name = value`, `name += value`, ...
    Assignment,
    /// `let name = value`
    Declaration,
    /// `if (condition) {...}`
    If,
    /// `else <If>`
    ElseIf,
    /// `else {...}`
    Else,

    /// if statement
    Conditional,

    /// foreach statement
    Foreach,

    /// `<Expr> in <Expr>`
    ForInExpr,

    /// `continue;`, `break;`, `return ...;`
    ControlStatement,

    /// Something like `f();`
    ExprStatement,

    /// `{ <statement>*; <expr>? }`
    FnBody,

    /// `(a, b, ...)`, including `()`
    Tuple,

    ///
    ExprBlock,

    ///
    ConditionalBranch,

    /// for the root node
    ROOT,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind.into())
    }
}

impl From<&SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: &SyntaxKind) -> Self {
        Self(kind.into())
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<u16> for SyntaxKind {
    fn from(i: u16) -> Self {
        assert!(i <= Self::ROOT as u16);
        // only usage of unsafe code
        #[allow(unsafe_code)]
        unsafe {
            std::mem::transmute::<u16, Self>(i)
        }
    }
}

impl From<SyntaxKind> for u16 {
    fn from(kind: SyntaxKind) -> Self {
        kind as Self
    }
}

impl From<&SyntaxKind> for u16 {
    fn from(kind: &SyntaxKind) -> Self {
        (*kind).into()
    }
}

impl SyntaxKind {
    pub(crate) fn token_name(self) -> String {
        use SyntaxKind::*;
        match self {
            L_PAREN => "(",
            R_PAREN => ")",
            L_BRACKET => "[",
            R_BRACKET => "]",
            L_BRACE => "{",
            R_BRACE => "}",
            PLUS_EQ => "+=",
            MINUS_EQ => "-=",
            MUL_EQ => "*=",
            DIV_EQ => "/=",
            MOD_EQ => "%=",
            PLUS => "+",
            MINUS => "-",
            ASTERISK => "*",
            SLASH => "/",
            PERCENT => "%",
            EQ_EQ => "==",
            GREATER_EQ => ">=",
            GREATER => ">",
            LESS_EQ => "<=",
            LESS => "<",
            NOT_EQ => "!=",
            EQ => "=",
            SHIFT_LEFT => "<<",
            SHIFT_RIGHT => ">>",
            DOT => ".",
            COLON => ":",
            QMARK => "?",
            SEMICOLON => ";",
            COMMA => ",",
            TILDE => "~",
            AND_KW => "and",
            OR_KW => "or",
            NOT_KW => "not",
            IN_KW => "in",
            LET_KW => "let",
            IF_KW => "if",
            ELSE_KW => "else",
            FOREACH_KW => "foreach",
            CONTINUE_KW => "continue",
            BREAK_KW => "break",
            RETURN_KW => "return",
            TRUE_KW => "true",
            FALSE_KW => "false",
            FN_KW => "fn",
            NUMBER => "number",
            ID => "id",
            STRING => "'...'",
            MULTILINE_STRING => "'''...'''",
            LINE_COMMENT => "//...\n",
            BLOCK_COMMENT => "/*...*/",
            NEWLINE => "\\n",
            WHITESPACE => " ",
            ERROR => "error",
            _ => "",
        }
        .into()
    }
}
