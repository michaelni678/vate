use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Error, Expr, ExprField, ExprIndex, ExprLit, ExprPath, Lit, Member, Result};

pub fn expand_path(input: Expr) -> Result<TokenStream2> {
    let mut accessors = Vec::new();
    parse_expr(&input, &mut accessors)?;
    Ok(quote! {
        [#(#accessors),*]
    })
}

fn parse_expr(expr: &Expr, accessors: &mut Vec<TokenStream2>) -> Result<()> {
    match expr {
        Expr::Path(ExprPath { path, .. }) => {
            let ident = path.require_ident()?;
            accessors.push(quote!(::vate::Accessor::Root(stringify!(#ident))));
        }
        Expr::Field(ExprField { base, member, .. }) => {
            parse_expr(base, accessors)?;
            match member {
                Member::Named(ident) => {
                    accessors.push(quote!(::vate::Accessor::Field(stringify!(#ident))))
                }
                Member::Unnamed(index) => {
                    accessors.push(quote!(::vate::Accessor::TupleIndex(#index)))
                }
            }
        }
        Expr::Index(ExprIndex { expr, index, .. }) => {
            parse_expr(expr, accessors)?;
            match &**index {
                Expr::Lit(ExprLit { lit: Lit::Str(str), .. }) => {
                    let key = str.value();
                    accessors.push(quote!(::vate::Accessor::Key(#key.to_string())));
                }
                Expr::Lit(ExprLit { lit: Lit::Int(int), .. }) => {
                    let index = int.base10_parse::<usize>()?;
                    accessors.push(quote!(::vate::Accessor::Index(#index)));
                }
                Expr::Path(ExprPath { path, .. }) => {
                    let ident = path.require_ident()?;
                    accessors.push(quote!(::vate::Accessor::Variant(stringify!(#ident))));
                }
                _ => return Err(Error::new_spanned(index, "Expected `usize` to generate `Accessor::Index` or `&'static str` to generate to generate `Accessor::Key`")),
            }
        }
        _ => return Err(Error::new_spanned(expr, "Unsupported expression")),
    }
    Ok(())
}
