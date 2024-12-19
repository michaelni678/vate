//! Helpers for expanding the `Validate` derive macro.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse_quote, Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Field, Fields,
    FieldsNamed, FieldsUnnamed, Index, Result, Type, Variant,
};

use crate::utils::{filter_attrs, parse_attrs};

pub fn expand_derive_validate(
    DeriveInput {
        attrs,
        ident: type_ident,
        generics,
        data,
        ..
    }: DeriveInput,
) -> Result<TokenStream2> {
    let core = quote! { ::vate::core };
    let validators = quote! { ::vate::validators };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let context_type: Type =
        parse_attrs(filter_attrs(&attrs), "context")?.unwrap_or(parse_quote!(()));
    let error_type: Type = parse_attrs(filter_attrs(&attrs), "error")?.unwrap_or(parse_quote!(()));

    let mut body = Vec::new();

    body.push(quote! {
        use #core::Validator;
    });

    match data {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Named(FieldsNamed { named, .. }) => {
                let mut field_idents = Vec::new();
                let mut validations = Vec::new();

                for Field {
                    ident: field_ident,
                    attrs,
                    ..
                } in named
                {
                    let field_ident = field_ident.unwrap();

                    field_idents.push(field_ident.clone());

                    for Attribute { meta, .. } in filter_attrs(&attrs) {
                        let tokens = &meta.require_list()?.tokens;

                        validations.push(quote! {{
                                let validating_args = #core::ValidatingArgs {
                                    target: #field_ident,
                                    context,
                                };

                                let mut invalid = #core::Invalid::default();
                                invalid.type_ident = #core::TypeIdent::Struct(stringify!(#type_ident));
                                invalid.field_ident = #core::FieldIdent::Named(stringify!(#field_ident));

                                let interpreting_args = #core::InterpretingArgs {
                                    interpreter,
                                    data,
                                };

                                let result = #validators::Bundle!(#tokens).run(
                                    validating_args,
                                    invalid,
                                    interpreting_args,
                                    report,
                                );

                                match result {
                                    Ok(#core::ControlFlow::Continue) => {},
                                    ret => return ret,
                                }
                            }});
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

                    for Attribute { meta, .. } in filter_attrs(&attrs) {
                        let tokens = &meta.require_list()?.tokens;

                        validations.push(quote! {{
                            let validating_args = #core::ValidatingArgs {
                                target: &fields.#index,
                                context,
                            };

                            let mut invalid = #core::Invalid::default();
                            invalid.type_ident = #core::TypeIdent::Struct(stringify!(#type_ident));
                            invalid.field_ident = #core::FieldIdent::Unnamed(#index);

                            let interpreting_args = #core::InterpretingArgs {
                                interpreter,
                                data,
                            };

                            let result = #validators::Bundle!(#tokens).run(
                                validating_args,
                                invalid,
                                interpreting_args,
                                report,
                            );

                            match result {
                                Ok(#core::ControlFlow::Continue) => {},
                                ret => return ret,
                            }
                        }});
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

            for Variant {
                ident: variant_ident,
                fields,
                ..
            } in variants
            {
                match fields {
                    Fields::Named(FieldsNamed { named, .. }) => {
                        let mut field_idents = Vec::new();
                        let mut validations = Vec::new();

                        for Field {
                            ident: field_ident,
                            attrs,
                            ..
                        } in named
                        {
                            let field_ident = field_ident.unwrap();

                            field_idents.push(field_ident.clone());

                            for Attribute { meta, .. } in filter_attrs(&attrs) {
                                let tokens = &meta.require_list()?.tokens;

                                validations.push(quote! {{
                                    let validating_args = #core::ValidatingArgs {
                                        target: #field_ident,
                                        context,
                                    };

                                    let mut invalid = #core::Invalid::default();
                                    invalid.type_ident = #core::TypeIdent::Enum(stringify!(#variant_ident), stringify!(#field_ident));
                                    invalid.field_ident = #core::FieldIdent::Named(stringify!(#field_ident));

                                    let interpreting_args = #core::InterpretingArgs {
                                        interpreter,
                                        data,
                                    };

                                    let result = #validators::Bundle!(#tokens).run(
                                        validating_args,
                                        invalid,
                                        interpreting_args,
                                        report,
                                    );

                                    match result {
                                        Ok(#core::ControlFlow::Continue) => {},
                                        ret => return ret,
                                    }
                                }});
                            }
                        }

                        arms.push(quote! {
                            Self::#variant_ident { #(#field_idents,)* } => {
                                #(#validations)*
                            },
                        });
                    }
                    Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                        let mut field_idents = Vec::new();
                        let mut validations = Vec::new();

                        for (index, Field { attrs, .. }) in unnamed.into_iter().enumerate() {
                            field_idents.push(format_ident!("fields_{index}"));

                            let index = Index::from(index);

                            for Attribute { meta, .. } in filter_attrs(&attrs) {
                                let tokens = &meta.require_list()?.tokens;

                                validations.push(quote! {{
                                    let validating_args = #core::ValidatingArgs {
                                        target: fields.#index,
                                        context,
                                    };

                                    let mut invalid = #core::Invalid::default();
                                    invalid.type_ident = #core::TypeIdent::Enum(stringify!(#type_ident), stringify!(#variant_ident));
                                    invalid.field_ident = #core::FieldIdent::Unnamed(#index);

                                    let interpreting_args = #core::InterpretingArgs {
                                        interpreter,
                                        data,
                                    };

                                    let result = #validators::Bundle!(#tokens).run(
                                        validating_args,
                                        invalid,
                                        interpreting_args,
                                        report,
                                    );

                                    match result {
                                        Ok(#core::ControlFlow::Continue) => {},
                                        ret => return ret,
                                    }
                                }});
                            }
                        }

                        arms.push(quote! {
                            Self::#variant_ident(#(#field_idents,)*) => {
                                let fields = (#(#field_idents,)*);

                                #(#validations)*
                            },
                        });
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

    body.push(quote! { Ok(#core::ControlFlow::Continue) });

    Ok(quote! {
        impl #impl_generics #core::Validate for #type_ident #ty_generics #where_clause {
            type Context = #context_type;

            type Error = #error_type;

            fn validate<D, R>(
                &self,
                context: &Self::Context,
                #core::InterpretingArgs { interpreter, data }: #core::InterpretingArgs<D>,
                report: &mut R,
            ) -> Result<#core::ControlFlow, Self::Error>
            where
                R: #core::Report,
            {
                #(#body)*
            }
        }
    })
}
