extern crate proc_macro;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(SerdeDefaultViable)]
pub fn derive_it(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let name = ast.ident.clone();

    match syn::Item::from(ast) {
        syn::Item::Struct(_) => {
            quote::quote! {
                paste::paste! {
                    #[derive(Default, Clone, serde::Deserialize, serde::Serialize)]
                    #[serde(transparent, default)]
                    struct [<Serde #name>] {
                        inner: #name
                    }

                    impl<'de> serde::Deserialize<'de> for #name {
                        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de> {
                            Ok([<Serde #name>]::deserialize(deserializer)?.inner)
                        }
                    }

                    impl serde::Serialize for #name {
                        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where
                            S: serde::Serializer,
                        {
                            let outer = [<Serde #name>] { inner: self.clone() };
                            outer.serialize(serializer)
                        }
                    }
                }
            }
        }
        syn::Item::Enum(_) => {
            quote::quote! {
                paste::paste! {
                    #[derive(Clone, serde::Deserialize, serde::Serialize)]
                    #[serde(transparent)]
                    struct [<Serde #name>](#name);

                    impl<'de> serde::Deserialize<'de> for #name {
                        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de> {
                            Ok([<Serde #name>]::deserialize(deserializer)?.0)
                        }
                    }

                    impl serde::Serialize for #name {
                        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where
                            S: serde::Serializer,
                        {
                            use serde::Serialize;
                            let outer = [<Serde #name>](self.clone());
                            outer.serialize(serializer)
                        }
                    }
                }
            }
        }
        _ => quote::quote!(),
    }
    .into()
}
