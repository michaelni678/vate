use crate::{
    core::{Accessor, Collector, Exit, Report, Validator},
    Validate,
};

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
