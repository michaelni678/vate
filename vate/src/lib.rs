//! The "vate" crate is a versatile and powerful Rust library designed for validating data
//! structures. It provides a flexible framework for defining and applying custom validators,
//! along with a collection of built-in validators for common use cases.
//!
//! **All publicly accessible components are re-exported at the root of the crate for convenience.** \
//! This allows you to access them directly using `vate::{item}`, where `item` is any publicly exposed component.
//!
//! # Usage
//! ```rust
//! use vate::*;
//!
//! #[derive(Validate)]
//! struct CreateUser {
//!     #[vate(StringAlphanumeric, StringLengthRange::Chars { min: 4, max: 20 })]
//!     username: String,
//!     #[vate(StringAscii, StringLengthRange::Chars { min: 8, max: usize::MAX })]
//!     password: String,
//!     #[vate(Compare!( == &self.password ))]
//!     confirm_password: String,
//! }
//!
//! let create_user = CreateUser {
//!     username: String::from("username"),
//!     password: String::from("password"),
//!     confirm_password: String::from("password"),
//! };
//!
//! let mut report = Report::new(Accessor::Root("create_user"));
//! let _ = create_user.validate::<InvalidsAndErrors>(&(), &mut report);
//!
//! assert!(report.is_valid());
//! ```

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
    pub use vate_derive::path;
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
    boolean::{Boolean, BooleanFalse, BooleanTrue},
    bundle::{Bundle, Bundle2},
    collection::CollectionIterate,
    compare::{
        Compare, CompareEqualTo, CompareGreaterThan, CompareGreaterThanOrEqualTo, CompareLessThan,
        CompareLessThanOrEqualTo, CompareNotEqualTo,
    },
    iterator::{
        ExactSizeIteratorLengthEquals, IteratorIndexed, IteratorKeyed, IteratorLengthEquals,
    },
    nested::Nested,
    option::{OptionNone, OptionSome, OptionSomeThen},
    string::{
        StringAlphabetic, StringAlphanumeric, StringAscii, StringAsciiDigit, StringLengthEquals,
        StringLengthRange, StringLowercase, StringUppercase,
    },
};

#[cfg(feature = "password")]
#[doc(hidden)]
pub use validators::miscellaneous::PasswordStrong;

#[cfg(feature = "regex")]
#[doc(hidden)]
pub use validators::string::StringMatchesRegex;

#[doc(hidden)]
pub use vate_derive::path;

pub use vate_derive::Validate;
