//! Assorted utilities.

use quote::ToTokens;
use syn::{
    parse::Parse, parse2, punctuated::Punctuated, Attribute, Error, MetaNameValue, Result, Token,
};

/// Filters attributes for the `vate` ident.
pub fn filter_vate_attrs<'a>(
    attrs: impl IntoIterator<Item = &'a Attribute>,
) -> impl Iterator<Item = &'a Attribute> {
    attrs
        .into_iter()
        .filter(|attr| attr.path().is_ident("vate"))
}

/// Parses the specified type `T` from the attributes with the given identifier.
///
/// Returns an error if the attribute is repeated or if parsing fails.
pub fn parse_attrs<'a, T>(
    attrs: impl IntoIterator<Item = &'a Attribute>,
    ident: &'static str,
) -> Result<Option<T>>
where
    T: Parse,
{
    let mut output = None;
    for Attribute { meta, .. } in attrs.into_iter() {
        let arguments = meta
            .require_list()?
            .parse_args_with(Punctuated::<MetaNameValue, Token![,]>::parse_terminated)?;

        for MetaNameValue { path, value, .. } in arguments {
            if !path.is_ident(ident) {
                continue;
            }

            if output.is_some() {
                return Err(Error::new_spanned(
                    path,
                    format!("Attribute \"{ident}\" is defined multiple times"),
                ));
            }

            let parsed = parse2(value.into_token_stream())?;
            output.replace(parsed);
        }
    }
    Ok(output)
}
