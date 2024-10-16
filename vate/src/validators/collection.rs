use crate::{Accessor, Collector, Exit, Report, Validator};

/// Converts target to an iterator and runs the inner validator.
///
/// Requires the target type to be an implementor of `for<'a> &'a T: IntoIterator`.
///
/// Takes an implementor of `Validator` for field `0`.
///
/// # Examples
/// See the usages of the iterator validators, such as the [`crate::IteratorIndexed`] validator.
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
