use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
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
        syn::Data::Enum(data) => expand_derive_validate_enum(ident, generics, data, &attrs),
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

    let body = parse_inner_validator_attrs(|item| quote!(&self.#item), &data.fields)?;

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

fn expand_derive_validate_enum(
    ident: syn::Ident,
    generics: syn::Generics,
    data: syn::DataEnum,
    attrs: &[syn::Attribute],
) -> syn::Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let data_type = parse_outer_type_attrs("data", attrs)?;
    let error_type = parse_outer_type_attrs("error", attrs)?;

    let mut arms = Vec::new();

    for variant in data.variants {
        let variant_ident = &variant.ident;

        let variant_fields = match &variant.fields {
            syn::Fields::Unit => quote!(),
            syn::Fields::Named(fields) => {
                let mut field_idents = Vec::new();
                for field in fields.named.iter() {
                    let field_ident = field.ident.clone().unwrap();
                    field_idents.push(quote!(#field_ident));
                }
                quote!({ #(#field_idents)* })
            }
            syn::Fields::Unnamed(fields) => {
                let mut field_idents = Vec::new();
                let field_count = fields.unnamed.len();
                for index in 0..field_count {
                    let field_ident = format_ident!("field{}", index);
                    field_idents.push(field_ident);
                }
                quote!((#(#field_idents)*))
            }
        };

        let variant_arm = match &variant.fields {
            syn::Fields::Unit => vec![],
            syn::Fields::Named(_) => parse_inner_validator_attrs(
                |field_ident| {
                    let ident = format_ident!("{field_ident}");
                    quote!(#ident)
                },
                &variant.fields,
            )?,
            syn::Fields::Unnamed(_) => parse_inner_validator_attrs(
                |index| {
                    let field_ident = format_ident!("field{index}");
                    quote!(#field_ident)
                },
                &variant.fields,
            )?,
        };

        arms.push(quote! {
            #ident::#variant_ident #variant_fields => {
                let mut child_report = ::vate::Report::<Self::Error>::new(::vate::Accessor::Variant(stringify!(#variant_ident)));
                {
                    // The proc-macro expects the ident `parent_report`, so rename child_report
                    // temporarily in this scope so this expands to accept the correct report.
                    let parent_report = &mut child_report;
                    #(#variant_arm)*
                }
                C::apply(parent_report, child_report)?;
            }
        });
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
                match self {
                    #(#arms)*
                    _ => {},
                }
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

fn parse_inner_validator_attrs(
    item_retriever_fn: impl Fn(TokenStream2) -> TokenStream2,
    fields: &syn::Fields,
) -> syn::Result<Vec<TokenStream2>> {
    let mut body = Vec::new();

    for (index, field) in fields.iter().enumerate() {
        let item = match &field.ident {
            Some(ident) => quote!(#ident),
            None => {
                let index = syn::Index::from(index);
                quote!(#index)
            }
        };

        let item_retriever = item_retriever_fn(item);

        let accessor = match &field.ident {
            Some(ident) => quote!(::vate::Accessor::Field(stringify!(#ident))),
            None => quote!(::vate::Accessor::TupleIndex(#index)),
        };

        for attr in field.attrs.iter() {
            if !attr.path().is_ident("vate") {
                continue;
            }
            let tokens = &attr.meta.require_list()?.tokens;
            let code = quote! {
                ::vate::Bundle!(#tokens).run::<C>(#accessor, #item_retriever, data, parent_report)?;
            };
            body.push(code);
        }
    }

    Ok(body)
}
