#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

//! The "vate" crate is a versatile and powerful Rust library designed for validating data
//! structures. It provides a flexible framework for defining and applying custom validators,
//! along with a collection of built-in validators for common use cases.
//!
//! # Validators
//!
//! ### Boolean
//! - [`BooleanFalse`]
//! - [`BooleanTrue`]
//!
//! ### Bundle
//! - [`Bundle`]
//! - [`Bundle2`]
//!
//! ### Collection
//! - [`CollectionIterate`]
//!
//! ### Compare
//! - [`CompareEqualTo`]
//! - [`CompareGreaterThan`]
//! - [`CompareGreaterThanOrEqualTo`]
//! - [`CompareLessThan`]
//! - [`CompareLessThanOrEqualTo`]
//! - [`CompareNotEqualTo`]
//!
//! ### Iterator
//! - [`ExactSizeIteratorLengthEquals`]
//! - [`IteratorIndexed`]
//! - [`IteratorKeyed`]
//! - [`IteratorLengthEquals`]
//!
//! ### Nested
//! - [`Nested`]
//!
//! ### Option
//! - [`OptionNone`]
//! - [`OptionSome`]
//! - [`OptionSomeThen`]
//!
//! ### String
//! - [`StringAlphabetic`]
//! - [`StringAlphanumeric`]
//! - [`StringAscii`]
//! - [`StringLengthEquals`]
//! - [`StringLengthRange`]
//! - [`StringLowercase`]
//! - [`StringUppercase`]
//!
//! # Collectors
//! - [`InvalidsAndErrors`]
//! - [`FirstInvalidAndPrecedingErrors`]
//! - [`Everything`]
//!
//! # Utility
//! - [`path`]

extern crate self as vate;

pub mod collectors;
pub mod core;
pub mod validators;
pub mod extras {
    #[cfg(feature = "regex")]
    pub use regex::Regex;
}

// Inline the re-exports of core components in the docs.
#[doc(inline)]
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
    compare::{
        Compare,
        CompareEqualTo, CompareGreaterThan, CompareGreaterThanOrEqualTo, CompareLessThan,
        CompareLessThanOrEqualTo, CompareNotEqualTo,
    },
    iterator::{
        ExactSizeIteratorLengthEquals, IteratorIndexed, IteratorKeyed, IteratorLengthEquals,
    },
    nested::Nested,
    option::{OptionNone, OptionSome, OptionSomeThen},
    string::{
        StringAlphabetic, StringAlphanumeric, StringAscii, StringLengthEquals, StringLengthRange,
        StringLowercase, StringUppercase,
    },
};

#[cfg(feature = "regex")]
#[doc(hidden)]
pub use validators::string::StringMatchesRegex;

pub use vate_derive::{path, Validate};