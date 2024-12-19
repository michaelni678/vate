//! Core types and traits.

pub mod interpreting;
pub mod validating;

pub use interpreting::*;
pub use validating::*;

// Re-export Validate derive macro.
pub use vate_macros::Validate;

/// Allows the implementor to be validated.
pub trait Validate {
    /// Custom context type.
    type Context;

    /// Custom error type.
    type Error;

    /// Validate an instance of the implementor.
    fn validate<D, R>(
        &self,
        context: &Self::Context,
        interpreting_args: InterpretingArgs<D>,
        report: &mut R,
    ) -> Result<ControlFlow, Self::Error>
    where
        R: Report;
}

/// Defines a validator.
pub trait Validator<T, C, E> {
    /// Runs the validator.
    fn run<D, R>(
        self,
        validating_args: ValidatingArgs<T, C>,
        invalid: Invalid,
        interpreting_args: InterpretingArgs<D>,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report;
}

/// The control flow for validators.
#[must_use]
pub enum ControlFlow {
    Continue,
    Exit,
}
