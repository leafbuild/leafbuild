//! Syntax tree types
use crate::parser::Is;
use crate::syntax_kind::SyntaxKind::{self, *};
use crate::LeafbuildLanguage;
use leafbuild_derive::{ast_node, ast_token, FakeAstNode};
use rowan::NodeOrToken;
///
pub type SyntaxNode = rowan::SyntaxNode<LeafbuildLanguage>;
///
pub type SyntaxToken = rowan::SyntaxToken<LeafbuildLanguage>;
///
pub type SyntaxElement = NodeOrToken<SyntaxNode, SyntaxToken>;

impl Is for &SyntaxNode {
    fn is(self, kind: SyntaxKind) -> bool {
        self.kind().is(kind)
    }
}

impl Is for &SyntaxToken {
    fn is(self, kind: SyntaxKind) -> bool {
        self.kind().is(kind)
    }
}

impl Is for &SyntaxElement {
    fn is(self, kind: SyntaxKind) -> bool {
        match self {
            NodeOrToken::Node(n) => n.is(kind),
            NodeOrToken::Token(t) => t.is(kind),
        }
    }
}

///
pub trait CastableFromSyntaxNode {
    ///
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;
}

///
pub trait AstNode: CastableFromSyntaxNode {
    ///
    // #[deprecated = "use accessors instead of providing access to the underlying syntax node"]
    fn syntax(&self) -> &SyntaxNode;
}

///
pub trait FakeAstNode: CastableFromSyntaxNode {}

///
pub trait CastableFromSyntaxToken {
    ///
    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized;
}

///
pub trait AstToken: CastableFromSyntaxToken {
    ///
    fn get_text(&self) -> &str;
}

///
pub trait CastTo: Sized {
    ///
    fn cast_to<T>(self) -> Option<T>
    where
        T: CastableFromSyntaxNode;
}

impl CastTo for SyntaxNode {
    fn cast_to<T>(self) -> Option<T>
    where
        T: CastableFromSyntaxNode,
    {
        T::cast(self)
    }
}

///
pub trait CastToToken: Sized {
    ///
    fn cast_to_token<T>(self) -> Option<T>
    where
        T: CastableFromSyntaxToken;
}

impl CastToToken for SyntaxToken {
    fn cast_to_token<T>(self) -> Option<T>
    where
        T: CastableFromSyntaxToken,
    {
        T::cast(self)
    }
}

///
#[ast_token(ID)]
#[derive(Clone, Debug)]
pub struct Id {
    syntax: SyntaxToken,
}

///
#[ast_node(Expr)]
#[derive(Clone, Debug)]
pub struct Expr {
    syntax: SyntaxNode,
}

///
#[ast_node(ArrayLitExpr)]
#[derive(Clone, Debug)]
pub struct ArrayLitExpr {
    syntax: SyntaxNode,
}

impl ArrayLitExpr {
    /// Provides an iterator over the children expressions,
    /// which will be the elements of the array.
    pub fn expr_iter(&self) -> impl Iterator<Item = Expr> + '_ {
        self.syntax.children().filter_map(Expr::cast)
    }
}

///
#[ast_node(MapLitExpr)]
#[derive(Clone, Debug)]
pub struct MapLitExpr {
    syntax: SyntaxNode,
}

impl MapLitExpr {
    /// Provides an iterator over the children `kexpr`s
    pub fn kexpr_iter(&self) -> impl Iterator<Item = KExpr> + '_ {
        self.syntax.children().filter_map(KExpr::cast)
    }
}

///
#[ast_node(StrLit)]
#[derive(Clone, Debug)]
pub struct StrLit {
    syntax: SyntaxNode,
}

///
#[ast_node(PrimaryExpr)]
#[derive(Clone, Debug)]
pub struct PrimaryExpr {
    syntax: SyntaxNode,
}

/// An unary prefix operation
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum PrefixUnaryOpType {
    /// + (does nothing)
    Plus,
    /// - (changes sign of number)
    Minus,
    /// negates boolean value
    Not,
    /// swaps bits
    BitwiseNot,
}

