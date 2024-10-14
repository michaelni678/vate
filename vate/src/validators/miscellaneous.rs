#[allow(unused_imports)]
use crate::{Accessor, Collector, Exit, Report, Validator};

/// Validates a password is strong using zxcvbn.
/// 
/// Requires the target type to be an implementor of `AsRef<str>`.
/// 
/// Takes an optional list of inputs like username, email, etc. to 
/// factor into the password strength calculation.
/// 
/// Enabled with the `password` feature.
/// 
/// # Examples
/// ```rust
/// use vate::{path, Accessor, Everything, PasswordStrong, Report, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     username: String,
/// 
///     #[vate(PasswordStrong([]))]
///     a: &'static str,
///     #[vate(PasswordStrong([]))]
///     b: &'static str,
///     #[vate(PasswordStrong([&self.username]))]
///     c: &'static str,
///     #[vate(PasswordStrong([&self.username]))]
///     d: &'static str,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     username: String::from("crate_vate_is_great_123"),
/// 
///     a: "password123", // A horrible password in general.
///     b: "crate_vate_is_great_12345!", // Strong password (this does not check inputs).
///     c: "crate_vate_is_great_12345!", // Weak password. Too similar to an input (username).
///     d: "a9012ckas9!@381j!@#sjai12d!@#)(asidj", // Strong password.
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
/// 
/// println!("{report:#?}");
///
/// assert!(report.is_any_invalid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_all_valid_at_path(path!(example.b)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.c)).unwrap());
/// assert!(report.is_all_valid_at_path(path!(example.d)).unwrap());
#[cfg(feature = "password")]
pub struct PasswordStrong<'a, const N: usize>(pub [&'a str; N]);

#[cfg(feature = "password")]
impl<'a, const N: usize, T: AsRef<str>, D, E> Validator<T, D, E> for PasswordStrong<'a, N> {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(inputs) = self;

        let mut child_report = Report::new(accessor);

        let entropy = zxcvbn::zxcvbn(target.as_ref(), inputs);

        if entropy.score() >= zxcvbn::Score::Three {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message("is too weak");
            // TODO: `entropy.feedback()` gives some feedback on the password.
            // This validator does not include the feedback in the message,
            // but maybe it should?
            // if let Some(feedback) = entropy.feedback() {
            //     child_report.set_message(format!("is too weak. {feedback}"));
            // }
        }

        C::apply(parent_report, child_report)
    }
}