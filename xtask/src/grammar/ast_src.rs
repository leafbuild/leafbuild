pub(crate) struct KindsSrc<'a> {
    pub(crate) punct: &'a [(&'a str, &'a str)],
    pub(crate) keywords: &'a [&'a str],
    pub(crate) contextual_keywords: &'a [&'a str],
    pub(crate) literals: &'a [&'a str],
    pub(crate) tokens: &'a [&'a str],
    pub(crate) nodes: &'a [&'a str],
}

pub(crate) const KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        (",", "COMMA"),
        ("+", "PLUS"),
        ("-", "MINUS"),
        ("/", "SLASH"),
        ("*", "STAR"),
        ("%", "PERCENT"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("{", "L_CURLY"),
        ("}", "R_CURLY"),
        ("[", "L_BRACKET"),
        ("]", "R_BRACKET"),
        ("<", "L_ANGLE"),
        (">", "R_ANGLE"),
        ("=", "EQ"),
        ("!", "BANG"),
        ("==", "EQEQ"),
        ("!=", "NOT_EQ"),
        (">=", "GREATER_EQ"),
        ("<=", "LESSTHAN_EQ"),
        ("+=", "PLUS_EQ"),
        ("-=", "MINUS_EQ"),
        ("*=", "STAR_EQ"),
        ("/=", "SLASH_EQ"),
        ("%=", "PERCENT_EQ"),
        ("&&", "AMP_AMP"),
        ("||", "PIPE2"),
        (";", "SEMICOLON"),
        (":", "COLON"),
        ("?", "QMARK"),
        (".", "DOT"),
    ],
    keywords: &[
        "let", "if", "else", "foreach", "struct", "fn", "break", "continue", "return", "true",
        "false", "and", "or", "not", "in",
    ],
    contextual_keywords: &[],
    literals: &["INT_NUMBER", "FLOAT_NUMBER", "STR", "MULTILINE_STR"],
    tokens: &[
        "ERROR",
        "IDENT",
        "WHITESPACE",
        "COMMENT",
        "NEWLINE",
        "BLOCK_COMMENT",
        "SHEBANG",
    ],
    nodes: &[
        "BUILD_DEFINITION",
        "K_EXPR",
        "STMT_WRAP",
        "FN_CALL_EXPR",
        "INDEX_EXPR",
        "STR_LIT",
        "PRIMARY_EXPR",
        "PREFIX_UNARY_EXPR",
        "ARRAY_LIT_EXPR",
        "MAP_LIT_EXPR",
        "TUPLE_EXPR",
        "BOOL_LIT",
        "FN_ARG_EXPR",
        "FN_CALL_ARGS",
        "INDEX_EXPR_BRACKETS",
        "BIN_EXPR",
        "CONDITIONAL",
        "CONDITIONAL_STMT",
        "BLOCK",
        "EXPR_EVAL_STMT",
        "ASSIGNMENT_STMT",
        "DECLARATION_STMT",
        "CONTINUE_STMT",
        "BREAK_STMT",
        "RETURN_STMT",
        "FOREACH_STMT",
        "FOREACH_IN_EXPR",
        "STRUCT_DECL",
        "STRUCT_FIELD_LIST",
        "STRUCT_FIELD",
        "TYPE_REF",
        "GENERIC_PARAMS",
    ],
};

#[derive(Default, Debug)]
pub(crate) struct AstSrc {
    pub(crate) tokens: Vec<String>,
    pub(crate) nodes: Vec<AstNodeSrc>,
    pub(crate) enums: Vec<AstEnumSrc>,
}

#[derive(Debug)]
pub(crate) struct AstNodeSrc {
    pub(crate) doc: Vec<String>,
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    pub(crate) fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Field {
    Token(String),
    Node {
        name: String,
        ty: String,
        cardinality: Cardinality,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Cardinality {
    Optional,
    Many,
}

#[derive(Debug)]
pub(crate) struct AstEnumSrc {
    pub(crate) doc: Vec<String>,
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    pub(crate) variants: Vec<String>,
}
