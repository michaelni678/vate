use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn expand_path(input: syn::Expr) -> syn::Result<TokenStream2> {
    let mut accessors = Vec::new();
    parse_expr(&input, &mut accessors)?;
    Ok(quote! {
        [#(#accessors),*]
    })
}

fn parse_expr(expr: &syn::Expr, accessors: &mut Vec<TokenStream2>) -> syn::Result<()> {
    match expr {
        syn::Expr::Path(syn::ExprPath { path, .. }) => {
            let ident = path.require_ident()?;
            accessors.push(quote!(::vate::Accessor::Root(stringify!(#ident))));
        }
        syn::Expr::Field(syn::ExprField { base, member, .. }) => {
            parse_expr(base, accessors)?;
            match member {
                syn::Member::Named(ident) => {
                    accessors.push(quote!(::vate::Accessor::Field(stringify!(#ident))))
                }
                syn::Member::Unnamed(index) => {
                    accessors.push(quote!(::vate::Accessor::TupleIndex(#index)))
                }
            }
        }
        syn::Expr::Index(syn::ExprIndex {
            expr,
            index,
            ..
        }) => {
            parse_expr(&expr, accessors)?;
            match &**index {
                syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(str), .. }) => {
                    let key = str.value();
                    accessors.push(quote!(::vate::Accessor::Key(#key.to_string())));
                }
                syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(int), .. }) => {
                    let index = int.base10_parse::<usize>()?;
                    accessors.push(quote!(::vate::Accessor::Index(#index)));
                }
                syn::Expr::Path(syn::ExprPath { path, .. }) => {
                    let ident = path.require_ident()?;
                    accessors.push(quote!(::vate::Accessor::Variant(stringify!(#ident))));
                }
                _ => return Err(syn::Error::new_spanned(index, "Expected `usize` to generate `Accessor::Index` or `&'static str` to generate to generate `Accessor::Key`")),
            }
        }
        _ => return Err(syn::Error::new_spanned(expr, "Unsupported expression")),
    }
    Ok(())
}
