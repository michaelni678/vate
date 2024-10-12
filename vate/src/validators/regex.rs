use crate::{Accessor, Collector, Exit, Report, Validator};

/// Validates a string matches the regex at field `0`.
/// 
///
/// # Target Type
/// Implementors of `AsRef<str>`.
/// 
///
/// # Fields / Arguments
/// `0`: the regex to match against.
/// 
///
/// # Feature Flags
/// `regex`
/// 
///
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
///     b: "hEllo",
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
pub struct StringMatchesRegex<'a>(pub &'a crate::extras::Regex);

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
