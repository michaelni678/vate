use std::borrow::Cow;

use crate::{Accessor, Collector, Exit, Report, Validator};

/// Validates an ordering.
///
/// # Examples
/// ```rust
/// use vate::{
///     path, Accessor, CollectionIterate, Compare, Everything,
///     IteratorIndexed, Report, Validate,
/// };
///
/// #[derive(Validate)]
/// struct Numbers {
///     #[vate(CollectionIterate(IteratorIndexed(Compare!( >= 0 ))))]
///     inner: Vec<i32>,
/// }
///
/// let numbers = Numbers {
///     inner: vec![-2, -1, 0, 1, 2],
/// };
///
/// let mut report = Report::new(Accessor::Root("numbers"));
///
/// let _ = numbers.validate::<Everything>(&(), &mut report);
///
/// assert_eq!(report.get_leaves().count(), 5);
///
/// assert_eq!(report.get_children_at_path(&path!(numbers.inner[0])).count(), 1);
/// assert!(report.get_children_at_path(&path!(numbers.inner[0])).all(|child| child.is_invalid()));
///
/// assert_eq!(report.get_children_at_path(&path!(numbers.inner[1])).count(), 1);
/// assert!(report.get_children_at_path(&path!(numbers.inner[1])).all(|child| child.is_invalid()));
///
/// assert_eq!(report.get_children_at_path(&path!(numbers.inner[2])).count(), 1);
/// assert!(report.get_children_at_path(&path!(numbers.inner[2])).all(|child| child.is_valid()));
///
/// assert_eq!(report.get_children_at_path(&path!(numbers.inner[3])).count(), 1);
/// assert!(report.get_children_at_path(&path!(numbers.inner[3])).all(|child| child.is_valid()));
///
/// assert_eq!(report.get_children_at_path(&path!(numbers.inner[4])).count(), 1);
/// assert!(report.get_children_at_path(&path!(numbers.inner[4])).all(|child| child.is_valid()));
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! _Compare {
    ( < & $value:expr) => {
        ::vate::CompareValues::LT(::std::borrow::Cow::Borrowed(&$value))
    };
    ( < $value:expr) => {
        ::vate::CompareValues::LT(::std::borrow::Cow::Owned($value))
    };
    ( <= & $value:expr) => {
        ::vate::CompareValues::LE(::std::borrow::Cow::Borrowed(&$value))
    };
    ( <= $value:expr) => {
        ::vate::CompareValues::LE(::std::borrow::Cow::Owned($value))
    };
    ( > & $value:expr) => {
        ::vate::CompareValues::GT(::std::borrow::Cow::Borrowed(&$value))
    };
    ( > $value:expr) => {
        ::vate::CompareValues::GT(::std::borrow::Cow::Owned($value))
    };
    ( >= & $value:expr) => {
        ::vate::CompareValues::GE(::std::borrow::Cow::Borrowed(&$value))
    };
    ( >= $value:expr) => {
        ::vate::CompareValues::GE(::std::borrow::Cow::Owned($value))
    };
    ( == & $value:expr) => {
        ::vate::CompareValues::EQ(::std::borrow::Cow::Borrowed(&$value))
    };
    ( == $value:expr) => {
        ::vate::CompareValues::EQ(::std::borrow::Cow::Owned($value))
    };
    ( != & $value:expr) => {
        ::vate::CompareValues::NE(::std::borrow::Cow::Borrowed(&$value))
    };
    ( != $value:expr) => {
        ::vate::CompareValues::NE(::std::borrow::Cow::Owned($value))
    };
}

#[doc(inline)]
pub use _Compare as Compare;

/// Validates an ordering.
///
/// There are certain situations where using this validator directly is necessary.
/// However, in most cases you can use the [`Compare`] macro instead.
///
/// # Examples
/// See the [`Compare`] macro.
pub enum CompareValues<'a, T: Clone> {
    LT(Cow<'a, T>),
    LE(Cow<'a, T>),
    GT(Cow<'a, T>),
    GE(Cow<'a, T>),
    EQ(Cow<'a, T>),
    NE(Cow<'a, T>),
}

impl<'a, T, D, E, U> Validator<&T, D, E> for CompareValues<'a, U>
where
    T: PartialOrd<U> + PartialEq<U>,
    U: Clone,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        let comparison = match self {
            Self::LT(other) => target.lt(other),
            Self::LE(other) => target.le(other),
            Self::GT(other) => target.gt(other),
            Self::GE(other) => target.ge(other),
            Self::EQ(other) => target.eq(other),
            Self::NE(other) => target.ne(other),
        };

        if comparison {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
        }

        C::apply(parent_report, child_report)
    }
}
