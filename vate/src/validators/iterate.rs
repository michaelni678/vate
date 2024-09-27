use std::collections::{BTreeMap, HashMap};

use crate::{Accessor, Collector, Exit, Report, Validator};

pub struct Iterate<V>(pub V);

impl<T, D, E, V> Validator<T, D, E> for Iterate<V> 
where 
    T: ToAccessorIterator,
    V: Validator<T::Blob, D, E>,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(validator) = self;

        let mut child_report = Report::new(accessor.clone());

        let child_result =
            target.accessor_iter()
                .try_for_each(|(accessor, blob)| {
                    validator.run::<C>(
                        accessor,
                        &blob,
                        data,
                        &mut child_report,
                    )
                });

        let parent_result = parent_report.push_child::<C>(child_report);
        
        child_result?;
        parent_result
    }
}

pub trait ToAccessorIterator {
    type Blob;
    fn accessor_iter(&self) -> impl Iterator<Item = (Accessor, &Self::Blob)>;
}

impl<T> ToAccessorIterator for Vec<T> {
    type Blob = T;
    fn accessor_iter(&self) -> impl Iterator<Item = (Accessor, &Self::Blob)> {
        self.iter().enumerate().map(|(index, blob)| (Accessor::Index(index), blob))
    }
}

impl<K: ToString, V> ToAccessorIterator for BTreeMap<K, V> {
    type Blob = V;
    fn accessor_iter(&self) -> impl Iterator<Item = (Accessor, &Self::Blob)> {
        self.iter().map(|(key, value)| (Accessor::Key(key.to_string()), value))
    }
}

impl<K: ToString, V> ToAccessorIterator for HashMap<K, V> {
    type Blob = V;
    fn accessor_iter(&self) -> impl Iterator<Item = (Accessor, &Self::Blob)> {
        self.iter().map(|(key, value)| (Accessor::Key(key.to_string()), value))
    }
}