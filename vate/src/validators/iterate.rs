use crate::{Accessor, Collector, Exit, Report, Validator};

pub struct Iterate<V>(pub V);

impl<T, D, E, V> Validator<T, D, E> for Iterate<V>
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

pub struct Indexed<V>(pub V);

impl<T, D, E, V> Validator<T, D, E> for Indexed<V>
where
    T: Iterator + Clone,
    V: Validator<T::Item, D, E>,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(validator) = self;

        let mut child_report = Report::new(accessor);

        let child_result = target
            .clone()
            .enumerate()
            .try_for_each(|(index, target_element)| {
                validator.run::<C>(
                    Accessor::Index(index),
                    &target_element,
                    data,
                    &mut child_report,
                )
            });

        let parent_result = parent_report.push_child::<C>(child_report);

        child_result?;
        parent_result
    }
}

pub struct Keyed<V>(pub V);

impl<'a, T, D, E, Key: 'a, Value: 'a, V> Validator<T, D, E> for Keyed<V>
where
    Key: ToString,
    T: Iterator<Item = (&'a Key, &'a Value)> + Clone,
    V: Validator<Value, D, E>,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(validator) = self;

        let mut child_report = Report::new(accessor);

        let child_result = target.clone().try_for_each(|(key, value)| {
            validator.run::<C>(
                Accessor::Key(key.to_string()),
                value,
                data,
                &mut child_report,
            )
        });

        let parent_result = parent_report.push_child::<C>(child_report);

        child_result?;
        parent_result
    }
}
