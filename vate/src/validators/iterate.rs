use std::marker::PhantomData;

use crate::{Accessor, Collector, Exit, Report, Validator};

pub struct Iterate<M, V> {
    pub mapper: M,
    pub validator: V,
}

impl<T, D, E, M, V> Validator<T, D, E> for Iterate<M, V>
where
    for<'a> &'a T: IntoIterator<Item = &'a <M as IteratorMapper<'a>>::Before>,
    for<'a> M: IteratorMapper<'a>,
    for<'a> V: Validator<&'a <M as IteratorMapper<'a>>::After, D, E>,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self { mapper, validator } = self;

        let mut child_report = Report::new(accessor.clone());

        let child_result = mapper.map(target.into_iter()).try_for_each(|(accessor, after)| {
            validator.run::<C>(accessor, &after, data, &mut child_report)
        });

        let parent_result = parent_report.push_child::<C>(child_report);
        child_result?;
        parent_result
    }
}

pub trait IteratorMapper<'a> {
    type Before: 'a;
    type After: 'a;
    fn map(&self, iterator: impl Iterator<Item = &'a Self::Before>) -> impl Iterator<Item = (Accessor, &'a Self::After)>;
}

pub struct IndexIteratorMapper<T>(PhantomData<T>);

impl<T> IndexIteratorMapper<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'a, T: 'a> IteratorMapper<'a> for IndexIteratorMapper<T> {
    type Before = T;
    type After = T;
    fn map(&self, iterator: impl Iterator<Item = &'a Self::Before>) -> impl Iterator<Item = (Accessor, &'a Self::After)> {
        iterator.enumerate().map(|(index, after)| (Accessor::Index(index), after))
    }
}