///
#[ast_node(PrefixUnaryOpExpr)]
#[derive(Clone, Debug)]
pub struct PrefixUnaryExpr {
    syntax: SyntaxNode,
}

impl PrefixUnaryExpr {
    /// The type of prefix unary operation
    #[must_use]
    pub fn op_type(&self) -> PrefixUnaryOpType {
        match self.syntax.first_token().unwrap().kind() {
            PLUS => PrefixUnaryOpType::Plus,
            MINUS => PrefixUnaryOpType::Minus,
            NOT_KW => PrefixUnaryOpType::Not,
            TILDE => PrefixUnaryOpType::BitwiseNot,
            _ => unreachable!(),
        }
    }

    /// The lower expression
    #[must_use]
    pub fn expr(&self) -> Expr {
        self.syntax.children().find_map(Expr::cast).unwrap()
    }
}

/// The type of infix binary operation
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InfixBinOpType {
    /// Addition
    Add,
    /// Subtraction
    Subtract,
    /// Multiplication
    Multiply,
    /// Division
    Divide,
    /// Modulo, remainder (`a % b`)
    TakeModulo,

    /// Shift left
    ShiftLeft,
    /// Shift right
    ShiftRight,

    /// Compare <
    CompareLessThan,
    /// Compare >
    CompareGreaterThan,
    /// Compare <=
    CompareLessThanOrEqualTo,
    /// Compare >=
    CompareGreaterThanOrEqualTo,
    /// Compare ==
    CompareEquals,
    /// Compare !=
    CompareNotEquals,

    /// and
    BooleanAnd,
    /// or
    BooleanOr,
}

///
#[ast_node(InfixBinOpExpr)]
#[derive(Clone, Debug)]
pub struct InfixBinaryOpExpr {
    syntax: SyntaxNode,
}

///
#[ast_node(FuncCallExpr)]
#[derive(Clone, Debug)]
pub struct FuncCallExpr {
    syntax: SyntaxNode,
}

///
#[ast_node(MethodCallExpr)]
#[derive(Clone, Debug)]
pub struct MethodCallExpr {
    syntax: SyntaxNode,
}

impl MethodCallExpr {
    /// Returns the base expression this method is called on.
    #[must_use]
    pub fn get_base(&self) -> Expr {
        self.syntax.children().find_map(Expr::cast).unwrap()
    }

    /// Returns the id corresponding to the name of the method called.
    #[must_use]
    pub fn get_method_name(&self) -> Id {
        self.syntax
            .children_with_tokens()
            .filter_map(NodeOrToken::into_token)
            .find_map(Id::cast)
            .unwrap()
    }

    /// Returns the arguments passed to this method.
    #[must_use]
    pub fn get_method_args(&self) -> FuncCallArgs {
        self.syntax.children().find_map(FuncCallArgs::cast).unwrap()
    }
}

///
#[ast_node(PropertyAccessExpr)]
#[derive(Clone, Debug)]
pub struct PropertyAccessExpr {
    syntax: SyntaxNode,
}

impl PropertyAccessExpr {
    /// Get the base expression
    #[must_use]
    pub fn get_base(&self) -> Expr {
        self.syntax.children().find_map(Expr::cast).unwrap()
    }

    /// Get the property name
    #[must_use]
    pub fn get_property_name(&self) -> Id {
        self.syntax
            .children_with_tokens()
            .filter_map(NodeOrToken::into_token)
            .find_map(Id::cast)
            .unwrap()
    }
}

///
#[ast_node(TupleExpr)]
#[derive(Clone, Debug)]
pub struct TupleExpr {
    syntax: SyntaxNode,
}

///
#[ast_node(IndexedExpr)]
#[derive(Clone, Debug)]
pub struct IndexExpr {
    syntax: SyntaxNode,
}

impl IndexExpr {
    ///
    #[must_use]
    pub fn get_base(&self) -> Expr {
        self.syntax.children().find_map(Expr::cast).unwrap()
    }
}

