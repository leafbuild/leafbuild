use proc_macro2::{Punct, Spacing, TokenStream};
use quote::{format_ident, quote};

use super::to_upper_snake_case;

pub(crate) fn generate_kinds(grammar: super::ast_src::KindsSrc) -> TokenStream {
    let (single_byte_tokens_values, single_byte_tokens): (Vec<_>, Vec<_>) = grammar
        .punct
        .iter()
        .filter(|(token, _name)| token.len() == 1)
        .map(|(token, name)| (token.chars().next().unwrap(), format_ident!("{}", name)))
        .unzip();

    let punctuation_values = grammar.punct.iter().map(|(token, _name)| {
        if "{}[]()".contains(token) {
            let c = token.chars().next().unwrap();
            quote! { #c }
        } else {
            let cs = token.chars().map(|c| Punct::new(c, Spacing::Joint));
            quote! { #(#cs)* }
        }
    });
    let punctuation = grammar
        .punct
        .iter()
        .map(|(_token, name)| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let full_keywords_values = &grammar.keywords;
    let full_keywords = full_keywords_values
        .iter()
        .map(|kw| format_ident!("{}_KW", to_upper_snake_case(&kw)));

    let all_keywords_values = grammar
        .keywords
        .iter()
        .chain(grammar.contextual_keywords.iter())
        .collect::<Vec<_>>();
    let all_keywords_idents = all_keywords_values.iter().map(|kw| format_ident!("{}", kw));
    let all_keywords = all_keywords_values
        .iter()
        .map(|name| format_ident!("{}_KW", to_upper_snake_case(&name)))
        .collect::<Vec<_>>();

    let literals = grammar
        .literals
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let tokens = grammar
        .tokens
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let nodes = grammar
        .nodes
        .iter()
        .map(|name| format_ident!("{}", name))
        .collect::<Vec<_>>();

    let ast = quote! {
        #![allow(bad_style, missing_docs, unreachable_pub)]
        /// The kind of syntax node, e.g. `IDENT`, `USE_KW`, or `STRUCT`.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        #[repr(u16)]
        pub enum SyntaxKind {
            // Technical SyntaxKinds: they appear temporally during parsing,
            // but never end up in the final tree
            #[doc(hidden)]
            TOMBSTONE,
            #[doc(hidden)]
            EOF,
            #(#punctuation,)*
            #(#all_keywords,)*
            #(#literals,)*
            #(#tokens,)*
            #(#nodes,)*

            // Technical kind so that we can cast from u16 safely
            #[doc(hidden)]
            __LAST,
        }
        use self::SyntaxKind::*;

        impl SyntaxKind {
            pub fn is_keyword(self) -> bool {
                match self {
                    #(#all_keywords)|* => true,
                    _ => false,
                }
            }

            pub fn is_punct(self) -> bool {
                match self {
                    #(#punctuation)|* => true,
                    _ => false,
                }
            }

            pub fn is_literal(self) -> bool {
                match self {
                    #(#literals)|* => true,
                    _ => false,
                }
            }

            pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
                let kw = match ident {
                    #(#full_keywords_values => #full_keywords,)*
                    _ => return None,
                };
                Some(kw)
            }

            pub fn from_char(c: char) -> Option<SyntaxKind> {
                let tok = match c {
                    #(#single_byte_tokens_values => #single_byte_tokens,)*
                    _ => return None,
                };
                Some(tok)
            }
        }

        #[macro_export]
        #[doc = "A nice way of getting syntax kinds from tokens"]
        macro_rules! T {
            #([#punctuation_values] => { $crate::SyntaxKind::#punctuation };)*
            #([#all_keywords_idents] => { $crate::SyntaxKind::#all_keywords };)*
            [ident] => { $crate::SyntaxKind::IDENT };
            [newline] => { $crate::SyntaxKind::NEWLINE };
            [int_number] => { $crate::SyntaxKind::INT_NUMBER };
            [float_number] => { $crate::SyntaxKind::FLOAT_NUMBER };
            [str] => { $crate::SyntaxKind::STR };
            [multiline_str] => { $crate::SyntaxKind::MULTILINE_STR };
            [comment] => { $crate::SyntaxKind::COMMENT };
            [block_comment] => { $crate::SyntaxKind::BLOCK_COMMENT };
            [whitespace] => { $crate::SyntaxKind::WHITESPACE };
            [not] =>  { $crate::SyntaxKind::NOT_KW };
            [shebang] => { $crate::SyntaxKind::SHEBANG };
        }
    };

    ast
}
