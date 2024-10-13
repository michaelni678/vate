use crate::{
    core::{Accessor, Collector, Exit, Report, Validator},
    Validate,
};

/// Validates the nested implementor of `Validate`.
///
/// Requires the target type to be an implementor of `Validate`.
///
/// # Usage
/// ```rust
/// use vate::{path, Accessor, BooleanTrue, BooleanFalse, Everything, Nested, Report, Validate};
///
/// #[derive(Validate)]
/// struct Example1 {
///     #[vate(Nested)]
///     example2: Example2,
/// }
///
/// #[derive(Validate)]
/// struct Example2 {
///     #[vate(BooleanTrue)]
///     a: bool,
///     #[vate(BooleanTrue)]
///     b: bool,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example1 = Example1 {
///     example2: Example2 {
///         a: true,
///         b: false,
///     },
/// };
///
/// let _ = example1.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example1.example2.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example1.example2.b)).unwrap());
/// ```
pub struct Nested;

impl<T: Validate<Data = D, Error = E>, D, E> Validator<T, D, E> for Nested {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);
        let child_result = target.validate::<C>(data, &mut child_report);
        let parent_result = C::apply(parent_report, child_report);
        child_result?;
        parent_result
    }
}