///
#[ast_node(IndexedExprBrackets)]
#[derive(Clone, Debug)]
pub struct IndexExprBrackets {
    syntax: SyntaxNode,
}

impl IndexExprBrackets {
    ///
    #[must_use]
    pub fn get_index_expr(&self) -> Expr {
        self.syntax.children().find_map(Expr::cast).unwrap()
    }
}

///
#[derive(Clone, Debug, FakeAstNode)]
pub enum FuncCallArg {
    ///
    #[kind(Expr)]
    Anonymous(Expr),
    ///
    #[kind(KExpr)]
    Named(KExpr),
}

impl FuncCallArg {
    ///
    #[must_use]
    pub const fn as_anonymous(&self) -> Option<&Expr> {
        match self {
            Self::Anonymous(expr) => Some(expr),
            _ => None,
        }
    }

    ///
    #[must_use]
    pub const fn as_named(&self) -> Option<&KExpr> {
        match self {
            Self::Named(kexpr) => Some(kexpr),
            _ => None,
        }
    }
}

///
#[ast_node(FuncCallArgs)]
#[derive(Clone, Debug)]
pub struct FuncCallArgs {
    syntax: SyntaxNode,
}

impl FuncCallArgs {
    /// Iterator over arguments
    pub fn args_iter(&self) -> impl Iterator<Item = FuncCallArg> + '_ {
        self.syntax.children().filter_map(FuncCallArg::cast)
    }
}

///
#[ast_node(KExpr)]
#[derive(Clone, Debug)]
pub struct KExpr {
    syntax: SyntaxNode,
}

impl KExpr {
    /// The key identifier
    #[must_use]
    pub fn id(&self) -> SyntaxToken {
        self.syntax.first_token().unwrap()
    }

    /// The `=` token
    #[must_use]
    pub fn eq_token(&self) -> SyntaxToken {
        self.syntax
            .children_with_tokens()
            .find(|it| it.is(EQ))
            .unwrap()
            .into_token()
            .unwrap()
    }

    /// The expression
    #[must_use]
    pub fn expr(&self) -> Expr {
        self.syntax.children().find_map(Expr::cast).unwrap()
    }
}

///
#[ast_node(Assignment)]
#[derive(Clone, Debug)]
pub struct Assignment {
    syntax: SyntaxNode,
}

///
#[ast_node(Declaration)]
#[derive(Clone, Debug)]
pub struct Declaration {
    syntax: SyntaxNode,
}

///
#[ast_node(If)]
#[derive(Clone, Debug)]
pub struct If {
    syntax: SyntaxNode,
}

///
#[ast_node(ElseIf)]
#[derive(Clone, Debug)]
pub struct ElseIf {
    syntax: SyntaxNode,
}

///
#[ast_node(Else)]
#[derive(Clone, Debug)]
pub struct Else {
    syntax: SyntaxNode,
}

///
#[ast_node(Conditional)]
#[derive(Clone, Debug)]
pub struct Conditional {
    syntax: SyntaxNode,
}

///
#[ast_node(Foreach)]
#[derive(Clone, Debug)]
pub struct Foreach {
    syntax: SyntaxNode,
}

///
#[ast_node(ForInExpr)]
#[derive(Clone, Debug)]
pub struct ForInExpr {
    syntax: SyntaxNode,
}

///
#[ast_node(ControlStatement)]
#[derive(Clone, Debug)]
pub struct ControlStatement {
    syntax: SyntaxNode,
}

///
#[ast_node(ExprStatement)]
#[derive(Clone, Debug)]
pub struct ExprStatement {
    syntax: SyntaxNode,
}

impl ExprStatement {
    ///
    #[must_use]
    pub fn get_expr(&self) -> Expr {
        self.syntax.children().find_map(Expr::cast).unwrap()
    }
}

///
#[ast_node(FnBody)]
#[derive(Clone, Debug)]
pub struct FnBody {
    syntax: SyntaxNode,
}

///
#[ast_node(Tuple)]
#[derive(Clone, Debug)]
pub struct Tuple {
    syntax: SyntaxNode,
}

