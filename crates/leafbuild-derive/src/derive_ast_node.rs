/*
 *   Copyright (c) 2021 Dinu Blanovschi
 *   All rights reserved.
 *   Licensed under the terms of the BSD-3 Clause license, see LICENSE for more.
 */
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse_macro_input, Data, DeriveInput};

use proc_macro2::{Ident, TokenStream};
use std::iter::FromIterator;
use syn::parse::Parser;
use syn::spanned::Spanned;

pub fn derive_ast_node(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_item = TokenStream::from(item.clone());
    let input = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;
    let (generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match &input.data {
        Data::Struct(_) => {
            let attr = input
                .attrs
                .iter()
                .find(|it| {
                    it.path
                        .get_ident()
                        .map_or(false, |id| id.to_string().as_str() == "kind")
                })
                .expect("Need attribute kind on #[derive(AstNode)] for struct");
            let kind = attr.parse_args_with(IdentParser).unwrap();
            proc_macro::TokenStream::from(quote! {
                impl #generics NewSynTreeElem for #name #ty_generics #where_clause {
                    unsafe fn new(syntax: SyntaxNode) -> Self {
                        Self {syntax}
                    }
                }

                impl #generics CastableFromSyntaxNode for #name #ty_generics #where_clause {
                    const KIND: SyntaxKind = #kind;
                }

                impl #generics AstNode for #name #ty_generics #where_clause {
                    fn syntax(&self) -> &SyntaxNode {
                        &self.syntax
                    }
                }
            })
        }
        Data::Enum(ref data) => {
            let variants = data
                .variants
                .iter()
                .filter_map(|variant| {
                    let attr = variant.attrs.iter().find(|variant_attr| {
                        variant_attr
                            .path
                            .get_ident()
                            .map_or(false, |id| id.to_string().as_str() == "kind")
                    })?;

                    let syntax_kind = attr.parse_args_with(IdentParser).unwrap();

                    let ty = variant
                        .attrs
                        .iter()
                        .find(|variant_attr| {
                            variant_attr
                                .path
                                .get_ident()
                                .map_or(false, |id| id.to_string().as_str() == "ty")
                        })
                        .and_then(|it| it.parse_args_with(IdentParser).ok())
                        .unwrap_or_else(|| syntax_kind.clone());

                    Some(Variant {
                        syntax_kind,
                        target_type: ty,
                        variant_name: &variant.ident,
                    })
                })
                .collect::<Variants>();
            let variants2 = variants.v.iter().collect::<Variants2>();
            proc_macro::TokenStream::from(quote! {
                impl #generics NewSynTreeElem for #name #ty_generics #where_clause {
                    unsafe fn new(syntax: SyntaxNode) -> Self {
                        panic!("Cannot create from enum")
                    }
                }

                impl #generics CastableFromSyntaxNode for #name #ty_generics #where_clause {
                    fn cast(syntax: SyntaxNode) -> Option<Self> where Self: Sized {
                        match syntax.kind() {
                            #variants
                            _ => None,
                        }
                    }
                }

                impl #generics AstNode for #name #ty_generics #where_clause {
                    fn syntax(&self) -> &SyntaxNode {
                        match self {
                            #variants2
                            _ => panic!("Cannot get syntax of {:?}", self)
                        }
                    }
                }
            })
        }
        _ => {
            panic!("Not defined on types other than structs and enums")
        }
    }
}

struct IdentParser;

impl Parser for IdentParser {
    type Output = Ident;

    fn parse2(self, tokens: TokenStream) -> syn::Result<Self::Output> {
        Ok(Ident::new(&tokens.to_string(), tokens.span()))
    }
}

struct Variant<'a> {
    syntax_kind: Ident,
    variant_name: &'a Ident,
    target_type: Ident,
}

impl<'a> ToTokens for Variant<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Variant {
            ref syntax_kind,
            variant_name,
            ref target_type,
        } = *self;
        tokens.append_all(
            quote! {#syntax_kind => Some(Self::#variant_name(#target_type::cast(syntax).unwrap()))},
        )
    }
}

struct Variants<'a> {
    v: Vec<Variant<'a>>,
}

impl<'a> FromIterator<Variant<'a>> for Variants<'a> {
    fn from_iter<T: IntoIterator<Item = Variant<'a>>>(iter: T) -> Self {
        Self {
            v: iter.into_iter().collect(),
        }
    }
}

impl<'a> ToTokens for Variants<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.v.iter().map(|variant| {
            quote! {#variant,}
        }))
    }
}

struct Variant2<'a> {
    variant_name: &'a Ident,
}

impl<'a> ToTokens for Variant2<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Variant2 { variant_name } = *self;
        tokens.append_all(quote! {Self::#variant_name(v) => v.syntax()})
    }
}

struct Variants2<'a> {
    v: Vec<Variant2<'a>>,
}

impl<'a> FromIterator<&'a Variant<'a>> for Variants2<'a> {
    fn from_iter<T: IntoIterator<Item = &'a Variant<'a>>>(iter: T) -> Self {
        Self {
            v: iter
                .into_iter()
                .map(|it| Variant2 {
                    variant_name: it.variant_name,
                })
                .collect(),
        }
    }
}

impl<'a> ToTokens for Variants2<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.v.iter().map(|variant| {
            quote! {#variant,}
        }))
    }
}
