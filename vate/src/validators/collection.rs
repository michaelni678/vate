use crate::{Accessor, Collector, Exit, Report, Validator};

/// Runs the inner validator, converting target to an iterator.
///
/// # Target Type
/// Implementors of `for<'a> &'a T: IntoIterator`.
///
/// # Fields / Arguments
/// `0`: the inner validator.
///
/// # Feature Flags
/// None
///
/// # Usage
/// See the usages of the iterator validators, such as the `IteratorIndexed` validator.
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
