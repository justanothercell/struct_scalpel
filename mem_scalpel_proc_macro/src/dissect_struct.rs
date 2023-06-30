use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use venial::Struct;

pub(crate) fn dissect_struct(s: Struct) -> TokenStream {
    let struct_name = s.name;
    TokenStream::from(quote! {
        impl #struct_name {

        }
    })
}