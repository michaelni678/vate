use std::fmt;

/// Contains information regarding validations.
#[derive(Default, Clone)]
pub struct Invalid<'a> {
    /// The type ident.
    pub type_ident: TypeIdent,

    /// The field ident.
    pub field_ident: FieldIdent,

    /// The validator idents.
    pub validator_idents: Vec<ValidatorIdent>,

    /// The details for the validators.
    pub all_validator_details: Vec<Details<'a>>,
}

impl<'a> Invalid<'a> {
    pub fn push(mut self, validator_ident: ValidatorIdent, validator_details: Details<'a>) -> Self {
        self.validator_idents.push(validator_ident);
        self.all_validator_details.push(validator_details);
        self
    }
}

/// A type ident.
#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TypeIdent {
    /// An unspecified field ident.
    ///
    /// This is the default, but should be changed.
    #[default]
    Unspecified,

    /// A struct's ident.
    ///
    /// For example, `X` in `struct X`.
    Struct(&'static str),

    /// An enum's ident and variant.
    ///
    /// For example, `X` and `Y` in `enum X { Y }`.
    Enum(&'static str, &'static str),
}

impl fmt::Display for TypeIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unspecified => write!(f, "<unspecified type ident>"),
            Self::Struct(ident) => write!(f, "{ident}"),
            Self::Enum(ident, variant) => write!(f, "{ident}::{variant}"),
        }
    }
}

/// A field ident.
#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FieldIdent {
    /// An unspecified field ident.
    ///
    /// This is the default, but should be changed.
    #[default]
    Unspecified,

    /// A named field ident.
    ///
    /// For example, `a` in `struct X { a: i32 }`.
    Named(&'static str),

    /// An unnamed field ident.
    ///
    /// For example, `0` in `struct X(i32)`.
    Unnamed(usize),
}

impl fmt::Display for FieldIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unspecified => write!(f, "<unspecified field ident>"),
            Self::Named(ident) => write!(f, "{ident}"),
            Self::Unnamed(ident) => write!(f, "{ident}"),
        }
    }
}

/// A validator ident.
#[derive(Default, Hash, Clone, Copy, PartialEq, Eq)]
pub struct ValidatorIdent {
    /// The name of the validator.
    pub name: &'static str,

    /// The variant of the validator.
    pub variant: u8,
}

impl ValidatorIdent {
    /// Create an instance with only name.
    pub fn set_name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }
}

/// A validator's details.
#[derive(Default, Clone)]
pub struct Details<'a>(Vec<&'a dyn ToString>);

impl<'a> Details<'a> {
    /// Set a detail at an index.
    pub fn set_detail(mut self, index: usize, detail: &'a dyn ToString) -> Self {
        if index >= self.0.len() {
            // OPTIMIZE: this may resize multiple times depending on order that
            // details are added. Maybe reserve extra capacity or something.
            self.0.resize_with(index + 1, || &"");
        }

        self.0[index] = detail;
        self
    }

    /// Get a detail at an index as a string.
    ///
    /// Panics if out of bounds.
    pub fn get_detail(&self, index: usize) -> String {
        self.0[index].to_string()
    }
}
