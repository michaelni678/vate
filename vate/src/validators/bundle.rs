use crate::{Accessor, Collector, Exit, Report, Validator};

/// Runs the two inner validators.
///
/// # Target Type
/// Generic
///
/// # Fields / Arguments
/// `0`: the first inner validator.
/// `1`: the second inner validator.
///
/// # Feature Flags
/// None
///
/// # Usage
/// ```rust
/// use vate::{path, Accessor, Bundle2, Compare, Everything, Report, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(Bundle2(Compare!( > 1 ), Compare!( < 3 )))]
///     a: i32,
///     #[vate(Bundle2(Compare!( > 1 ), Compare!( < 3 )))]
///     b: i32,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: 2,
///     b: 5,
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
pub struct Bundle2<V1, V2>(pub V1, pub V2);

impl<T, D, E, V1, V2> Validator<T, D, E> for Bundle2<V1, V2>
where
    V1: Validator<T, D, E>,
    V2: Validator<T, D, E>,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(validator1, validator2) = self;

        validator1.run::<C>(accessor.clone(), target, data, parent_report)?;
        validator2.run::<C>(accessor, target, data, parent_report)
    }
}

// Note: This macro's name is `UpperCamelCase`, which doesn't conform with typical macro naming conventions.
// However, it was done to match the naming convention of normal validators.
/// Runs the inner validators.
///
/// # Target Type
/// Generic
///
/// # Fields / Arguments
/// A comma-separated list of validators.
///
/// # Feature Flags
/// None
///
/// # Usage
/// ```rust
/// use vate::{path, Accessor, Bundle, Compare, Everything, Report, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(Bundle!(Compare!( > 1 ), Compare!( < 3 )))]
///     a: i32,
///     #[vate(Bundle!(Compare!( > 1 ), Compare!( < 3 )))]
///     b: i32,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: 2,
///     b: 5,
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
#[macro_export]
macro_rules! Bundle {
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {
        $crate::Bundle2($a, $b)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::Bundle2($a, Bundle!($($rest)*))
    };
}
