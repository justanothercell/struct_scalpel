use proc_macro::TokenStream;

mod dissect_struct;


#[proc_macro_derive(Dissect)]
pub fn dissect(stream: TokenStream) -> TokenStream {
    match venial::parse_declaration(proc_macro2::TokenStream::from(stream)) {
        Ok(venial::Declaration::Struct(s)) => dissect_struct::dissect_struct(s),
        Ok(venial::Declaration::Enum(e)) => unimplemented!(),
        Ok(venial::Declaration::Union(u)) => unimplemented!(),
        Ok(_) => panic!("Can only derive this trait on a struct, enum or union"),
        Err(_) => panic!("Error parsing into valid Rust"),
    }
/* 
    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let field_name = fields.iter().map(|field| &field.ident);
    let field_type = fields.iter().map(|field| &field.ty);

    let path = args.path;
    let struct_name = &input.ident;

    TokenStream::from(quote! {
        // Preserve the input struct unchanged in the output.
        #input

        impl #struct_name {
            fn route() {
                println!("{}", #path);

                // The following repetition expands to, for example:
                //
                //    let id: u32 = Default::default();
                //    let car_name: String = Default::default();
                #(
                    let #field_name: #field_type = Default::default();
                )*
            }
        }
    })
    */
}

