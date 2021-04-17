use super::Grammar;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::Ident;

pub fn visitor(grammar: &Grammar) -> TokenStream {
    let visitor_methods_default_impls = VisitorMethodsDefaultImpls(grammar);
    let visitor_token_methods_match = VisitorTokenMethodsMatch(grammar);
    let visitor_node_methods_match = VisitorNodeMethodsMatch(grammar);

    quote! {
        use super::{
            implementation::*, AstNode, AstToken, SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken,
        };
        #[doc = "The visitor trait"]
        pub trait Visitor {
            type Output;
            type Sink;
            #visitor_methods_default_impls

            #[allow(unsafe_code)]
            fn visit(&self, node: SyntaxNode) -> Self::Output {
                match node.kind() {
                    #visitor_node_methods_match
                    _ => unreachable!(),
                }
            }

            #[allow(unsafe_code)]
            fn visit_token(&self, token: SyntaxToken) -> Self::Output {
                match token.kind() {
                    #visitor_token_methods_match
                    _ => unreachable!(),
                }
            }

            #[allow(unsafe_code)]
            fn visit_element(&self, element: SyntaxElement) -> Self::Output {
                match element {
                    SyntaxElement::Node(node) => self.visit(node),
                    SyntaxElement::Token(token) => self.visit_token(token),
                }
            }
        }
    }
}

struct VisitorMethodsDefaultImpls<'a>(&'a Grammar);

impl<'a> ToTokens for VisitorMethodsDefaultImpls<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for token in self.0.tokens.iter() {
            let visit_name = Ident::new(
                &format!("visit_{}", &token.lowercased_name),
                Span::call_site(),
            );
            let token_type_name = Ident::new(&token.name, Span::call_site());
            let token_name = Ident::new(&token.lowercased_name, Span::call_site());
            let docs = format!("Visits a {} token", &token.name);
            tokens.append_all(quote! {
                #[doc = #docs]
                fn #visit_name (&self, #token_name: #token_type_name) -> Self::Output;
            });
        }

        for token in self.0.const_tokens.iter() {
            let visit_name = Ident::new(
                &format!("visit_{}", &token.lowercased_name),
                Span::call_site(),
            );
            let token_type_name = Ident::new(&token.name, Span::call_site());
            let token_name = Ident::new(&token.lowercased_name, Span::call_site());
            let docs = format!("Visits a {} token", &token.name);
            tokens.append_all(quote! {
                #[doc = #docs]
                fn #visit_name (&self, #token_name: #token_type_name) -> Self::Output;
            });
        }

        for node in self.0.nodes.iter() {
            let visit_name = Ident::new(
                &format!("visit_{}", &node.lowercased_name),
                Span::call_site(),
            );
            let node_type_name = Ident::new(&node.name, Span::call_site());
            let node_name = Ident::new(&node.lowercased_name, Span::call_site());

            let docs = format!("Visits a {} node", &node.name);

            tokens.append_all(quote! {
                #[doc = #docs]
                fn #visit_name (&self, #node_name: #node_type_name) -> Self::Output;
            });
        }
    }
}

struct VisitorTokenMethodsMatch<'a>(&'a Grammar);

impl<'a> ToTokens for VisitorTokenMethodsMatch<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for token in self.0.tokens.iter() {
            let visit_name = Ident::new(
                &format!("visit_{}", &token.lowercased_name),
                Span::call_site(),
            );
            let token_type_name = Ident::new(&token.name, Span::call_site());
            let kind = Ident::new(&token.kind, Span::call_site());
            tokens.append_all(quote! {
                SyntaxKind::#kind => self.#visit_name(unsafe {#token_type_name::new(token)}),
            });
        }

        for token in self.0.const_tokens.iter() {
            let visit_name = Ident::new(
                &format!("visit_{}", &token.lowercased_name),
                Span::call_site(),
            );
            let token_type_name = Ident::new(&token.name, Span::call_site());
            let kind = Ident::new(&token.kind, Span::call_site());
            tokens.append_all(quote! {
                SyntaxKind::#kind => self.#visit_name(unsafe {#token_type_name::new(token)}),
            });
        }
    }
}

struct VisitorNodeMethodsMatch<'a>(&'a Grammar);

impl<'a> ToTokens for VisitorNodeMethodsMatch<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for node in self.0.nodes.iter() {
            let visit_name = Ident::new(
                &format!("visit_{}", &node.lowercased_name),
                Span::call_site(),
            );
            let node_type_name = Ident::new(&node.name, Span::call_site());
            let kind = Ident::new(&node.kind, Span::call_site());
            tokens.append_all(quote! {
                SyntaxKind::#kind => self.#visit_name(unsafe {#node_type_name::new(node)}),
            });
        }
    }
}
