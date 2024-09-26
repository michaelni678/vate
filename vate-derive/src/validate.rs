use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::punctuated::Punctuated;

pub fn expand_derive_validate(input: syn::DeriveInput) -> syn::Result<TokenStream2> {
    let syn::DeriveInput {
        ident,
        generics,
        data,
        attrs,
        ..
    } = input;
    match data {
        syn::Data::Struct(data) => expand_derive_validate_struct(ident, generics, data, attrs),
        _ => unimplemented!("Unsupported data storage type"),
    }
}

pub fn expand_derive_validate_struct(
    ident: syn::Ident,
    generics: syn::Generics,
    data: syn::DataStruct,
    attrs: Vec<syn::Attribute>,
) -> syn::Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut data_type = quote!(());
    let mut error_type = quote!(());

    for attr in attrs {
        if !attr.path().is_ident("vate") {
            continue;
        }
        let list = attr.meta.require_list()?;
        let definitions = list
            .parse_args_with(Punctuated::<syn::MetaNameValue, syn::Token![,]>::parse_terminated)?;
        for definition in definitions {
            let ty = definition.value;
            if definition.path.is_ident("data") {
                data_type = quote!(#ty);
            } else if definition.path.is_ident("error") {
                error_type = quote!(#ty);
            }
        }
    }

    let mut body = Vec::new();

    for (index, field) in data.fields.into_iter().enumerate() {
        let item_ident = field.ident.map_or(quote!(#index), |ident| quote!(#ident));
        for attr in field.attrs.iter() {
            if !attr.path().is_ident("vate") {
                continue;
            }
            let tokens = &attr.meta.require_list()?.tokens;
            let code = quote! {
                ::vate::Bundle!(#tokens).run::<C>(::vate::Accessor::Field(stringify!(#item_ident)), &self.#item_ident, data, parent_report)?;
            };
            body.push(code);
        }
    }

    Ok(quote! {
        impl #impl_generics ::vate::Validate for #ident #ty_generics #where_clause {
            type Data = #data_type;

            type Error = #error_type;

            fn validate<C: ::vate::Collector<Self::Error>>(
                &self,
                data: &Self::Data,
                parent_report: &mut ::vate::Report<Self::Error>,
            ) -> Result<(), ::vate::Exit<Self::Error>> {
                use ::vate::Validator;
                #(#body)*
                Ok(())
            }
        }
    })
}