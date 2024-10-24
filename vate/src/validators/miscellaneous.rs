use crate::{Accessor, Collector, Exit, Report, Validator};

// Closure validator, for quick and simple validations.
impl<T, D, E, F> Validator<T, D, E> for F
where
    F: Fn(T, &D) -> Result<bool, E>,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: T,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        child_report.set_validity(self(target, data));

        C::apply(parent_report, child_report)
    }
}
