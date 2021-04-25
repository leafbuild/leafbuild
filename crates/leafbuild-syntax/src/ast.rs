//! Syntax tree types
#![allow(clippy::missing_panics_doc)]
use std::marker::PhantomData;

use crate::syntax_kind::SyntaxKind;
use crate::LeafbuildLanguage;
use rowan::{NodeOrToken, SyntaxNodeChildren};
///
pub type SyntaxNode = rowan::SyntaxNode<LeafbuildLanguage>;
///
pub type SyntaxToken = rowan::SyntaxToken<LeafbuildLanguage>;
///
pub type SyntaxElement = NodeOrToken<SyntaxNode, SyntaxToken>;

/// The main trait to go from untyped `SyntaxNode`  to a typed ast. The
/// conversion itself has zero runtime cost: ast and syntax nodes have exactly
/// the same representation: a pointer to the tree root and a pointer to the
/// node itself.
pub trait AstNode {
    /// Returns true if it can cast a given kind to this type of node.
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    /// Tries to cast the given untyped node to this type of node.
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    /// Gets the underlying syntax node.
    fn syntax(&self) -> &SyntaxNode;
    /// Clones this node for update
    fn clone_for_update(&self) -> Self
    where
        Self: Sized,
    {
        Self::cast(self.syntax().clone_for_update()).unwrap()
    }
}

/// Like `AstNode`, but wraps tokens rather than interior nodes.
pub trait AstToken {
    /// Returns `true` if it can cast a given kind to this type of token.
    fn can_cast(token: SyntaxKind) -> bool
    where
        Self: Sized;

    /// Tries to cast the given untyped token to this type of token.
    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized;

    /// Gets the underlying token.
    fn syntax(&self) -> &SyntaxToken;

    /// Returns the text of the token.
    fn text(&self) -> &str {
        self.syntax().text()
    }
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

/// An iterator over `SyntaxNode` children of a particular AST type.
#[derive(Debug, Clone)]
pub struct AstChildren<N> {
    inner: SyntaxNodeChildren<LeafbuildLanguage>,
    ph: PhantomData<N>,
}

impl<N> AstChildren<N> {
    fn new(parent: &SyntaxNode) -> Self {
        AstChildren {
            inner: parent.children(),
            ph: PhantomData,
        }
    }
}

impl<N: AstNode> Iterator for AstChildren<N> {
    type Item = N;
    fn next(&mut self) -> Option<N> {
        self.inner.find_map(N::cast)
    }
}

mod expr_ext;
mod stmt_ext;
mod support;

mod implnew;
pub use implnew::*;

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
