use proc_macro::TokenStream;

mod dissect_struct;
mod dissect_enum;
mod dissect_union;

#[proc_macro_derive(Dissectible)]
pub fn dissect(stream: TokenStream) -> TokenStream {
    match venial::parse_declaration(proc_macro2::TokenStream::from(stream)) {
        Ok(venial::Declaration::Struct(s)) => dissect_struct::dissect_struct(s),
        Ok(venial::Declaration::Enum(e)) => dissect_enum::dissect_enum(e),
        Ok(venial::Declaration::Union(u)) => dissect_union::dissect_union(u),
        Ok(_) => panic!("Can only derive this trait on a struct, enum or union"),
        Err(_) => panic!("Error parsing into valid Rust"),
    }
}