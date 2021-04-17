use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::Ident;

use super::Grammar;

pub fn get_kinds(grammar: &Grammar) -> TokenStream {
    struct SKind<'a>(&'a str);
    struct SKindTokenNameMatch<'a>(Vec<(SKind<'a>, &'a str)>);

    impl<'a> ToTokens for SKind<'a> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let name = Ident::new(self.0, Span::call_site());
            tokens.append_all(quote! {#name, })
        }
    }

    impl<'a> ToTokens for SKindTokenNameMatch<'a> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(
                self.0
                    .iter()
                    .map(|it| {
                        let name = Ident::new(it.0 .0, Span::call_site());
                        let token_name = it.1;
                        quote! {#name => #token_name, }
                    })
                    .flatten(),
            )
        }
    }

    let kinds = grammar
        .const_tokens
        .iter()
        .map(|it| it.kind.as_str())
        .chain(grammar.tokens.iter().map(|it| it.kind.as_str()))
        .chain(grammar.nodes.iter().map(|it| it.kind.as_str()))
        .map(SKind);

    let kinds_token_name_match = SKindTokenNameMatch(
        grammar
            .const_tokens
            .iter()
            .map(|it| (SKind(it.kind.as_str()), it.text.as_str()))
            .chain(
                grammar
                    .tokens
                    .iter()
                    .map(|it| (SKind(it.kind.as_str()), it.name.as_str())),
            )
            .collect(),
    );

    struct TokenMacroMatches<'a>(Vec<(SKind<'a>, &'a str)>);

    impl<'a> ToTokens for TokenMacroMatches<'a> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(
                self.0
                    .iter()
                    .map(|it| {
                        let matcher: TokenStream = if it.1.chars().any(|it| "([{)]}".contains(it)) {
                            format!(r#"'{}'"#, it.1).parse().unwrap()
                        } else if it.1 == "\n" {
                            r#"'\n'"#.parse().unwrap()
                        } else {
                            it.1.parse().unwrap()
                        };
                        let kind = Ident::new(&it.0 .0, Span::call_site());
                        quote! {[#matcher] => { $crate::SyntaxKind:: #kind };}
                    })
                    .flatten(),
            )
        }
    }

    let token_macro_matches = TokenMacroMatches(
        grammar
            .const_tokens
            .iter()
            .map(|it| (SKind(it.kind.as_str()), it.text.as_str()))
            .chain(
                grammar
                    .tokens
                    .iter()
                    .map(|it| (SKind(it.kind.as_str()), it.name.as_str())),
            )
            .collect(),
    );

    quote! {
        #![allow(missing_docs)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
        #[repr(u16)]
        pub enum SyntaxKind {
            ERROR,
            EOF,
            TOMBSTONE,
            #(#kinds)*
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
            pub(crate) fn token_name(&self) -> &'static str {
                use SyntaxKind::*;
                match self {
                    #kinds_token_name_match
                    _ => "",
                }
            }
        }

        #[macro_export]
        #[doc = "A nice way of getting [`SyntaxKind`]s from tokens in rust itself."]
        macro_rules! T {
            #token_macro_matches
        }
    }
}
