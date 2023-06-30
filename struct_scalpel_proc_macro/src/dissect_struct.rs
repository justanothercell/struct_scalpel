use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::Index;
use venial::{Struct, NamedStructFields, TupleStructFields, GenericParamList};

pub(crate) fn dissect_struct(s: Struct) -> TokenStream {
    let fields = match &s.fields {
        venial::StructFields::Unit => dissect_unit_struct(),
        venial::StructFields::Tuple(t) => dissect_tuple_struct(t),
        venial::StructFields::Named(n) => dissect_named_struct(n),
    };

    let struct_name = &s.name;

    let generics = s.generic_params.to_owned().unwrap_or(GenericParamList::default());
    let wheres = &s.where_clause;
    let attrs = s.attributes.iter()
    // filter out all doc comments
    .filter(|a| a.path.iter().map(|t| t.to_string()).collect::<Vec<_>>().concat() != "doc")
    .map(|a| a.to_token_stream().to_string());
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
                    structure: ::struct_scalpel::Structure::Struct(#fields)
                };
                ::std::mem::forget(dummy);
                layout
            }
        }
    })
}

#[allow(unused_variables)]
fn dissect_named_struct(n: &NamedStructFields) -> proc_macro2::TokenStream {
    let field_info = n.fields.into_iter().map(|(f, _)| {
        let fname = f.name.to_string();
        let f = &f.name;
        quote! {
            (
                #fname,
                ::struct_scalpel::FieldInfo::from_val_and_base(base, &dummy.#f)
            )
        }
    });

    quote! {
        ::struct_scalpel::StructFields::Named(vec![#(#field_info,)*])
    }
}

#[allow(unused_variables)]
fn dissect_tuple_struct(t: &TupleStructFields) -> proc_macro2::TokenStream {
    let field_info = t.fields.into_iter().enumerate().map(|(i, (f, _))| {
        let ty = &f.ty;
        let index = Index::from(i);
        quote! {
            ::struct_scalpel::FieldInfo::from_val_and_base(base, &dummy.#index)
        }
    });

    quote! {
        ::struct_scalpel::StructFields::Tuple(vec![#(#field_info,)*])
    }
}

#[allow(unused_variables)]
fn dissect_unit_struct() -> proc_macro2::TokenStream {
    quote! {
        ::struct_scalpel::StructFields::Unit
    }
}