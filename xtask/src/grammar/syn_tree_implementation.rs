use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::Ident;

use super::{Child, ElementType, Grammar, Node};
use log::error;
pub fn syn_tree_implementation(grammar: &Grammar) -> TokenStream {
    let mut stream = quote! {};

    fn mark_section(stream: &mut TokenStream, section: &str) {
        let ts = format!("//\n// {}\n//", section)
            .parse::<TokenStream>()
            .unwrap();
        stream.append_all(ts);
    }

    stream.append_all(quote! {
        #![allow(missing_docs)]
        use super::{
            AstNode, AstToken, SyntaxKind, SyntaxNode, SyntaxToken,
        };
    });

    mark_section(&mut stream, "Const tokens");
    grammar
        .const_tokens
        .iter()
        .for_each(|it| append_for_token(&mut stream, &it.name, &it.kind));

    mark_section(&mut stream, "Tokens");
    grammar
        .tokens
        .iter()
        .for_each(|it| append_for_token(&mut stream, &it.name, &it.kind));

    mark_section(&mut stream, "Nodes");
    grammar
        .nodes
        .iter()
        .for_each(|node| append_for_node(&mut stream, grammar, node));
    stream
}

fn append_for_token(stream: &mut TokenStream, name: &str, kind: &str) {
    let name = Ident::new(name, Span::call_site());
    let kind = Ident::new(kind, Span::call_site());

    stream.append_all(quote! {
        #[derive(Clone, Debug)]
        #[repr(transparent)]
        pub struct #name {
            syntax: SyntaxToken,
        }

        impl AstToken for #name {
            const KIND: SyntaxKind = SyntaxKind::#kind;

            #[allow(unsafe_code)]
            unsafe fn new(syntax: SyntaxToken) -> Self {
                Self { syntax }
            }

            fn get_text(&self) -> &str {
                self.syntax.text()
            }
        }
    });
}

fn append_for_node(stream: &mut TokenStream, grammar: &Grammar, node: &Node) {
    let name = Ident::new(&node.name, Span::call_site());
    let kind = Ident::new(&node.kind, Span::call_site());
    let children = Children(grammar, &node.children, &node.name);

    stream.append_all(quote! {
        #[derive(Debug, Clone)]
        pub struct #name {
            syntax: SyntaxNode,
        }

        impl AstNode for #name {
            const KIND: SyntaxKind = SyntaxKind::#kind;

            #[allow(unsafe_code)]
            unsafe fn new(syntax: SyntaxNode) -> Self {
                Self { syntax }
            }

            fn syntax(&self) -> &SyntaxNode {
                &self.syntax
            }
        }

        impl #name {
            #children
        }
    })
}

struct Children<'a>(&'a Grammar, &'a [Child], &'a str);

impl<'a> ToTokens for Children<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.1.iter().for_each(|child| {
            let name = child.get_sytax_element_name();
            let name_ident = Ident::new(name, Span::call_site());
            let element_type = self.0.element_type(name);
            let is_tok = match element_type {
                Some(ElementType::Token) => true,
                Some(ElementType::Node) => false,
                None => {
                    error!(
                        "Cannot find child element of name '{}', in '{}'",
                        name, self.2,
                    );
                    panic!("Unknown reference")
                }
            };

            let (fn_name, return_type, helper_function) = match child {
                Child::Single(_, fn_name_fragment) => (
                    format!("get_{}", fn_name_fragment),
                    quote! {#name_ident},
                    "get_single",
                ),
                Child::Multiple(_, fn_name_fragment) => (
                    format!("get_{}_iter", fn_name_fragment),
                    quote! {impl Iterator<Item=#name_ident>},
                    "get_multiple",
                ),
                Child::First(_, fn_name_fragment) => (
                    format!("get_{}", fn_name_fragment),
                    quote! {#name_ident},
                    "get_first",
                ),
                Child::Last(_, fn_name_fragment) => (
                    format!("get_{}", fn_name_fragment),
                    quote! {#name_ident},
                    "get_last",
                ),
                Child::Optional(_, fn_name_fragment) => (
                    format!("get_{}_opt", fn_name_fragment),
                    quote! {Option<#name_ident>},
                    "get_opt",
                ),
            };

            let fn_name = Ident::new(&fn_name, Span::call_site());
            let helper_function = if is_tok {
                Ident::new(&format!("{}_tok", helper_function), Span::call_site())
            } else {
                Ident::new(helper_function, Span::call_site())
            };

            tokens.append_all(quote! {
                pub fn #fn_name (&self) -> #return_type {
                    super::#helper_function(&self.syntax)
                }
            })
        });
    }
}
