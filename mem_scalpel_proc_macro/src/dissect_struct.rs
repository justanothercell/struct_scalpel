use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::Index;
use venial::{Struct, NamedStructFields, TupleStructFields, GenericParamList, Attribute};

pub(crate) fn dissect_struct(s: Struct) -> TokenStream {
    match &s.fields {
        venial::StructFields::Unit => dissect_unit_struct(&s),
        venial::StructFields::Tuple(t) => dissect_tuple_struct(&s, t),
        venial::StructFields::Named(n) => dissect_named_struct(&s, n),
    }
}

pub(crate) fn dissect_named_struct(s: &Struct, n: &NamedStructFields) -> TokenStream {
    let struct_name = &s.name;

    let generics = s.generic_params.to_owned().unwrap_or(GenericParamList::default());
    let wheres = &s.where_clause;
    let attrs = s.attributes.iter()
    // filter out all doc comments
    .filter(|a| a.path.iter().map(|t| t.to_string()).collect::<Vec<_>>().concat() != "doc")
    .map(|a| a.to_token_stream().to_string())
    .collect::<Vec<_>>();

    let field_info = n.fields.into_iter().map(|(f, _)| {
        let fname = f.name.to_string();
        let f = &f.name;
        quote! {
            (
                #fname,
                ::mem_scalpel::FieldInfo::from_val_and_base(base, &dummy.#f)
            )
        }
    });

    TokenStream::from(quote! {
        impl #generics ::mem_scalpel::Dissectible for #struct_name #generics #wheres {
            fn field_info() -> ::mem_scalpel::LayoutInfo {
                let dummy: #struct_name #generics = unsafe { ::std::mem::MaybeUninit::zeroed().assume_init() };
                let base = &dummy as *const _ as usize;
                ::mem_scalpel::LayoutInfo::NamedStruct {
                    attrs: vec![#(#attrs,)*],
                    name: ::std::any::type_name::<#struct_name #generics>(),
                    size: ::std::mem::size_of::<#struct_name #generics>(),
                    align: ::std::mem::align_of::<#struct_name #generics>(),
                    fields: vec![#(#field_info,)*]
                }
            }
        }
    })
}

pub(crate) fn dissect_tuple_struct(s: &Struct, t: &TupleStructFields) -> TokenStream {
    let struct_name = &s.name;

    let generics = s.generic_params.to_owned().unwrap_or(GenericParamList::default());
    let wheres = &s.where_clause;
    let attrs = s.attributes.iter()
    // filter out all doc comments
    .filter(|a| a.path.iter().map(|t| t.to_string()).collect::<Vec<_>>().concat() != "doc")
    .map(|a| a.to_token_stream().to_string())
    .collect::<Vec<_>>();

    let field_info = t.fields.into_iter().enumerate().map(|(i, (f, _))| {
        let ty = &f.ty;
        let index = Index::from(i);
        quote! {
            ::mem_scalpel::FieldInfo::from_val_and_base(base, &dummy.#index)
        }
    });

    TokenStream::from(quote! {
        impl #generics ::mem_scalpel::Dissectible for #struct_name #generics #wheres {
            fn field_info() -> ::mem_scalpel::LayoutInfo {
                let dummy: #struct_name #generics = unsafe { ::std::mem::MaybeUninit::zeroed().assume_init() };
                let base = &dummy as *const _ as usize;
                ::mem_scalpel::LayoutInfo::TupleStruct {
                    attrs: vec![#(#attrs,)*],
                    name: ::std::any::type_name::<#struct_name #generics>(),
                    size: ::std::mem::size_of::<#struct_name #generics>(),
                    align: ::std::mem::align_of::<#struct_name #generics>(),
                    fields: vec![#(#field_info,)*]
                }
            }
        }
    })
}

pub(crate) fn dissect_unit_struct(s: &Struct) -> TokenStream {
    let struct_name = &s.name;

    let generics = s.generic_params.to_owned().unwrap_or(GenericParamList::default());
    let wheres = &s.where_clause;
    let attrs = s.attributes.iter()
    // filter out all doc comments
    .filter(|a| a.path.iter().map(|t| t.to_string()).collect::<Vec<_>>().concat() != "doc")
    .map(|a| a.to_token_stream().to_string())
    .collect::<Vec<_>>();
    TokenStream::from(quote! {
        impl #generics ::mem_scalpel::Dissectible for #struct_name #generics #wheres {
            fn field_info() -> ::mem_scalpel::LayoutInfo {
                ::mem_scalpel::LayoutInfo::UnitStruct {
                    attrs: vec![#(#attrs,)*],
                    name: ::std::any::type_name::<#struct_name #generics>(),
                }
            }
        }
    })
}