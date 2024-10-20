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

    let data_type = parse_outer_attrs("data", attrs)?;
    let error_type = parse_outer_attrs("error", attrs)?;

    let destructured = destructure(&ident, &data.fields);

    let body = parse_inner_attrs(&data.fields)?;

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
                #[allow(unused_variables)]
                let #destructured = self;
                #body
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

    let data_type = parse_outer_attrs("data", attrs)?;
    let error_type = parse_outer_attrs("error", attrs)?;

    let mut arms = Vec::new();

    for syn::Variant { ident, fields, .. } in data.variants.iter() {
        let destructured = destructure(ident, fields);

        let body = parse_inner_attrs(fields)?;

        arms.push(quote! {
            Self::#destructured => {
                let mut child_report = ::vate::Report::<Self::Error>::new(::vate::Accessor::Variant(stringify!(#ident)));
                {
                    // The proc-macro expects the ident `parent_report`, so rename child_report
                    // temporarily in this scope so this expands to accept the correct report.
                    let parent_report = &mut child_report;
                    #body
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

fn parse_outer_attrs(ident: &'static str, attrs: &[syn::Attribute]) -> syn::Result<TokenStream2> {
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

fn parse_inner_attrs(fields: &syn::Fields) -> syn::Result<TokenStream2> {
    let parsed: Vec<Vec<TokenStream2>> = match fields {
        syn::Fields::Unit => Ok(vec![]),
        syn::Fields::Named(fields) => {
            fields.named.iter().map(|field| {
                let target = field.ident.as_ref().unwrap();
                let accessor = quote!(::vate::Accessor::Field(stringify!(#target)));

                field.attrs.iter()
                    .filter(|attr| attr.path().is_ident("vate"))
                    .map(|attr| -> syn::Result<TokenStream2> {
                        let tokens = &attr.meta.require_list()?.tokens;
                        Ok(quote! {
                            ::vate::Bundle!(#tokens).run::<C>(#accessor, #target, data, parent_report)?;
                        })
                    })
                    .collect()
            }).collect()
        },
        syn::Fields::Unnamed(fields) => {
            fields.unnamed.iter().enumerate().map(|(index, field)| {
                let target = format_ident!("field{index}");
                let accessor = quote!(::vate::Accessor::TupleIndex(#index));
                field.attrs.iter()
                    .filter(|attr| attr.path().is_ident("vate"))
                    .map(|attr| -> syn::Result<TokenStream2> {
                        let tokens = &attr.meta.require_list()?.tokens;
                        Ok(quote! {
                            ::vate::Bundle!(#tokens).run::<C>(#accessor, #target, data, parent_report)?;
                        })
                    })
                    .collect()
            }).collect()
        },
    }?;
    Ok(quote!(#(#(#parsed)*)*))
}

fn destructure(ident: &syn::Ident, fields: &syn::Fields) -> TokenStream2 {
    match fields {
        syn::Fields::Unit => quote!(#ident),
        syn::Fields::Named(fields) => {
            let fields = fields.named.iter().map(|field| {
                let field = field.ident.as_ref().unwrap();
                quote!(#field)
            });
            quote!(#ident { #(#fields,)* })
        }
        syn::Fields::Unnamed(fields) => {
            let fields: Vec<_> = (0..fields.unnamed.len())
                .map(|index| format_ident!("field{}", index))
                .collect();
            quote!(#ident(#(#fields,)*))
        }
    }
}
