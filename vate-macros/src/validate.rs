use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{
    punctuated::Punctuated, Attribute, Data, DataEnum, DataStruct, DeriveInput, Fields, Generics,
    Ident, MetaNameValue, Result, Token, Type, Variant,
};

pub fn expand_derive_validate(input: DeriveInput) -> Result<TokenStream2> {
    let DeriveInput {
        ident,
        generics,
        data,
        attrs,
        ..
    } = input;
    match data {
        Data::Struct(data) => expand_derive_validate_struct(ident, generics, data, &attrs),
        Data::Enum(data) => expand_derive_validate_enum(ident, generics, data, &attrs),
        _ => unimplemented!("Unsupported data storage type"),
    }
}

fn expand_derive_validate_struct(
    ident: Ident,
    generics: Generics,
    data: DataStruct,
    attrs: &[Attribute],
) -> Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let data_type = parse_outer_type_attrs("data", attrs)?;
    let error_type = parse_outer_type_attrs("error", attrs)?;

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
    ident: Ident,
    generics: Generics,
    data: DataEnum,
    attrs: &[Attribute],
) -> Result<TokenStream2> {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let data_type = parse_outer_type_attrs("data", attrs)?;
    let error_type = parse_outer_type_attrs("error", attrs)?;

    let mut arms = Vec::new();

    for Variant { ident, fields, .. } in data.variants.iter() {
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

fn parse_outer_type_attrs(ident: &'static str, attrs: &[Attribute]) -> Result<TokenStream2> {
    for attr in attrs {
        if !attr.path().is_ident("vate") {
            continue;
        }
        let list = attr.meta.require_list()?;
        let definitions =
            list.parse_args_with(Punctuated::<MetaNameValue, Token![,]>::parse_terminated)?;
        for definition in definitions {
            let ty: Type = syn::parse2(definition.value.into_token_stream())?;
            if definition.path.is_ident(ident) {
                return Ok(quote!(#ty));
            }
        }
    }
    Ok(quote!(()))
}

fn parse_inner_attrs(fields: &Fields) -> Result<TokenStream2> {
    let parsed = match fields {
        Fields::Unit => Ok(vec![]),
        Fields::Named(fields) => {
            fields.named.iter().map(|field| {
                let target = field.ident.as_ref().unwrap();
                let accessor = quote!(::vate::Accessor::Field(stringify!(#target)));

                field.attrs.iter()
                    .filter(|attr| attr.path().is_ident("vate"))
                    .map(|attr| {
                        let tokens = &attr.meta.require_list()?.tokens;
                        Ok(quote! {
                            ::vate::Bundle!(#tokens).run::<C>(#accessor, #target, data, parent_report)?;
                        })
                    })
                    .collect::<Result<Vec<_>>>()
            }).collect::<Result<Vec<_>>>()
        },
        Fields::Unnamed(fields) => {
            fields.unnamed.iter().enumerate().map(|(index, field)| {
                let target = format_ident!("field{index}");
                let accessor = quote!(::vate::Accessor::TupleIndex(#index));
                field.attrs.iter()
                    .filter(|attr| attr.path().is_ident("vate"))
                    .map(|attr| {
                        let tokens = &attr.meta.require_list()?.tokens;
                        Ok(quote! {
                            ::vate::Bundle!(#tokens).run::<C>(#accessor, #target, data, parent_report)?;
                        })
                    })
                    .collect::<Result<Vec<_>>>()
            }).collect::<Result<Vec<_>>>()
        },
    }?;
    Ok(quote!(#(#(#parsed)*)*))
}

fn destructure(ident: &Ident, fields: &Fields) -> TokenStream2 {
    match fields {
        Fields::Unit => quote!(#ident),
        Fields::Named(fields) => {
            let fields = fields.named.iter().map(|field| {
                let field = field.ident.as_ref().unwrap();
                quote!(#field)
            });
            quote!(#ident { #(#fields,)* })
        }
        Fields::Unnamed(fields) => {
            let fields: Vec<_> = (0..fields.unnamed.len())
                .map(|index| format_ident!("field{}", index))
                .collect();
            quote!(#ident(#(#fields,)*))
        }
    }
}
