use proc_macro::TokenStream;

mod path;
mod validate;

/// Derives the `Validate` trait.
#[proc_macro_derive(Validate, attributes(vate))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    validate::expand_derive_validate(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Generates an array of `Accessor` representing the specified path.
#[proc_macro]
pub fn path(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::Expr);
    path::expand_path(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
