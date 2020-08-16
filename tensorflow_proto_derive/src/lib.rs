#[cfg(any(feature = "serde", feature = "convert"))]
extern crate proc_macro;

#[cfg(feature = "serde")]
struct SerdeDefaultViable;

#[cfg(feature = "serde")]
impl syn::visit_mut::VisitMut for SerdeDefaultViable {
    fn visit_derive_input_mut(&mut self, node: &mut syn::DeriveInput) {
        node.attrs
            .push(syn::parse_quote!(#[derive(serde::Deserialize, serde::Serialize)]));

        if let syn::Data::Struct(_) = node.data {
            node.attrs.push(syn::parse_quote!(#[serde(default)]));
        }

        syn::visit_mut::visit_derive_input_mut(self, node);
    }
}

#[cfg(feature = "serde")]
#[proc_macro_attribute]
pub fn serde_default_viable(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    use syn::visit_mut::VisitMut;

    let mut item = syn::parse_macro_input!(input);
    SerdeDefaultViable.visit_derive_input_mut(&mut item);
    quote::quote!(#item).into()
}

#[cfg(feature = "convert")]
#[proc_macro_derive(BytesTryConvertMessage)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = &item.ident;
    if let syn::Data::Struct(_) = item.data {
        quote::quote! {
            impl std::convert::TryFrom<#ident> for Vec<u8> {
                type Error = prost::EncodeError;

                fn try_from(message: #ident) -> Result<Self, Self::Error> {
                    use prost::Message;

                    let mut bytes = vec![];
                    message.encode(&mut bytes)?;
                    Ok(bytes)
                }
            }

            impl std::convert::TryFrom<Vec<u8>> for #ident {
                type Error = prost::DecodeError;

                fn try_from(bytes: Vec<u8>) -> Result<#ident, Self::Error> {
                    prost::Message::decode(bytes.as_slice())
                }
            }
        }
    } else {
        quote::quote!()
    }
    .into()
}
