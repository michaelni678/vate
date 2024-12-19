//! Validating types and traits.

pub mod invalid;
pub mod report;

pub use invalid::*;
pub use report::*;

/// Resources necessary for validating.
pub struct ValidatingArgs<'a, T, C> {
    /// The value that is validated.
    pub target: T,

    /// Custom context for validating.
    pub context: &'a C,
}
