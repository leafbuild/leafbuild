//! Syntax tree types
#![allow(clippy::missing_panics_doc)]
use crate::parser::Is;
use crate::syntax_kind::SyntaxKind::{self};
use crate::LeafbuildLanguage;
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
pub trait AstNode {
    /// The `SyntaxKind` of this node.
    const KIND: SyntaxKind;
    /// Creates a node of type `Self`
    /// with the given `SyntaxNode`,
    /// without performing any checks.
    #[allow(unsafe_code)]
    unsafe fn new(syntax: SyntaxNode) -> Self;
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
    ///
    // #[deprecated = "use accessors instead of providing access to the underlying syntax node"]
    fn syntax(&self) -> &SyntaxNode;
}

///
pub trait AstToken {
    const KIND: SyntaxKind;

    #[allow(unsafe_code)]
    unsafe fn new(syntax: SyntaxToken) -> Self;

    ///
    fn cast(syntax: SyntaxToken) -> Option<Self>
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

    ///
    fn get_text(&self) -> &str;
}

///
pub trait CastTo: Sized {
    ///
    fn cast_to<T>(self) -> Option<T>
    where
        T: AstNode;
}

impl CastTo for SyntaxNode {
    fn cast_to<T>(self) -> Option<T>
    where
        T: AstNode,
    {
        T::cast(self)
    }
}

///
pub trait CastToToken: Sized {
    ///
    fn cast_to_token<T>(self) -> Option<T>
    where
        T: AstToken;
}

impl CastToToken for SyntaxToken {
    fn cast_to_token<T>(self) -> Option<T>
    where
        T: AstToken,
    {
        T::cast(self)
    }
}

mod implementation;
pub use implementation::*;

fn get_single_tok<T: AstToken>(syntax: &SyntaxNode) -> T {
    get_first_tok(syntax)
}

fn get_single<T: AstNode>(syntax: &SyntaxNode) -> T {
    get_first(syntax)
}

fn get_multiple_tok<T: AstToken>(syntax: &SyntaxNode) -> impl Iterator<Item = T> {
    syntax
        .children_with_tokens()
        .filter_map(|it| it.into_token())
        .filter_map(T::cast)
}

fn get_multiple<T: AstNode>(syntax: &SyntaxNode) -> impl Iterator<Item = T> {
    syntax.children().filter_map(T::cast)
}

fn get_first_tok<T: AstToken>(syntax: &SyntaxNode) -> T {
    get_opt_tok(syntax).unwrap()
}

fn get_first<T: AstNode>(syntax: &SyntaxNode) -> T {
    get_opt(syntax).unwrap()
}

fn get_last_tok<T: AstToken>(syntax: &SyntaxNode) -> T {
    syntax
        .children_with_tokens()
        .filter_map(|it| it.into_token())
        .filter_map(T::cast)
        .last()
        .unwrap()
}

fn get_last<T: AstNode>(syntax: &SyntaxNode) -> T {
    syntax.children().filter_map(T::cast).last().unwrap()
}

fn get_opt_tok<T: AstToken>(syntax: &SyntaxNode) -> Option<T> {
    syntax
        .children_with_tokens()
        .filter_map(|it| it.into_token())
        .find_map(T::cast)
}

fn get_opt<T: AstNode>(syntax: &SyntaxNode) -> Option<T> {
    syntax.children().find_map(T::cast)
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
