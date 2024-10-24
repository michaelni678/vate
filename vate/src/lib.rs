//! The "vate" crate is a versatile and powerful Rust library designed for validating data
//! structures. It provides a flexible framework for defining and applying custom validators,
//! along with a collection of built-in validators for common use cases.
//!
//! **All publicly accessible components are re-exported at the root of the crate for convenience.** \
//! This allows you to access them directly using `vate::{item}`, where `item` is any publicly exposed component.

extern crate self as vate;

/// Built-in collectors, implementing the [`Collector`] trait.
pub mod collectors;

/// Core components.
pub mod core;

/// Feature-dependent re-exports.
pub mod extras {
    #[cfg(feature = "regex")]
    pub use regex::Regex;
}

/// Utilities for convenience.
pub mod utils {
    pub use vate_macros::path;
}

/// Built-in validators, implementing the [`Validator`] trait.
pub mod validators;

// Hide re-exports of core components in the docs.
#[doc(hidden)]
pub use core::{Accessor, Collector, Exit, Report, Validate, Validator};

// Hide the re-exports of collectors in the docs.
#[doc(hidden)]
pub use collectors::{Everything, FirstInvalidAndPrecedingErrors, InvalidsAndErrors};

// Hide the re-exports of validators in the docs.
#[doc(hidden)]
pub use validators::{
    boolean::{BooleanFalse, BooleanTrue},
    bundle::{Bundle, Bundle2},
    collection::CollectionIterate,
    compare::{Compare, CompareValues},
    iterator::{
        ExactSizeIteratorLengthEquals, IteratorIndexed, IteratorKeyed, IteratorLengthEquals,
    },
    nested::Nested,
    option::{OptionNone, OptionSome, OptionSomeThen},
    string::{
        StringAlphabetic, StringAlphanumeric, StringAscii, StringAsciiAlphabetic, StringAsciiDigit,
        StringLengthEquals, StringLengthRange, StringLowercase, StringUppercase,
    },
};

#[cfg(feature = "regex")]
#[doc(hidden)]
pub use validators::string::StringMatchesRegex;

#[doc(hidden)]
pub use vate_macros::path;

pub use vate_macros::Validate;
