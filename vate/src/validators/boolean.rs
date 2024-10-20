use crate::{Accessor, Collector, Exit, Report, Validator};

/// Validates a boolean is `true`.
///
/// Requires the target type to be `bool`.
///
/// # Examples
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
/// assert!(report.is_all_valid_at_path(&path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(&path!(example.b)).unwrap());
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
/// Requires the target type to be `bool`.
///
/// # Examples
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
/// assert!(report.is_all_valid_at_path(&path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(&path!(example.b)).unwrap());
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

/// Set the report validity to the boolean.
/// This validator produces no message due to lack of information to
/// work with.
///
/// Takes a boolean for field `0`.
///
/// # Examples
/// ```rust
/// use vate::{path, Accessor, Boolean, Everything, Report, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     a: i32,
///     b: i32,
///     #[vate(Boolean(a < b), Boolean(*a == 5))]
///     validations: (),
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: 3,
///     b: 4,
///     validations: (),
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// let validities = report.get_validities_at_path(&path!(example.validations));
///
/// assert_eq!(validities.len(), 2);
/// assert!(matches!(validities[0], Ok(true)));
/// assert!(matches!(validities[1], Ok(false)));
/// ```
pub struct Boolean(pub bool);

impl<T, D, E> Validator<T, D, E> for Boolean {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        _target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(validity) = *self;

        let mut child_report = Report::new(accessor);

        child_report.set_validity(Ok(validity));

        C::apply(parent_report, child_report)
    }
}