impl Tuple {
    ///
    pub fn expr_iter(&self) -> impl Iterator<Item = Expr> + '_ {
        self.syntax.children().filter_map(Expr::cast)
    }
}

///
#[ast_node(ExprBlock)]
#[derive(Clone, Debug)]
pub struct ExprBlock {
    syntax: SyntaxNode,
}

///
#[ast_node(ConditionalBranch)]
#[derive(Clone, Debug)]
pub struct ConditionalBranch {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, FakeAstNode)]
pub enum Statement {
    ///
    #[kind(Assignment)]
    Assignment(Assignment),
    ///
    #[kind(Declaration)]
    Declaration(Declaration),
    ///
    #[kind(ExprStatement)]
    Expr(ExprStatement),
    ///
    #[kind(Conditional)]
    Conditional(Conditional),
    ///
    #[kind(Foreach)]
    Foreach(Foreach),
}

impl Statement {
    ///
    #[must_use]
    pub const fn as_assignment(&self) -> Option<&Assignment> {
        match self {
            Self::Assignment(assignment) => Some(assignment),
            _ => None,
        }
    }
    ///
    #[must_use]
    pub const fn as_declaration(&self) -> Option<&Declaration> {
        match self {
            Self::Declaration(declaration) => Some(declaration),
            _ => None,
        }
    }
    ///
    #[must_use]
    pub const fn as_expr_statement(&self) -> Option<&ExprStatement> {
        match self {
            Self::Expr(expr_statement) => Some(expr_statement),
            _ => None,
        }
    }
    ///
    #[must_use]
    pub const fn as_conditional(&self) -> Option<&Conditional> {
        match self {
            Self::Conditional(conditional) => Some(conditional),
            _ => None,
        }
    }
    ///
    #[must_use]
    pub const fn as_foreach(&self) -> Option<&Foreach> {
        match self {
            Self::Foreach(foreach) => Some(foreach),
            _ => None,
        }
    }
}

///
#[derive(Clone, Debug)]
pub enum LangItem {
    /// A statement
    Statement(Statement),
}

impl CastableFromSyntaxNode for LangItem {
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        let p: Option<Statement> = Statement::cast(syntax);
        if let Some(statement) = p {
            return Some(Self::Statement(statement));
        }

        None
    }
}

impl FakeAstNode for LangItem {}

impl LangItem {
    /// Casts to statement and returns None if it isn't
    #[must_use]
    pub const fn as_statement(&self) -> Option<&Statement> {
        match self {
            Self::Statement(statement) => Some(statement),
            _ => None,
        }
    }
}

///
#[ast_node(ROOT)]
#[derive(Clone, Debug)]
pub struct Root {
    syntax: SyntaxNode,
}

impl Root {
    ///
    pub fn lang_items(&self) -> impl Iterator<Item = LangItem> + '_ {
        self.syntax.children().filter_map(LangItem::cast)
    }
}

///
pub fn print(indent: usize, element: SyntaxElement) {
    let kind = element.kind();
    print!("{:indent$}", "", indent = indent);
    match element {
        NodeOrToken::Node(node) => {
            println!("- {:?} {:?}", kind, node.text_range());
            for child in node.children_with_tokens() {
                print(indent + 2, child);
            }
        }

        NodeOrToken::Token(token) => {
            println!("- {:?} {:?} {:?}", token.text(), kind, token.text_range())
        }
    }
}

#[cfg(test)]
pub fn test_dbg(indent: usize, element: SyntaxElement, s: &mut String) {
    let kind = element.kind();
    *s += &format!("{:indent$}", "", indent = indent);
    match element {
        NodeOrToken::Node(node) => {
            *s += &format!("- {:?} {:?}\n", kind, node.text_range());
            for child in node.children_with_tokens() {
                test_dbg(indent + 2, child, s);
            }
        }

        NodeOrToken::Token(token) => {
            *s += &format!("- {:?} {:?} {:?}\n", token.text(), kind, token.text_range())
        }
    }
}
