use crate::{Accessor, Collector, Exit, Report, Validator};

pub struct CollectionIterate<V>(pub V);

impl<T, D, E, V> Validator<T, D, E> for CollectionIterate<V>
where
    for<'a> &'a T: IntoIterator,
    for<'a> V: Validator<<&'a T as IntoIterator>::IntoIter, D, E>,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(validator) = self;
        validator.run::<C>(accessor, &target.into_iter(), data, parent_report)
    }
}