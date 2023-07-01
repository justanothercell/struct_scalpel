use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use venial::{Union, GenericParamList};

pub(crate) fn dissect_union(u: Union) -> TokenStream {
    let struct_name = &u.name;

    let generics = u.generic_params.to_owned().unwrap_or(GenericParamList::default());
    let wheres = &u.where_clause;
    let attrs = u.attributes.iter()
    // filter out all doc comments
    .filter(|a| a.path.iter().map(|t| t.to_string()).collect::<Vec<_>>().concat() == "repr")
    .map(|a| a.to_token_stream().to_string());

    let fields = u.fields.fields.iter().map(|(f, _)| {
        let fname = f.name.to_string();
        let ty = &f.ty;
        quote!{
            (
                #fname,
                ::struct_scalpel::FieldInfo {
                    type_name: ::std::any::type_name::<#ty>(), 
                    size: ::std::mem::size_of::<#ty>(), 
                    align: ::std::mem::align_of::<#ty>(), 
                    offset: 0
                }
            )
        }
    });

    TokenStream::from(quote! {
        impl #generics ::struct_scalpel::Dissectible for #struct_name #generics #wheres {
            fn field_info() -> ::struct_scalpel::LayoutInfo {
                let dummy: #struct_name #generics = unsafe { ::struct_scalpel::dummy_nonzero() };
                let base = &dummy as *const _ as usize;
                let layout = ::struct_scalpel::LayoutInfo {
                    attrs: vec![#(#attrs,)*],
                    name: ::std::any::type_name::<#struct_name #generics>(),
                    size: ::std::mem::size_of::<#struct_name #generics>(),
                    align: ::std::mem::align_of::<#struct_name #generics>(),
                    structure: ::struct_scalpel::Structure::Union(vec![#(#fields,)*])
                };
                ::std::mem::forget(dummy);
                layout
            }
        }
    })
}