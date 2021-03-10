use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse_macro_input, Data, DeriveInput};

use proc_macro2::{Ident, TokenStream};
use std::iter::FromIterator;
use syn::parse::Parser;
use syn::spanned::Spanned;

pub fn derive_fake_ast_node(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;
    let (generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let variants = match input.data {
        Data::Enum(ref data) => data.variants.iter().filter_map(|variant| {
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
        }),
        _ => panic!("Only defined on enums"),
    }
    .collect::<Variants>();

    proc_macro::TokenStream::from(quote! {
        impl #generics CastableFromSyntaxNode for #name #ty_generics #where_clause {
            fn cast(syntax: SyntaxNode) -> Option<Self> where Self: Sized {
                match syntax.kind() {
                    #variants
                    _ => None,
                }
            }
        }

        impl #generics FakeAstNode for #name #ty_generics #where_clause {}
    })
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
