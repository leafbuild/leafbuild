//! Only meant to be used internally
extern crate proc_macro;
extern crate proc_macro2;

mod derive_ast_node;
mod derive_loc;

#[proc_macro_derive(Loc, attributes(whole_span, start_span, end_span))]
pub fn derive_loc(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_loc::derive_loc(item)
}

#[proc_macro_attribute]
pub fn ast_node(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    derive_ast_node::derive_ast_node(attr, item)
}
