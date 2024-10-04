use std::ops::Deref;

use crate::{Accessor, Collector, Exit, Report, Validator};

pub struct IteratorIndexed<V>(pub V);

impl<T, D, E, V> Validator<T, D, E> for IteratorIndexed<V>
where
    T: Iterator + Clone,
    T::Item: Deref,
    <<T as Iterator>::Item as Deref>::Target: Sized,
    V: Validator<<T::Item as Deref>::Target, D, E>,
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

pub struct IteratorKeyed<V>(pub V);

impl<'a, T, D, E, Key: 'a, Value: 'a, V> Validator<T, D, E> for IteratorKeyed<V>
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

pub struct IteratorKeyedPair<V>(pub V);

impl<'a, T, D, E, Key: 'a, Value: 'a, V> Validator<T, D, E> for IteratorKeyedPair<V>
where
    Key: ToString,
    T: Iterator<Item = (&'a Key, &'a Value)> + Clone,
    V: Validator<(&'a Key, &'a Value), D, E>,
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
                &(key, value),
                data,
                &mut child_report,
            )
        });

        let parent_result = parent_report.push_child::<C>(child_report);

        child_result?;
        parent_result
    }
}

pub struct IteratorLengthEquals(pub usize);

impl<T, D, E> Validator<T, D, E> for IteratorLengthEquals
where
    T: Iterator + Clone,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(required_len) = self;
        let target_len = target.clone().count();

        let mut child_report = Report::new(accessor);

        if *required_len == target_len {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message = format!("is not {required_len} items long");
        }

        parent_report.push_child::<C>(child_report)
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
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(required_len) = self;
        let target_len = target.len();

        let mut child_report = Report::new(accessor);

        if *required_len == target_len {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message = format!("is not {required_len} items long");
        }

        parent_report.push_child::<C>(child_report)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use vate::{
        path, Accessor, CollectionIterate, Compare, Everything, ExactSizeIteratorLengthEquals,
        IteratorIndexed, IteratorKeyed, IteratorLengthEquals, Report, Validate,
    };

    #[test]
    fn iterator_indexed() {
        #[derive(Validate)]
        struct Example {
            #[vate(CollectionIterate(IteratorIndexed(Compare!( > 0 ))))]
            v: Vec<u32>,
        }
        let example = Example {
            v: vec![0, 1, 2, 3, 4],
        };
        let mut report = Report::new(Accessor::Root("example"));
        let _ = example.validate::<Everything>(&(), &mut report);
        assert!(report.is_invalid_at_path(path!(example.v[0])).unwrap());
        assert!(report.is_valid_at_path(path!(example.v[1])).unwrap());
        assert!(report.is_valid_at_path(path!(example.v[2])).unwrap());
        assert!(report.is_valid_at_path(path!(example.v[3])).unwrap());
        assert!(report.is_valid_at_path(path!(example.v[4])).unwrap());
    }

    #[test]
    fn iterator_keyed() {
        #[derive(Validate)]
        struct Example {
            #[vate(CollectionIterate(IteratorKeyed(Compare!( > 0 ))))]
            hm: HashMap<&'static str, u32>,
        }
        let example = Example {
            hm: HashMap::from([("a", 0), ("b", 1), ("c", 2), ("d", 3), ("e", 4)]),
        };
        let mut report = Report::new(Accessor::Root("example"));
        let _ = example.validate::<Everything>(&(), &mut report);
        assert!(report.is_invalid_at_path(path!(example.hm["a"])).unwrap());
        assert!(report.is_valid_at_path(path!(example.hm["b"])).unwrap());
        assert!(report.is_valid_at_path(path!(example.hm["c"])).unwrap());
        assert!(report.is_valid_at_path(path!(example.hm["d"])).unwrap());
        assert!(report.is_valid_at_path(path!(example.hm["e"])).unwrap());
    }

    #[test]
    fn iterator_keyed_pair() {
        todo!();
    }

    #[test]
    fn iterator_length_equals() {
        #[derive(Validate)]
        struct Example {
            #[vate(CollectionIterate(IteratorLengthEquals(5)))]
            v: Vec<u32>,
        }
        let example = Example {
            v: vec![1, 2, 3, 4, 5],
        };
        let mut report = Report::new(Accessor::Root("example"));
        let _ = example.validate::<Everything>(&(), &mut report);
        assert!(report.is_valid_at_path(path!(example)).unwrap());
    }

    #[test]
    fn exact_size_iterator_length_equals() {
        #[derive(Validate)]
        struct Example {
            #[vate(CollectionIterate(ExactSizeIteratorLengthEquals(5)))]
            v: Vec<u32>,
        }
        let example = Example {
            v: vec![1, 2, 3, 4, 5],
        };
        let mut report = Report::new(Accessor::Root("example"));
        let _ = example.validate::<Everything>(&(), &mut report);
        assert!(report.is_valid_at_path(path!(example)).unwrap());
    }
}
