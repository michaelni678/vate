//! Helpers for expanding the `Validate` derive macro.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse_quote, Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Field, Fields,
    FieldsNamed, FieldsUnnamed, Index, Result, Type, Variant,
};

use crate::utility::{filter_vate_attrs, parse_attrs};

pub fn expand_derive_validate(
    DeriveInput {
        attrs,
        ident,
        generics,
        data,
        ..
    }: DeriveInput,
) -> Result<TokenStream2> {
    let type_ident = ident;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let context_type: Type =
        parse_attrs(filter_vate_attrs(&attrs), "context")?.unwrap_or(parse_quote!(()));

    let error_type: Type =
        parse_attrs(filter_vate_attrs(&attrs), "error")?.unwrap_or(parse_quote!(()));

    let mut body = Vec::new();

    match data {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Named(FieldsNamed { named, .. }) => {
                let mut field_idents = Vec::new();
                let mut validations = Vec::new();

                for Field { ident, attrs, .. } in named {
                    let field_ident = ident.unwrap();
                    field_idents.push(field_ident.clone());

                    for Attribute { meta, .. } in filter_vate_attrs(&attrs) {
                        let tokens = &meta.require_list()?.tokens;

                        let validation = quote! {{
                            let invalid = ::vate::core::Invalid {
                                type_ident: ::vate::core::TypeIdent::Struct(stringify!(#type_ident)),
                                field_ident: ::vate::core::FieldIdent::Named(stringify!(#field_ident)),
                                vtags: Vec::new(),
                                detailers: Vec::new(),
                            };

                            let result = ::vate::validators::bundle::Bundle!(#tokens).run(
                                #field_ident,
                                context,
                                invalid,
                                interpreter,
                                data,
                                report,
                            );

                            if !matches!(result, Ok(::vate::core::ControlFlow::Continue)) {
                                return result;
                            }
                        }};

                        validations.push(validation);
                    }
                }

                body.push(quote! {
                    let Self { #(#field_idents,)* } = self;
                    #(#validations)*
                });
            }
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let mut validations = Vec::new();

                for (index, Field { attrs, .. }) in unnamed.into_iter().enumerate() {
                    let index = Index::from(index);

                    for Attribute { meta, .. } in filter_vate_attrs(&attrs) {
                        let tokens = &meta.require_list()?.tokens;

                        let validation = quote! {{
                            let invalid = ::vate::core::Invalid {
                                type_ident: ::vate::core::TypeIdent::Struct(stringify!(#type_ident)),
                                field_ident: ::vate::core::FieldIdent::Unnamed(#index),
                                vtags: Vec::new(),
                                detailers: Vec::new(),
                            };

                            let result = ::vate::validators::bundle::Bundle!(#tokens).run(
                                &fields.#index,
                                context,
                                invalid,
                                interpreter,
                                data,
                                report,
                            );

                            if !matches!(result, Ok(::vate::core::ControlFlow::Continue)) {
                                return result;
                            }
                        }};

                        validations.push(validation);
                    }
                }

                body.push(quote! {
                    let fields = self;
                    #(#validations)*
                });
            }
            Fields::Unit => {}
        },
        Data::Enum(DataEnum { variants, .. }) => {
            let mut arms = Vec::new();

            for Variant { ident, fields, .. } in variants {
                let variant_ident = ident;

                match fields {
                    Fields::Named(FieldsNamed { named, .. }) => {
                        let mut field_idents = Vec::new();
                        let mut validations = Vec::new();

                        for Field { ident, attrs, .. } in named {
                            let field_ident = ident.unwrap();
                            field_idents.push(field_ident.clone());

                            for Attribute { meta, .. } in filter_vate_attrs(&attrs) {
                                let tokens = &meta.require_list()?.tokens;

                                let validation = quote! {{
                                    let invalid = ::vate::core::Invalid {
                                        type_ident: ::vate::core::TypeIdent::Enum(stringify!(#type_ident), stringify!(#variant_ident)),
                                        field_ident: ::vate::core::FieldIdent::Named(stringify!(#field_ident)),
                                        vtags: Vec::new(),
                                        detailers: Vec::new(),
                                    };

                                    let result = ::vate::validators::bundle::Bundle!(#tokens).run(
                                        #field_ident,
                                        context,
                                        invalid,
                                        interpreter,
                                        data,
                                        report,
                                    );

                                    if !matches!(result, Ok(::vate::core::ControlFlow::Continue)) {
                                        return result;
                                    }
                                }};

                                validations.push(validation);
                            }
                        }

                        let arm = quote! {
                            Self::#variant_ident { #(#field_idents,)* } => {
                                #(#validations)*
                            },
                        };

                        arms.push(arm);
                    }
                    Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                        let mut field_idents = Vec::new();
                        let mut validations = Vec::new();

                        for (index, Field { attrs, .. }) in unnamed.into_iter().enumerate() {
                            field_idents.push(format_ident!("fields_{index}"));

                            let index = Index::from(index);

                            for Attribute { meta, .. } in filter_vate_attrs(&attrs) {
                                let tokens = &meta.require_list()?.tokens;

                                let validation = quote! {{
                                    let invalid = ::vate::core::Invalid {
                                        type_ident: ::vate::core::TypeIdent::Enum(stringify!(#type_ident), stringify!(#variant_ident)),
                                        field_ident: ::vate::core::FieldIdent::Unnamed(#index),
                                        vtags: Vec::new(),
                                        detailers: Vec::new(),
                                    };

                                    let result = ::vate::validators::bundle::Bundle!(#tokens).run(
                                        fields.#index,
                                        context,
                                        invalid,
                                        interpreter,
                                        data,
                                        report,
                                    );

                                    if !matches!(result, Ok(::vate::core::ControlFlow::Continue)) {
                                        return result;
                                    }
                                }};

                                validations.push(validation);
                            }
                        }

                        let arm = quote! {
                            Self::#variant_ident(#(#field_idents,)*) => {
                                let fields = (#(#field_idents,)*);
                                #(#validations)*
                            },
                        };

                        arms.push(arm);
                    }
                    Fields::Unit => {}
                }
            }

            body.push(quote! {
                match self {
                    #(#arms)*
                    _ => {},
                }
            });
        }
        Data::Union(_) => return Err(Error::new_spanned(type_ident, "Unions are not supported")),
    }

    Ok(quote! {
        impl #impl_generics ::vate::core::Validate for #type_ident #ty_generics #where_clause {
            type Context = #context_type;

            type Error = #error_type;

            fn validate<D, R>(
                &self,
                context: &Self::Context,
                interpreter: &::vate::core::Interpreter<D>,
                data: &D,
                report: &mut R,
            ) -> Result<::vate::core::ControlFlow, Self::Error>
            where
                R: ::vate::core::Report
            {
                use ::vate::core::Validator;

                #(#body)*

                Ok(::vate::core::ControlFlow::Continue)
            }
        }
    })
}
