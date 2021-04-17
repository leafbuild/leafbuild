use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::Ident;

use super::{ConstToken, Token};
#[derive(Debug)]
pub struct Lexer<'a> {
    pub tokens: &'a [Token],
    pub const_tokens: &'a [ConstToken],
}

impl<'a> ToTokens for Lexer<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        impl ToTokens for Token {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                let Token {
                    ref name,
                    ref regex,
                    ..
                } = self;

                let name_ident = Ident::new(&name, Span::call_site());
                let regex = super::raw_str_literal(regex);

                tokens.append_all(quote! {
                    #[regex(#regex)]
                    #name_ident,
                });
            }
        }

        impl ToTokens for ConstToken {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                let ConstToken {
                    ref name, ref text, ..
                } = self;
                let name_ident = Ident::new(name, Span::call_site());

                tokens.append_all(quote! {
                    #[token(#text)]
                    #name_ident,
                })
            }
        }

        let Lexer {
            tokens: token_list,
            const_tokens,
        } = self;

        let tokens_match = MatchTokens(token_list);
        let const_tokens_match = MatchConstTokens(const_tokens);

        tokens.append_all(quote! {
            use crate::parser::Span;
            use crate::syntax_kind::SyntaxKind;
            use crate::T;
            use leafbuild_stdx::Let;
            use logos::Logos;
            use std::convert::TryInto;
            use std::ops::Range;

            #[derive(Logos, Copy, Clone, Debug, PartialEq, PartialOrd, Eq)]
            pub enum Tk {
                #(#token_list)*
                #(#const_tokens)*

                #[error]
                // fix for https://github.com/maciejhirsz/logos/issues/180
                #[regex(r"/\*([^*]|\*+[^*/])*\*?")]
                Error,
            }

            impl From<Tk> for SyntaxKind {
                fn from(tk: Tk) -> Self {
                    use SyntaxKind::*;
                    match tk {
                        #tokens_match
                        #const_tokens_match
                        Tk::Error => ERROR,
                    }
                }
            }

            #[allow(missing_debug_implementations)]
            pub struct Lexer<'a> {
                lexer: logos::SpannedIter<'a, Tk>,
            }
            impl<'a> Lexer<'a> {
                pub(crate) fn new(s: &'a str) -> Self {
                    let lexer = Tk::lexer(s).spanned();
                    Self { lexer }
                }
            }
            impl<'a> Iterator for Lexer<'a> {
                type Item = (SyntaxKind, Span);
                fn next(&mut self) -> Option<Self::Item> {
                    self.lexer.next().map(|(token, span)| {
                        (
                            token.into(),
                            span.let_(|it| -> Range<u32> {
                                it.start.try_into().unwrap()..it.end.try_into().unwrap()
                            })
                            .into(),
                        )
                    })
                }
            }

        });
    }
}

struct MatchTokens<'a>(&'a [Token]);
impl<'a> ToTokens for MatchTokens<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(
            self.0
                .iter()
                .map(|it| {
                    let name = Ident::new(&it.name, Span::call_site());
                    quote! {Tk::#name => T![#name],}
                })
                .flatten(),
        )
    }
}
struct MatchConstTokens<'a>(&'a [ConstToken]);

impl<'a> ToTokens for MatchConstTokens<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(
            self.0
                .iter()
                .map(|it| {
                    let name = Ident::new(&it.name, Span::call_site());
                    let matcher: TokenStream = if it.text.chars().any(|it| "([{)]}".contains(it)) {
                        format!(r#"'{}'"#, it.text).parse().unwrap()
                    } else if it.text == "\n" {
                        r#"'\n'"#.parse().unwrap()
                    } else {
                        it.text.parse().unwrap()
                    };
                    quote! {Tk::#name => T![#matcher],}
                })
                .flatten(),
        )
    }
}
