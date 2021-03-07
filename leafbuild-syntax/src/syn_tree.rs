//! Syntax tree types
use crate::parser::Is;
use crate::syntax_kind::SyntaxKind::{self, *};
use crate::LeafbuildLanguage;
use leafbuild_core::utils::*;
use leafbuild_derive::ast_node;
use leafbuild_derive::FakeAstNode;
use rowan::NodeOrToken;

type SyntaxNode = rowan::SyntaxNode<LeafbuildLanguage>;
type SyntaxToken = rowan::SyntaxToken<LeafbuildLanguage>;
type SyntaxElement = NodeOrToken<SyntaxNode, SyntaxToken>;

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
    fn syntax(&self) -> &SyntaxNode;
}

///
pub trait FakeAstNode: CastableFromSyntaxNode {}

///
pub trait CastTo<T>: Sized {
    ///
    fn cast_to(self) -> Option<T>;
}

impl<T> CastTo<T> for SyntaxNode
where
    T: CastableFromSyntaxNode,
{
    fn cast_to(self) -> Option<T> {
        T::cast(self)
    }
}

///
#[ast_node(Expr)]
#[derive(Debug)]
pub struct Expr {
    syntax: SyntaxNode,
}

///
#[ast_node(ArrayLitExpr)]
#[derive(Debug)]
pub struct ArrayLitExpr {
    syntax: SyntaxNode,
}

///
#[ast_node(MapLitExpr)]
#[derive(Debug)]
pub struct MapLitExpr {
    syntax: SyntaxNode,
}

///
#[ast_node(StrLit)]
#[derive(Debug)]
pub struct StrLit {
    syntax: SyntaxNode,
}

///
#[ast_node(PrimaryExpr)]
#[derive(Debug)]
pub struct PrimaryExpr {
    syntax: SyntaxNode,
}

///
#[ast_node(PrefixUnaryOpExpr)]
#[derive(Debug)]
pub struct PrefixUnaryExpr {
    syntax: SyntaxNode,
}

///
#[ast_node(InfixBinOpExpr)]
#[derive(Debug)]
pub struct InfixBinaryOpExpr {
    syntax: SyntaxNode,
}

///
#[ast_node(FuncCallExpr)]
#[derive(Debug)]
pub struct FuncCallExpr {
    syntax: SyntaxNode,
}

///
#[ast_node(MethodCallExpr)]
#[derive(Debug)]
pub struct MethodCallExpr {
    syntax: SyntaxNode,
}

///
#[ast_node(PropertyAccessExpr)]
#[derive(Debug)]
pub struct PropertyAccessExpr {
    syntax: SyntaxNode,
}

///
#[ast_node(TupleExpr)]
#[derive(Debug)]
pub struct TupleExpr {
    syntax: SyntaxNode,
}

///
#[ast_node(IndexedExpr)]
#[derive(Debug)]
pub struct IndexExpr {
    syntax: SyntaxNode,
}

///
#[ast_node(IndexedExprBraces)]
#[derive(Debug)]
pub struct IndexExprBraces {
    syntax: SyntaxNode,
}

///
#[derive(Debug, FakeAstNode)]
pub enum FuncCallArg {
    ///
    #[kind(Expr)]
    Anonymous(Expr),
    ///
    #[kind(KExpr)]
    Named(KExpr),
}

///
#[ast_node(FuncCallArgs)]
#[derive(Debug)]
pub struct FuncCallArgs {
    syntax: SyntaxNode,
}

impl FuncCallArgs {
    fn args(&self) -> impl Iterator<Item = FuncCallArg> {
        self.syntax.children().filter_map(|it| it.cast_to())
    }
}

///
#[ast_node(KExpr)]
#[derive(Debug)]
pub struct KExpr {
    syntax: SyntaxNode,
}

impl KExpr {
    fn id(&self) -> SyntaxToken {
        self.syntax.first_token().unwrap()
    }

    fn eq_token(&self) -> SyntaxToken {
        self.syntax
            .children_with_tokens()
            .find(|it| it.is(EQ))
            .unwrap()
            .into_token()
            .unwrap()
    }

    fn expr(&self) -> Expr {
        self.syntax.children().find_map(|it| it.cast_to()).unwrap()
    }
}

///
#[ast_node(PositionalArg)]
#[derive(Debug)]
pub struct PositionalArg {
    syntax: SyntaxNode,
}

///
#[ast_node(Assignment)]
#[derive(Debug)]
pub struct Assignment {
    syntax: SyntaxNode,
}

///
#[ast_node(Declaration)]
#[derive(Debug)]
pub struct Declaration {
    syntax: SyntaxNode,
}

///
#[ast_node(If)]
#[derive(Debug)]
pub struct If {
    syntax: SyntaxNode,
}

///
#[ast_node(ElseIf)]
#[derive(Debug)]
pub struct ElseIf {
    syntax: SyntaxNode,
}

///
#[ast_node(Else)]
#[derive(Debug)]
pub struct Else {
    syntax: SyntaxNode,
}

///
#[ast_node(Conditional)]
#[derive(Debug)]
pub struct Conditional {
    syntax: SyntaxNode,
}

///
#[ast_node(Foreach)]
#[derive(Debug)]
pub struct Foreach {
    syntax: SyntaxNode,
}

///
#[ast_node(ForInExpr)]
#[derive(Debug)]
pub struct ForInExpr {
    syntax: SyntaxNode,
}

///
#[ast_node(ControlStatement)]
#[derive(Debug)]
pub struct ControlStatement {
    syntax: SyntaxNode,
}

///
#[ast_node(ExprStatement)]
#[derive(Debug)]
pub struct ExprStatement {
    syntax: SyntaxNode,
}

///
#[ast_node(FnBody)]
#[derive(Debug)]
pub struct FnBody {
    syntax: SyntaxNode,
}

///
#[ast_node(Tuple)]
#[derive(Debug)]
pub struct Tuple {
    syntax: SyntaxNode,
}

///
#[ast_node(ExprBlock)]
#[derive(Debug)]
pub struct ExprBlock {
    syntax: SyntaxNode,
}

///
#[ast_node(ConditionalBranch)]
#[derive(Debug)]
pub struct ConditionalBranch {
    syntax: SyntaxNode,
}

///
#[ast_node(Statement)]
#[derive(Debug)]
pub struct Statement {
    syntax: SyntaxNode,
}

///
#[ast_node(LangItem)]
#[derive(Debug)]
pub struct LangItem {
    syntax: SyntaxNode,
}

impl LangItem {
    fn as_statement(&self) -> Option<Statement> {
        self.syntax.first_child().unwrap().let_(Statement::cast)
    }
}

///
#[ast_node(ROOT)]
#[derive(Debug)]
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
