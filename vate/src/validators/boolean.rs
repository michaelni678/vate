use crate::{Accessor, Collector, Exit, Report, Validator};

/// ### Description
/// Validates a boolean is `true`.
/// ### Usage
/// ```rust
/// use vate::{Validate, BooleanTrue, Everything, Report, Accessor, path};
/// 
/// #[derive(Validate)]
/// struct Example {
///     #[vate(BooleanTrue)]
///     a: bool,
/// }
/// 
/// let mut report = Report::new(Accessor::Root("example"));
/// 
/// let example = Example {
///     a: true,
/// };
/// 
/// let _ = example.validate::<Everything>(&(), &mut report); 
/// 
/// assert!(report.is_valid_at_path(path!(example.a)).unwrap());
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
