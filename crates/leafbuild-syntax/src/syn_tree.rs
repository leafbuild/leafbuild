/*
 *   Copyright (c) 2021 Dinu Blanovschi
 *   All rights reserved.
 *   Licensed under the terms of the BSD-3 Clause license, see LICENSE for more.
 */
//! Syntax tree types
#![allow(clippy::missing_panics_doc)]
use crate::parser::Is;
use crate::syntax_kind::SyntaxKind::{self, *};
use crate::LeafbuildLanguage;
use leafbuild_derive::{ast_token, AstNode};
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
pub trait NewSynTreeElem: Sized {
    ///
    #[allow(unsafe_code)]
    unsafe fn new(syntax: SyntaxNode) -> Self;
}

///
pub trait CastableFromSyntaxNode: NewSynTreeElem {
    ///
    const KIND: SyntaxKind = ERROR;
    ///
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind() {
            kind if kind == Self::KIND => Some(
                #[allow(unsafe_code)]
                unsafe {
                    Self::new(syntax)
                },
            ),
            _ => None,
        }
    }
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
#[derive(Clone, Debug, AstNode)]
#[kind(Expr)]
pub struct Expr {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, AstNode)]
#[kind(ArrayLitExpr)]
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
#[derive(Clone, Debug, AstNode)]
#[kind(MapLitExpr)]
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
#[derive(Clone, Debug, AstNode)]
#[kind(StrLit)]
pub struct StrLit {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, AstNode)]
#[kind(PrimaryExpr)]
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
}

///
#[derive(Clone, Debug, AstNode)]
#[kind(PrefixUnaryOpExpr)]
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
#[derive(Clone, Debug, AstNode)]
#[kind(InfixBinOpExpr)]
pub struct InfixBinaryOpExpr {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, AstNode)]
#[kind(FuncCallExpr)]
pub struct FuncCallExpr {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, AstNode)]
#[kind(MethodCallExpr)]
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
#[derive(Clone, Debug, AstNode)]
#[kind(PropertyAccessExpr)]
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
#[derive(Clone, Debug, AstNode)]
#[kind(TupleExpr)]
pub struct TupleExpr {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, AstNode)]
#[kind(IndexedExpr)]
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
#[derive(Clone, Debug, AstNode)]
#[kind(IndexedExprBrackets)]
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
#[derive(Clone, Debug, AstNode)]
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
#[derive(Clone, Debug, AstNode)]
#[kind(FuncCallArgs)]
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
#[derive(Clone, Debug, AstNode)]
#[kind(KExpr)]
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
#[derive(Clone, Debug, AstNode)]
#[kind(Assignment)]
pub struct Assignment {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, AstNode)]
#[kind(Declaration)]
pub struct Declaration {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, AstNode)]
#[kind(If)]
pub struct If {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, AstNode)]
#[kind(ElseIf)]
pub struct ElseIf {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, AstNode)]
#[kind(Else)]
pub struct Else {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, AstNode)]
#[kind(Conditional)]
pub struct Conditional {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, AstNode)]
#[kind(Foreach)]
pub struct Foreach {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, AstNode)]
#[kind(ForInExpr)]
pub struct ForInExpr {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, AstNode)]
#[kind(ControlStatement)]
pub struct ControlStatement {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, AstNode)]
#[kind(ExprStatement)]
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
#[derive(Clone, Debug, AstNode)]
#[kind(FnBody)]
pub struct FnBody {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, AstNode)]
#[kind(Tuple)]
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
#[derive(Clone, Debug, AstNode)]
#[kind(ExprBlock)]
pub struct ExprBlock {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, AstNode)]
#[kind(ConditionalBranch)]
pub struct ConditionalBranch {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, AstNode)]
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
#[derive(Clone, Debug, AstNode)]
#[kind(StructDecl)]
pub struct StructDecl {
    syntax: SyntaxNode,
}

///
#[derive(Clone, Debug, AstNode)]
pub enum LangItem {
    /// A statement
    #[kind(Statement)]
    Statement(Statement),
    /// A struct declaration
    #[kind(StructDecl)]
    StructDecl(StructDecl),
}

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
#[derive(Clone, Debug, AstNode)]
#[kind(ROOT)]
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
