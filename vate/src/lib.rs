#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

//! The "vate" crate is a versatile and powerful Rust library designed for validating data
//! structures. It provides a flexible framework for defining and applying custom validators,
//! along with a collection of built-in validators for common use cases. Whether you need to
//! validate simple fields, nested structures, or collections, "vate" allows you to create
//! reusable and composable validation logic.

extern crate self as vate;

/// Built-in collectors.
mod collectors;

/// Core functionality.
mod core;

/// Built-in validators.
mod validators;

#[doc(inline)]
pub use collectors::{Everything, FirstInvalidAndPrecedingErrors, InvalidsAndErrors};

#[doc(inline)]
pub use core::{Accessor, Collector, Exit, Report, Validate, Validator};

#[doc(inline)]
pub use validators::{
    boolean::{BooleanFalse, BooleanTrue},
    bundle::Bundle2,
    collection::CollectionIterate,
    compare::{
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
pub use vate_derive::{path, Validate};

#[cfg(feature = "regex")]
#[doc(hidden)]
pub use validators::string::StringMatchesRegex;

pub mod extras {
    #[cfg(feature = "regex")]
    pub use regex::Regex;
}
