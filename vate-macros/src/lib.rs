//! Procedural macros for "Vate".

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

mod utility;
mod validate;

/// Derives the `Validate` trait.
#[proc_macro_derive(Validate, attributes(vate))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    validate::expand_derive_validate(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
