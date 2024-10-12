use std::ops::Deref;

use crate::{Accessor, Collector, Exit, Report, Validator};

/// Runs the inner validator, passing over the iterated items. The indices of the elements will generate `Accessor::Index`.
///
/// # Target Type
/// Implementors of `Iterator` and `Clone`.
/// 
/// **WARNING: the iterator is cloned!**
///
/// # Fields / Arguments
/// `0`: the inner validator.
///
/// # Feature Flags
/// None
///
/// # Usage
/// ```rust
/// use vate::{path, Accessor, CollectionIterate, Everything, IteratorIndexed, Report, StringAlphabetic, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(CollectionIterate(IteratorIndexed(StringAlphabetic)))]
///     a: Vec<&'static str>,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: vec!["hello", "world", "!"],
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a[0])).unwrap());
/// assert!(report.is_all_valid_at_path(path!(example.a[1])).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.a[2])).unwrap());
/// ```
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

        let parent_result = C::apply(parent_report, child_report);

        child_result?;
        parent_result
    }
}

/// Runs the inner validator, passing over the iterated values. The keys of the values will generate `Accessor::Key`.
/// 
/// **NOTE: the key type must implement `ToString` so that it can generate `Accessor::Key`.**
///
/// # Target Type
/// Implementors of `Iterator` and `Clone`.
/// 
/// **WARNING: the iterator is cloned!**
///
/// # Fields / Arguments
/// `0`: the inner validator.
///
/// # Feature Flags
/// None
///
/// # Usage
/// ```rust
/// use std::collections::HashMap;
///
/// use vate::{path, Accessor, CollectionIterate, Compare, Everything, IteratorKeyed, Report, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(CollectionIterate(IteratorKeyed(Compare!( != 1 ))))]
///     a: HashMap<&'static str, i32>,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: HashMap::from([
///         ("zero", 0),
///         ("one", 1),
///         ("two", 2),
///     ]),
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a["zero"])).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.a["one"])).unwrap());
/// assert!(report.is_all_valid_at_path(path!(example.a["two"])).unwrap());
/// ```
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

        let parent_result = C::apply(parent_report, child_report);

        child_result?;
        parent_result
    }
}

/// Consumes an iterator, validating the number of elements is equal to field `0`.
///
/// # Target Type
/// Implementors of `Iterator` and `Clone`.
/// 
/// **WARNING: the iterator is cloned!**
///
/// # Fields / Arguments
/// `0`: the expected number of elements.
///
/// # Feature Flags
/// None
///
/// # Usage
/// ```rust
/// use vate::{path, Accessor, CollectionIterate, Everything, IteratorLengthEquals, Report, StringAlphabetic, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(CollectionIterate(IteratorLengthEquals(3)))]
///     a: Vec<i32>,
///     #[vate(CollectionIterate(IteratorLengthEquals(3)))]
///     b: Vec<i32>,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: vec![1, 2, 3],
///     b: vec![1, 2, 3, 4],
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
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
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message(format!("is not {required_len} items long"));
        }

        C::apply(parent_report, child_report)
    }
}

/// Validates the size of the iterator is equal to field `0`.
///
/// # Target Type
/// Implementors of `ExactSizeIterator`.
///
/// # Fields / Arguments
/// `0`: the expected number of elements.
///
/// # Feature Flags
/// None
///
/// # Usage
/// ```rust
/// use vate::{path, Accessor, CollectionIterate, Everything, ExactSizeIteratorLengthEquals, Report, StringAlphabetic, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(CollectionIterate(ExactSizeIteratorLengthEquals(3)))]
///     a: Vec<i32>,
///     #[vate(CollectionIterate(ExactSizeIteratorLengthEquals(3)))]
///     b: Vec<i32>,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: vec![1, 2, 3],
///     b: vec![1, 2, 3, 4],
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
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
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message(format!("is not {required_len} items long"));
        }

        C::apply(parent_report, child_report)
    }
}
