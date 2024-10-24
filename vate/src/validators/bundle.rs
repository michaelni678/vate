use crate::{Accessor, Collector, Exit, Report, Validator};

/// Runs the validators supplied to the macro.
///
/// # Examples
/// ```rust
/// use vate::{path, Accessor, Bundle, Everything, OptionSomeThen, Report, StringAlphanumeric, StringAsciiAlphabetic, StringLengthRange, Validate};
///
/// /// Social media accounts.
/// #[derive(Validate)]
/// struct Socials {
///     /// "Instaounce" usernames are alphanumeric and between 4 and 32 characters.
///     #[vate(OptionSomeThen(Bundle!(StringAlphanumeric, StringLengthRange::Chars { min: 4, max: 32 })))]
///     instaounce: Option<String>,
///     /// "LinkedOut" usernames are ascii-alphabetic and between 8 and 30 characters.
///     #[vate(OptionSomeThen(Bundle!(StringAsciiAlphabetic, StringLengthRange::Chars { min: 8, max: 30 })))]
///     linked_out: Option<String>,
///     /// "Blueit" usernames are between 5 and 20 characters.
///     #[vate(OptionSomeThen(Bundle!(StringLengthRange::Chars { min: 5, max: 20 })))]
///     blueit: Option<String>,
/// }
///
/// let socials = Socials {
///     instaounce: Some(String::from("vatecrateisgreat88")),
///     linked_out: None,
///     blueit: Some(String::from("anonymous.platypus.55")), // Not between 5 and 20 characters.
/// };
///
/// let mut report = Report::new(Accessor::Root("socials"));
///
/// let _ = socials.validate::<Everything>(&(), &mut report);
///
/// assert_eq!(report.get_leaves().count(), 3);
///
/// assert_eq!(report.get_children_at_path(&path!(socials.instaounce)).count(), 2);
/// assert!(report.get_children_at_path(&path!(socials.instaounce)).all(|child| child.is_valid()));
///
/// assert_eq!(report.get_children_at_path(&path!(socials.blueit)).count(), 1);
/// assert!(report.get_children_at_path(&path!(socials.blueit)).all(|child| child.is_invalid()));
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! _Bundle {
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {
        $crate::Bundle2($a, $b)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::Bundle2($a, ::vate::Bundle!($($rest)*))
    };
}

#[doc(inline)]
pub use _Bundle as Bundle;

/// Runs the two validators in fields `0` and `1`, forwarding the accessor.
///
/// There is not really any reason to use this validator directly. Use the [`Bundle`] macro instead.
///
/// [`Bundle2`] can be used to represent any number of validators. The way the [`Bundle`] macro works is by
/// expanding the list of validators into a combination of [`Bundle2`]s like so:
/// ```text
/// Bundle2(ValidatorA, Bundle2(ValidatorB, Bundle2(ValidatorC, ValidatorD)))
/// ```
///
/// # Examples
/// See the `Bundle` macro.
pub struct Bundle2<V1, V2>(pub V1, pub V2);

impl<T, D, E, V1, V2> Validator<T, D, E> for Bundle2<V1, V2>
where
    T: Copy,
    V1: Validator<T, D, E>,
    V2: Validator<T, D, E>,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: T,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(validator1, validator2) = self;

        validator1.run::<C>(accessor.clone(), target, data, parent_report)?;
        validator2.run::<C>(accessor, target, data, parent_report)
    }
}
