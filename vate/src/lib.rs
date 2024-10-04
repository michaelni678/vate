extern crate self as vate;

mod collectors;
mod core;
mod validators;

pub use collectors::{Everything, FirstInvalidAndPrecedingErrors, InvalidsAndErrors};
pub use core::{Accessor, Collector, Exit, Report, Validate, Validator};
pub use validators::{
    boolean::{False, True},
    bundle::Bundle2,
    collection::CollectionIterate,
    compare::{
        CompareEqualTo, CompareGreaterThan, CompareGreaterThanOrEqualTo, CompareLessThan,
        CompareLessThanOrEqualTo, CompareNotEqualTo,
    },
    iterator::{
        ExactSizeIteratorLengthEquals, IteratorIndexed, IteratorKeyed,
        IteratorLengthEquals,
    },
    nested::Nested,
    option::{OptionNone, OptionSome, OptionSomeThen},
    string::{
        StringAlphabetic, StringAlphanumeric, StringAscii, StringLengthEquals, StringLengthRange,
    },
};
pub use vate_derive::{path, Validate};
