//! Only meant to be used internally
extern crate proc_macro;
extern crate proc_macro2;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use std::borrow::Cow;
use std::iter::FromIterator;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Variant};

#[proc_macro_derive(Loc, attributes(whole_span, start_span, end_span))]
pub fn derive_loc(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;
    let (generics, ty_generics, where_clause) = input.generics.split_for_impl();

    if let Some((start, end, whole_span)) = find_start_end_span(name, &input.data) {
        proc_macro::TokenStream::from(quote! {
            impl #generics Loc for #name #ty_generics #where_clause {
                fn get_start(&self) -> usize {
                    #start
                }

                fn get_end(&self) -> usize {
                    #end
                }

                fn get_rng(&self) -> Location {
                    #whole_span
                }
            }
        })
    } else {
        "".parse().unwrap()
    }
}

fn find_start_end_span(
    name: &Ident,
    data: &Data,
) -> Option<(TokenStream, TokenStream, TokenStream)> {
    match *data {
        Data::Struct(ref data) => {
            find_span_points(&data.fields).map(|span_points| match span_points {
                SpanPoints::Whole(whole_span) => {
                    let name = &whole_span.1.ident;
                    (
                        quote! { self.#name.get_start() },
                        quote! { self.#name.get_end() },
                        quote! { self.#name.get_rng() },
                    )
                }
                SpanPoints::StartEnd(start, end) => {
                    let start_name = &start.1.ident;
                    let end_name = &end.1.ident;
                    if start.0 != end.0 {
                        (
                            quote! { self.#start_name.get_start() },
                            quote! { self.#end_name.get_end() },
                            quote! { self.#start_name.get_start()..self.#end_name.get_end() },
                        )
                    } else {
                        (
                            quote! { self.#start_name.get_start() },
                            quote! { self.#start_name.get_end() },
                            quote! { self.#start_name.get_rng() },
                        )
                    }
                }
                SpanPoints::StartFound(_) => {
                    panic!("Cannot derive Loc: only #[start_span] found on {}", name)
                }
                SpanPoints::EndFound(_) => {
                    panic!("Cannot derive Loc: only #[end_span] found on {}", name)
                }
                SpanPoints::NothingFound => {
                    panic!("Cannot derive Loc: no markers found on {}", name)
                }
            })
        }
        Data::Enum(ref data) => {
            let mat = data
                .variants
                .iter()
                .filter_map(|variant| Some((variant, find_span_points(&variant.fields)?)))
                .collect::<Vec<_>>();

            Some((
                {
                    let variants = VariantMatches {
                        enum_name: &name,
                        matches: &mat,
                        act: ActionType::GetStart,
                    };
                    quote! { match self { #variants } }
                },
                {
                    let variants = VariantMatches {
                        enum_name: &name,
                        matches: &mat,
                        act: ActionType::GetEnd,
                    };
                    quote! { match self { #variants } }
                },
                {
                    let variants = VariantMatches {
                        enum_name: &name,
                        matches: &mat,
                        act: ActionType::GetSpan,
                    };
                    quote! { match self { #variants } }
                },
            ))
        }
        Data::Union(_) => None,
    }
}

#[derive(Clone, Copy)]
enum SpanPoints<'a> {
    Whole((usize, &'a Field)),
    StartEnd((usize, &'a Field), (usize, &'a Field)),
    StartFound((usize, &'a Field)),
    EndFound((usize, &'a Field)),
    NothingFound,
}

impl<'a> SpanPoints<'a> {
    fn found_end(self, end: (usize, &'a Field)) -> Self {
        match self {
            Self::StartFound(start) => Self::StartEnd(start, end),
            Self::NothingFound => Self::EndFound(end),
            other => other,
        }
    }

    fn found_start(self, start: (usize, &'a Field)) -> Self {
        match self {
            Self::EndFound(end) => Self::StartEnd(start, end),
            Self::NothingFound => Self::StartFound(start),
            other => other,
        }
    }

    fn found_whole(self, whole: (usize, &'a Field)) -> Self {
        match self {
            Self::NothingFound | Self::StartFound(_) | Self::EndFound(_) => Self::Whole(whole),
            other => other,
        }
    }

    fn get_start_end(
        self,
        enum_name: &Ident,
        field_variant: &Ident,
    ) -> ((usize, &'a Field), (usize, &'a Field)) {
        match self {
            SpanPoints::Whole(whole) => (whole, whole),
            SpanPoints::StartEnd(start, end) => (start, end),
            SpanPoints::StartFound(_) => panic!(
                "Cannot derive Loc on {}: only #[start_span] found in {}",
                enum_name, field_variant
            ),
            SpanPoints::EndFound(_) => {
                panic!(
                    "Cannot derive Loc on {}: only #[end_span] found in {}",
                    enum_name, field_variant
                )
            }
            SpanPoints::NothingFound => {
                panic!("Cannot derive Loc on {}: neither #[start_span], #[end_span] nor #[whole_span] found in {}", enum_name, field_variant)
            }
        }
    }
}

impl<'a> FromIterator<&'a Field> for SpanPoints<'a> {
    fn from_iter<T: IntoIterator<Item = &'a Field>>(iter: T) -> Self {
        iter.into_iter().enumerate().collect()
    }
}

impl<'a> FromIterator<(usize, &'a Field)> for SpanPoints<'a> {
    fn from_iter<T: IntoIterator<Item = (usize, &'a Field)>>(iter: T) -> Self {
        iter.into_iter()
            .fold(SpanPoints::NothingFound, |outer_acc, (ind, f)| {
                f.attrs.iter().fold(outer_acc, |acc, attr| {
                    attr.path
                        .get_ident()
                        .map(|id| match id.to_string().as_str() {
                            "start_span" => acc.found_start((ind, f)),
                            "end_span" => acc.found_end((ind, f)),
                            "whole_span" => acc.found_whole((ind, f)),
                            _ => acc,
                        })
                        .unwrap_or(acc)
                })
            })
    }
}

fn find_span_points(fields: &Fields) -> Option<SpanPoints> {
    match fields {
        Fields::Named(ref fields) => Some(fields.named.iter().collect()),
        Fields::Unnamed(fields) => Some(fields.unnamed.iter().collect()),
        Fields::Unit => None,
    }
}

struct VariantMatches<'a> {
    enum_name: &'a Ident,
    matches: &'a Vec<(&'a Variant, SpanPoints<'a>)>,
    act: ActionType,
}

#[allow(clippy::enum_variant_names)]
enum ActionType {
    GetStart,
    GetEnd,
    GetSpan,
}

impl<'a> ToTokens for VariantMatches<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.matches.iter().for_each(|&m| match m.0.fields {
            Fields::Named(_) => {
                tokens.append_all({
                    let enum_name = self.enum_name;
                    let name = &m.0.ident;
                    let (start_field, end_field) = m.1.get_start_end(&self.enum_name, &m.0.ident);

                    match self.act {
                        ActionType::GetStart => {
                            let s_field = &start_field.1.ident;
                            quote! { #enum_name::#name { #s_field: loc, .. } => { loc.get_start() } }
                        }
                        ActionType::GetEnd => {
                            let e_field = if start_field.0 != end_field.0 { &end_field.1.ident } else { &start_field.1.ident } ;
                            quote! { #enum_name::#name { #e_field: loc, .. } => { loc.get_end() }}
                        }
                        ActionType::GetSpan => {
                            if start_field.0 == end_field.0 {
                                let s_field = &start_field.1.ident;
                                quote! { #enum_name::#name { #s_field: loc, .. } => { loc.get_rng() }}
                            } else {
                                let (s_field, e_field) = (&start_field.1.ident, &end_field.1.ident);
                                quote! { #enum_name::#name { #s_field: start, #e_field: end, ..} => { start.get_start()..end.get_end() }}
                            }
                        }
                    }
                });
            }
            Fields::Unnamed(ref f) => tokens.append_all({
                let enum_name = self.enum_name.to_string();
                let en = Ident::new(&enum_name, Span::mixed_site());
                let name = &m.0.ident;

                let (start_field, end_field) = m.1.get_start_end(&self.enum_name, &m.0.ident);

                let start_ind = Ident::new("start_span", Span::mixed_site());
                let end_ind = Ident::new("end_span", Span::mixed_site());

                let f = f.unnamed.iter().enumerate().map(|(ind, _)| {
                    if ind == start_field.0 {
                        Cow::Borrowed(&start_ind)
                    } else if ind == end_field.0 {
                        Cow::Borrowed(&end_ind)
                    } else {
                        Cow::Owned(Ident::new("_", Span::mixed_site()))
                    }
                });
                let act = match self.act {
                    ActionType::GetStart => {
                        quote! { { #start_ind.get_start() } }
                    }
                    ActionType::GetEnd => {
                        if start_field.0 != end_field.0 {
                            quote! { { #end_ind.get_end() } }
                        } else {
                            quote! { { #start_ind.get_end() } }
                        }
                    }
                    ActionType::GetSpan => {
                        if start_field.0 == end_field.0 {
                            quote! {{ #start_ind.get_rng() }}
                        } else {
                            quote! {{ #start_ind.get_start()..#end_ind.get_end() }}
                        }
                    }
                };

                quote! { #en::#name ( #(#f, )* ) => { #act } }
            }),
            Fields::Unit => {}
        })
    }
}
