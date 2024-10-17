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
        syn::Data::Struct(data) => expand_derive_validate_struct(ident, generics, data, &attrs),
        _ => unimplemented!("Unsupported data storage type"),
    }
}

fn expand_derive_validate_struct(
    ident: syn::Ident,
    generics: syn::Generics,
    data: syn::DataStruct,
    attrs: &[syn::Attribute],
) -> syn::Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let data_type = parse_outer_type_attrs("data", attrs)?;
    let error_type = parse_outer_type_attrs("error", attrs)?;

    let body = parse_inner_validator_attrs(data.fields)?;

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

fn parse_outer_type_attrs(
    ident: &'static str,
    attrs: &[syn::Attribute],
) -> syn::Result<TokenStream2> {
    for attr in attrs {
        if !attr.path().is_ident("vate") {
            continue;
        }
        let list = attr.meta.require_list()?;
        let definitions = list
            .parse_args_with(Punctuated::<syn::MetaNameValue, syn::Token![,]>::parse_terminated)?;
        for definition in definitions {
            let ty = definition.value;
            if definition.path.is_ident(ident) {
                return Ok(quote!(#ty));
            }
        }
    }
    Ok(quote!(()))
}

fn parse_inner_validator_attrs(fields: syn::Fields) -> syn::Result<Vec<TokenStream2>> {
    let mut body = Vec::new();

    for (index, field) in fields.into_iter().enumerate() {
        let index = syn::Index::from(index);

        let item_ident = match field.ident {
            Some(ref ident) => quote!(#ident),
            None => quote!(#index),
        };

        let accessor = match field.ident {
            Some(ident) => quote!(::vate::Accessor::Field(stringify!(#ident))),
            None => quote!(::vate::Accessor::TupleIndex(#index)),
        };

        for attr in field.attrs.iter() {
            if !attr.path().is_ident("vate") {
                continue;
            }
            let tokens = &attr.meta.require_list()?.tokens;
            let code = quote! {
                ::vate::Bundle!(#tokens).run::<C>(#accessor, &self.#item_ident, data, parent_report)?;
            };
            body.push(code);
        }
    }

    Ok(body)
}
