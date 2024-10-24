use std::ops::Deref;

use crate::{Accessor, Collector, Exit, Report, Validator};

pub struct IteratorIndexed<V>(pub V);

impl<T, D, E, V> Validator<T, D, E> for IteratorIndexed<V>
where
    T: Iterator,
    T::Item: Deref,
    for<'a> V: Validator<&'a <T::Item as Deref>::Target, D, E>,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: T,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(validator) = self;

        let mut child_report = Report::new(accessor);

        let child_result = target.enumerate().try_for_each(|(index, target_element)| {
            validator.run::<C>(
                Accessor::Index(index),
                &target_element,
                data,
                &mut child_report,
            )
        });

        let parent_result = C::apply(parent_report, child_report);

        child_result?;
        parent_result
    }
}

pub struct IteratorKeyed<V>(pub V);

impl<'a, T, D, E, Key: 'a, Value: 'a, V> Validator<T, D, E> for IteratorKeyed<V>
where
    Key: ToString,
    T: Iterator<Item = (&'a Key, &'a Value)>,
    V: Validator<&'a Value, D, E>,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        mut target: T,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(validator) = self;

        let mut child_report = Report::new(accessor);

        let child_result = target.try_for_each(|(key, value)| {
            validator.run::<C>(
                Accessor::Key(key.to_string()),
                value,
                data,
                &mut child_report,
            )
        });

        let parent_result = C::apply(parent_report, child_report);

        child_result?;
        parent_result
    }
}

pub struct IteratorLengthEquals(pub usize);

impl<T, D, E> Validator<T, D, E> for IteratorLengthEquals
where
    T: Iterator,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(required_len) = self;
        let target_len = target.count();

        let mut child_report = Report::new(accessor);

        if *required_len == target_len {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
        }

        C::apply(parent_report, child_report)
    }
}

pub struct ExactSizeIteratorLengthEquals(pub usize);

impl<T, D, E> Validator<T, D, E> for ExactSizeIteratorLengthEquals
where
    T: ExactSizeIterator,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(required_len) = self;
        let target_len = target.len();

        let mut child_report = Report::new(accessor);

        if *required_len == target_len {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
        }

        C::apply(parent_report, child_report)
    }
}
