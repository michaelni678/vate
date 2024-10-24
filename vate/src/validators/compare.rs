use std::borrow::Cow;

use crate::{Accessor, Collector, Exit, Report, Validator};

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
