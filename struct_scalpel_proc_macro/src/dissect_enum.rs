use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use venial::{Enum, GenericParamList};

pub(crate) fn dissect_enum(e: Enum) -> TokenStream {
    let enum_name = &e.name;

    let generics = e.generic_params.to_owned().unwrap_or(GenericParamList::default());
    let wheres = &e.where_clause;
    let attrs = e.attributes.iter()
    // filter out all doc comments
    .filter(|a| a.path.iter().map(|t| t.to_string()).collect::<Vec<_>>().concat() == "repr")
    .map(|a| a.to_token_stream().to_string());
    let variants = e.variants.iter().map(|(v, _)| {
        let name = &v.name;
        let name_str = v.name.to_string();
        match &v.contents {
            venial::StructFields::Unit => quote! {
                (#name_str, ::struct_scalpel::StructFields::Unit)
            },
            venial::StructFields::Tuple(t) => {
                let vals = t.fields.iter().map(|(_t, _)| quote!{
                    unsafe { ::struct_scalpel::dummy_nonzero() }
                }).collect::<Vec<_>>();

                let idents = &t.fields.iter().enumerate().map(|(i, (_t, _))| Ident::new(&format!("v{i}"), Span::call_site())).collect::<Vec<_>>();
                
                let field_info = t.fields.iter().zip(idents).map(| ((_t, _), ident)| quote! {
                    ::struct_scalpel::FieldInfo::from_val_and_base(base, #ident)
                }).collect::<Vec<_>>();
                quote! {{
                    let dummy = #enum_name :: #generics :: #name ( #(#vals,)* );
                    let base = &dummy as *const _ as usize;
                    let field = if let #enum_name :: #generics :: #name ( #(#idents,)* ) = &dummy {
                        (#name_str, ::struct_scalpel::StructFields::Tuple(vec![#(#field_info,)*]))
                    } else { unreachable!() };
                    ::std::mem::forget(dummy);
                    field
                }}
            },
            venial::StructFields::Named(n) => {
                let idents = n.fields.iter().map(|(f, _)| &f.name).collect::<Vec<_>>();
                
                let field_info = n.fields.iter().zip(idents.iter()).map(|((_t, _), ident)| quote! {
                    (stringify!(#ident), ::struct_scalpel::FieldInfo::from_val_and_base(base, #ident))
                });

                quote! {{
                    let dummy = #enum_name :: #generics :: #name { #(#idents: unsafe { ::struct_scalpel::dummy_nonzero() },)* };
                    let base = &dummy as *const _ as usize;
                    let field = if let #enum_name :: #generics :: #name { #(#idents,)* } = &dummy {
                        (#name_str, ::struct_scalpel::StructFields::Named(vec![#(#field_info,)*]))
                    } else { unreachable!() };
                    ::std::mem::forget(dummy);
                    field
                }}
            },
        }
    }).collect::<Vec<proc_macro2::TokenStream>>();

    TokenStream::from(quote! {
        impl #generics ::struct_scalpel::Dissectible for #enum_name #generics #wheres {
            fn field_info() -> ::struct_scalpel::LayoutInfo {
                ::struct_scalpel::LayoutInfo {
                    attrs: vec![#(#attrs,)*],
                    name: ::std::any::type_name::<#enum_name #generics>(),
                    size: ::std::mem::size_of::<#enum_name #generics>(),
                    align: ::std::mem::align_of::<#enum_name #generics>(),
                    structure: ::struct_scalpel::Structure::Enum(vec![#(#variants,)*])
                }
            }
        }
    })
}