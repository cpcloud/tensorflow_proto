extern crate proc_macro;

use proc_macro::TokenStream;
use syn::visit_mut::{self, VisitMut};

struct PrefixItemStruct;

impl VisitMut for PrefixItemStruct {
    fn visit_derive_input_mut(&mut self, node: &mut syn::DeriveInput) {
        node.attrs
            .push(syn::parse_quote!(#[derive(serde::Deserialize, serde::Serialize)]));

        if let syn::Data::Struct(_) = node.data {
            node.attrs.push(syn::parse_quote!(#[serde(default)]));
        }

        visit_mut::visit_derive_input_mut(self, node);
    }
}

#[proc_macro_attribute]
pub fn serde_default_viable(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = syn::parse_macro_input!(input);
    PrefixItemStruct.visit_derive_input_mut(&mut item);
    quote::quote!(#item).into()
}
