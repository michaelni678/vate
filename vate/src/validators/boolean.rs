use crate::{Accessor, Collector, Exit, Report, Validator};

/// Validates a boolean is `true`.
///
/// # Target Type
/// `bool`
///
/// # Fields / Arguments
/// None
///
/// # Feature Flags
/// None
///
/// # Usage
/// ```rust
/// use vate::{path, Accessor, BooleanTrue, Everything, Report, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(BooleanTrue)]
///     a: bool,
///     #[vate(BooleanTrue)]
///     b: bool,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: true,
///     b: false,
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
pub struct BooleanTrue;

impl<D, E> Validator<bool, D, E> for BooleanTrue {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &bool,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if *target {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message("is false");
        }

        C::apply(parent_report, child_report)
    }
}

/// Validates a boolean is `false`.
///
/// # Target Type
/// `bool`
///
/// # Fields / Arguments
/// None
///
/// # Feature Flags
/// None
///
/// # Usage
/// ```rust
/// use vate::{path, Accessor, BooleanFalse, Everything, Report, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(BooleanFalse)]
///     a: bool,
///     #[vate(BooleanFalse)]
///     b: bool,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: false,
///     b: true,
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
pub struct BooleanFalse;

impl<D, E> Validator<bool, D, E> for BooleanFalse {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &bool,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if !*target {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message("is true");
        }

        C::apply(parent_report, child_report)
    }
}
