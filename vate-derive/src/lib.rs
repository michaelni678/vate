use proc_macro::TokenStream;

mod path;
mod validate;

/// Derives the `Validate` trait.
/// 
/// # Usage
/// 
/// ## Struct
/// ```rust
/// use vate::{path, Accessor, Everything, Report, StringAlphabetic, Validate};
/// 
/// #[derive(Validate)]
/// struct Example {
///     #[vate(StringAlphabetic)]
///     a: String,
/// }
/// 
/// let example = Example {
///     a: String::from("!!!"),
/// };
/// 
/// let mut report = Report::new(Accessor::Root("example"));
/// let _ = example.validate::<Everything>(&(), &mut report);
/// 
/// assert!(report
///     .is_any_invalid_at_path(path!(example.a))
///     .unwrap());
/// ```
/// 
/// ## Tuple Struct
/// ```rust
/// use vate::{path, Accessor, Everything, Report, StringAlphabetic, Validate};
/// 
/// #[derive(Validate)]
/// struct Example(#[vate(StringAlphabetic)] String);
/// 
/// let example = Example(String::from("!!!"));
/// 
/// let mut report = Report::new(Accessor::Root("example"));
/// let _ = example.validate::<Everything>(&(), &mut report);
/// 
/// assert!(report
///     .is_any_invalid_at_path(path!(example.0))
///     .unwrap());
/// ```
#[proc_macro_derive(Validate, attributes(vate))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    validate::expand_derive_validate(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Generates an array of `Accessor` representing the specified path.
/// 
/// # Usage
/// ```rust
/// use vate::{path, Accessor};
/// 
/// let path = path!(a.b[0].c["Hello"].0);
/// let expected = [Accessor::Root("a"), Accessor::Field("b"), Accessor::Index(0), Accessor::Field("c"), Accessor::Key(String::from("Hello")), Accessor::TupleIndex(0)];
/// 
/// assert_eq!(path, expected);
/// ```
#[proc_macro]
pub fn path(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::Expr);
    path::expand_path(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
