use crate::extras::Regex;
use crate::{Accessor, Collector, Exit, Report, Validator};

/// # Description
/// Validates a string consists of only alphabetic characters.
/// # Target Type
/// Implementors of `AsRef<str>`.
/// # Arguments
/// None
/// # Feature Flag
/// None
/// # Usage
/// ```rust
/// use vate::{path, Accessor, Everything, Report, StringAlphabetic, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(StringAlphabetic)]
///     a: &'static str,
///     #[vate(StringAlphabetic)]
///     b: &'static str,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: "helloworld",
///     b: "hellow0rld",
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
pub struct StringAlphabetic;

impl<T: AsRef<str>, D, E> Validator<T, D, E> for StringAlphabetic {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if target.as_ref().chars().all(char::is_alphabetic) {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message("contains non-alphabetic characters");
        }

        C::apply(parent_report, child_report)
    }
}

/// # Description
/// Validates a string consists of only alphanumeric characters.
/// # Target Type
/// Implementors of `AsRef<str>`.
/// # Arguments
/// None
/// # Feature Flag
/// None
/// # Usage
/// ```rust
/// use vate::{path, Accessor, Everything, Report, StringAlphanumeric, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(StringAlphanumeric)]
///     a: &'static str,
///     #[vate(StringAlphanumeric)]
///     b: &'static str,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: "hellow0rld",
///     b: "hellow0rld!",
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
pub struct StringAlphanumeric;

impl<T: AsRef<str>, D, E> Validator<T, D, E> for StringAlphanumeric {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if target.as_ref().chars().all(char::is_alphanumeric) {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message("contains non-alphanumeric characters");
        }

        C::apply(parent_report, child_report)
    }
}

/// # Description
/// Validates a string consists of only ascii characters.
/// # Target Type
/// Implementors of `AsRef<str>`.
/// # Arguments
/// None
/// # Feature Flag
/// None
/// # Usage
/// ```rust
/// use vate::{path, Accessor, Everything, Report, StringAscii, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(StringAscii)]
///     a: &'static str,
///     #[vate(StringAscii)]
///     b: &'static str,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: "hellow0rld!",
///     b: "hellow0rld\u{00a1}",
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
pub struct StringAscii;

impl<T: AsRef<str>, D, E> Validator<T, D, E> for StringAscii {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if target.as_ref().is_ascii() {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message("contains non-ascii characters");
        }

        C::apply(parent_report, child_report)
    }
}

/// # Description
/// Validates a string's length equals argument `0`.
/// # Target Type
/// Implementors of `AsRef<str>`.
/// # Arguments
/// `0`: the expected length.
/// # Feature Flag
/// None
/// # Usage
/// ```rust
/// use vate::{path, Accessor, Everything, Report, StringLengthEquals, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(StringLengthEquals::Chars(9))]
///     a: &'static str,
///     #[vate(StringLengthEquals::Bytes(9))]
///     b: &'static str,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: "hello, \u{4E16}\u{754C}", // \u{4E16}\u{754C} is 9 characters but 13 bytes.
///     b: "hello, \u{4E16}\u{754C}",
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
pub enum StringLengthEquals {
    Bytes(usize),
    Chars(usize),
}

impl<T: AsRef<str>, D, E> Validator<T, D, E> for StringLengthEquals {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let s = target.as_ref();
        let (unit, required_len, target_len) = match *self {
            Self::Bytes(required_len) => ("byte", required_len, s.len()),
            Self::Chars(required_len) => ("character", required_len, s.chars().count()),
        };

        let mut child_report = Report::new(accessor);

        if required_len == target_len {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message(format!("is not {required_len} {unit}s long"));
        }

        C::apply(parent_report, child_report)
    }
}

/// # Description
/// Validates a string's length is in the inclusive range of arguments `min` and `max`.
/// # Target Type
/// Implementors of `AsRef<str>`.
/// # Arguments
/// `min`: the minimum length.
/// `max`: the maximum length.
/// # Feature Flag
/// None
/// # Usage
/// ```rust
/// use vate::{path, Accessor, Everything, Report, StringLengthRange, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(StringLengthRange::Chars { min: 5, max: 9 })]
///     a: &'static str,
///     #[vate(StringLengthRange::Bytes { min: 5, max: 9 })]
///     b: &'static str,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: "hello, \u{4E16}\u{754C}", // \u{4E16}\u{754C} is 9 characters but 13 bytes.
///     b: "hello, \u{4E16}\u{754C}",
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
pub enum StringLengthRange {
    Bytes { min: usize, max: usize },
    Chars { min: usize, max: usize },
}

impl<T: AsRef<str>, D, E> Validator<T, D, E> for StringLengthRange {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let s = target.as_ref();
        let (unit, min, max, target_len) = match *self {
            Self::Bytes { min, max } => ("byte", min, max, s.len()),
            Self::Chars { min, max } => ("character", min, max, s.chars().count()),
        };

        let mut child_report = Report::new(accessor);

        if target_len >= min && target_len <= max {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message(format!("is not between {min} and {max} {unit}s long"));
        }

        C::apply(parent_report, child_report)
    }
}

/// # Description
/// Validates a string matches the regex at argument `0`.
/// # Target Type
/// Implementors of `AsRef<str>`.
/// # Arguments
/// `0`: the regex to match against.
/// # Feature Flag
/// None
/// # Usage
/// ```rust
/// use once_cell::sync::Lazy;
/// use vate::{extras::Regex, path, Accessor, Everything, Report, StringMatchesRegex, Validate};
/// 
/// static LOWERCASE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^[a-z]+$").unwrap());
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(StringMatchesRegex(&LOWERCASE_REGEX))]
///     a: &'static str,
///     #[vate(StringMatchesRegex(&LOWERCASE_REGEX))]
///     b: &'static str,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: "hello",
///     b: "Hello",
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
pub struct StringMatchesRegex<'a>(pub &'a Regex);

impl<'a, T: AsRef<str>, D, E> Validator<T, D, E> for StringMatchesRegex<'a> {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(regex) = self;
        let target = target.as_ref();

        let mut child_report = Report::new(accessor);

        if regex.is_match(target) {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message(format!(
                "is \"{target}\", which does not match regex {regex}"
            ));
        }

        C::apply(parent_report, child_report)
    }
}
