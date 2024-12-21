#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! "Vate" is a powerful and versatile framework for data validation.

extern crate self as vate;

pub mod core;
/// Feature-dependent re-exports.
pub mod extras {
    #[cfg(feature = "regex")]
    pub use regex::Regex;
}
mod internal;
pub mod validators;
