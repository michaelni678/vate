mod collectors;
mod core;
mod validators;

pub use collectors::{FirstInvalidAndPrecedingErrors, InvalidsAndErrors};
pub use core::{Accessor, Collector, Exit, Report, Validate, Validator};
pub use validators::{
    bundle::Bundle2,
    collection::CollectionIterate,
    compare::{
        CompareEqualTo, CompareGreaterThan, CompareGreaterThanOrEqualTo, CompareLessThan,
        CompareLessThanOrEqualTo, CompareNotEqualTo,
    },
    iterator::{IteratorIndexed, IteratorKeyed, IteratorLengthEquals, ExactSizeIteratorLengthEquals},
    nested::Nested,
    option::{OptionNone, OptionSome, OptionSomeThen},
    string::{
        StringAlphabetic, StringAlphanumeric, StringAscii, StringLengthEquals, StringLengthRange,
    },
};
pub use vate_derive::{path, Validate};
