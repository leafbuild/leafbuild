use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use proc_macro2::TokenStream;

pub fn derive_ast_node(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input_item = TokenStream::from(item.clone());
    let input = parse_macro_input!(item as DeriveInput);
    let attr_data = TokenStream::from(attr);

    let name = &input.ident;
    let (generics, ty_generics, where_clause) = input.generics.split_for_impl();
    proc_macro::TokenStream::from(quote! {
        #input_item

        impl #generics CastableFromSyntaxNode for #name #ty_generics #where_clause {
            fn cast(syntax: SyntaxNode) -> Option<Self> where Self: Sized {
                match syntax.kind() {
                    #attr_data => Some(Self {syntax}),
                    _ => None,
                }
            }
        }

        impl #generics AstNode for #name #ty_generics #where_clause {
            fn syntax(&self) -> &SyntaxNode {
                &self.syntax
            }
        }
    })
}
