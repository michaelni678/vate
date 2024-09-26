use proc_macro::TokenStream;

mod validate;

#[proc_macro_derive(Validate, attributes(vate))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    validate::expand_derive_validate(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
