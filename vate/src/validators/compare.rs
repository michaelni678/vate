use std::{borrow::Cow, fmt::Display};

use crate::{Accessor, Collector, Exit, Report, Validator};

/// Convenience macro for generating comparison validators
/// `CompareLessThan`, `CompareLessThanOrEqualTo`, `CompareGreaterThan`,
/// `CompareGreaterThanOrEqualTo`, `CompareEqualTo`, and `CompareNotEqualTo`.
/// ### Usage
/// ```ignore
/// Compare!( < 5 ); // Generates CompareLessThan(Cow::Owned(5))
/// Compare!( == &self.a ); // Generates CompareEqualTo(Cow::Borrowed(&self.a))
/// ```
/// ### Warning
/// This macro is purely syntactical! Something like...
/// ```ignore
/// let x = &5;
/// Compare!( < x ); // Generates CompareLessThan(Cow::Owned(&5)), which is (probably) not what you want.
/// ```
/// ... may not work.
// Note: This macro's name is `UpperCamelCase`, which doesn't conform with typical macro naming conventions.
// However, it was done to match the naming convention of normal validators.
#[macro_export]
macro_rules! Compare {
    ( < & $value:expr) => {
        ::vate::CompareLessThan(::std::borrow::Cow::Borrowed(&$value))
    };
    ( < $value:expr) => {
        ::vate::CompareLessThan(::std::borrow::Cow::Owned($value))
    };
    ( <= & $value:expr) => {
        ::vate::CompareLessThanOrEqualTo(::std::borrow::Cow::Borrowed(&$value))
    };
    ( <= $value:expr) => {
        ::vate::CompareLessThanOrEqualTo(::std::borrow::Cow::Owned($value))
    };
    ( > & $value:expr) => {
        ::vate::CompareGreaterThan(::std::borrow::Cow::Borrowed(&$value))
    };
    ( > $value:expr) => {
        ::vate::CompareGreaterThan(::std::borrow::Cow::Owned($value))
    };
    ( >= & $value:expr) => {
        ::vate::CompareGreaterThanOrEqualTo(::std::borrow::Cow::Borrowed(&$value))
    };
    ( >= $value:expr) => {
        ::vate::CompareGreaterThanOrEqualTo(::std::borrow::Cow::Owned($value))
    };
    ( == & $value:expr) => {
        ::vate::CompareEqualTo(::std::borrow::Cow::Borrowed(&$value))
    };
    ( == $value:expr) => {
        ::vate::CompareEqualTo(::std::borrow::Cow::Owned($value))
    };
    ( != & $value:expr) => {
        ::vate::CompareNotEqualTo(::std::borrow::Cow::Borrowed(&$value))
    };
    ( != $value:expr) => {
        ::vate::CompareNotEqualTo(::std::borrow::Cow::Owned($value))
    };
}

pub struct CompareLessThan<'a, T: Clone>(pub Cow<'a, T>);

impl<T, D, E, U> Validator<T, D, E> for CompareLessThan<'_, U>
where
    T: PartialOrd<U> + Display,
    U: Clone + Display,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(other) = self;

        let mut child_report = Report::new(accessor);

        if target.lt(other) {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message = format!("is \"{target}\", which is not less than \"{other}\"");
        }

        parent_report.push_child::<C>(child_report)
    }
}

pub struct CompareLessThanOrEqualTo<'a, T: Clone>(pub Cow<'a, T>);

impl<T, D, E, U> Validator<T, D, E> for CompareLessThanOrEqualTo<'_, U>
where
    T: PartialOrd<U> + Display,
    U: Clone + Display,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(other) = self;

        let mut child_report = Report::new(accessor);

        if target.le(other) {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message =
                format!("is \"{target}\", which is not less than or equal to \"{other}\"");
        }

        parent_report.push_child::<C>(child_report)
    }
}

pub struct CompareGreaterThan<'a, T: Clone>(pub Cow<'a, T>);

impl<T, D, E, U> Validator<T, D, E> for CompareGreaterThan<'_, U>
where
    T: PartialOrd<U> + Display,
    U: Clone + Display,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(other) = self;

        let mut child_report = Report::new(accessor);

        if target.gt(other) {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message =
                format!("is \"{target}\", which is not greater than \"{other}\"");
        }

        parent_report.push_child::<C>(child_report)
    }
}

pub struct CompareGreaterThanOrEqualTo<'a, T: Clone>(pub Cow<'a, T>);

impl<T, D, E, U> Validator<T, D, E> for CompareGreaterThanOrEqualTo<'_, U>
where
    T: PartialOrd<U> + Display,
    U: Clone + Display,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(other) = self;

        let mut child_report = Report::new(accessor);

        if target.ge(other) {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message =
                format!("is \"{target}\", which is not greater than or equal to \"{other}\"");
        }

        parent_report.push_child::<C>(child_report)
    }
}

pub struct CompareEqualTo<'a, T: Clone>(pub Cow<'a, T>);

impl<T, D, E, U> Validator<T, D, E> for CompareEqualTo<'_, U>
where
    T: PartialEq<U> + Display,
    U: Clone + Display,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(other) = self;

        let mut child_report = Report::new(accessor);

        if target.eq(other) {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message = format!("is \"{target}\", which is not equal to \"{other}\"");
        }

        parent_report.push_child::<C>(child_report)
    }
}

pub struct CompareNotEqualTo<'a, T: Clone>(pub Cow<'a, T>);

impl<T, D, E, U> Validator<T, D, E> for CompareNotEqualTo<'_, U>
where
    T: PartialEq<U> + Display,
    U: Clone + Display,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(other) = self;

        let mut child_report = Report::new(accessor);

        if target.ne(other) {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message = format!("is \"{target}\", which is equal to \"{other}\"");
        }

        parent_report.push_child::<C>(child_report)
    }
}
