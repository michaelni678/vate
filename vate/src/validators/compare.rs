use std::{borrow::Cow, fmt::Display};

use crate::{Accessor, Collector, Exit, Report, Validator};

/// Convenience macro for generating comparison validators
/// `LessThan`, `LessThanOrEqualTo`, `GreaterThan`,
/// `GreaterThanOrEqualTo`, `EqualTo`, and `NotEqualTo`.
/// ### Usage
/// ```no_run
/// Compare!( < 5 ) // Generates LessThan(Cow::Owned(5))
/// Compare!( == &self.a ) // Generates EqualTo(Cow::Borrowed(&self.a))
/// ```
/// ### Warning
/// This macro is purely syntactical! Something like...
/// ```no_run
/// let x = &5;
/// Compare!( < x ) // Generates LessThan(Cow::Owned(&5)), which is (probably) not what you want.
/// ```
/// ... may not work.
#[macro_export]
macro_rules! Compare {
    ( < & $value:expr) => {
        ::vate::LessThan(::std::borrow::Cow::Borrowed(&$value))
    };
    ( < $value:expr) => {
        ::vate::LessThan(::std::borrow::Cow::Owned($value))
    };
    ( <= & $value:expr) => {
        ::vate::LessThanOrEqualTo(::std::borrow::Cow::Borrowed(&$value))
    };
    ( <= $value:expr) => {
        ::vate::LessThanOrEqualTo(::std::borrow::Cow::Owned($value))
    };
    ( > & $value:expr) => {
        ::vate::GreaterThan(::std::borrow::Cow::Borrowed(&$value))
    };
    ( > $value:expr) => {
        ::vate::GreaterThan(::std::borrow::Cow::Owned($value))
    };
    ( >= & $value:expr) => {
        ::vate::GreaterThanOrEqualTo(::std::borrow::Cow::Borrowed(&$value))
    };
    ( >= $value:expr) => {
        ::vate::GreaterThanOrEqualTo(::std::borrow::Cow::Owned($value))
    };
    ( == & $value:expr) => {
        ::vate::EqualTo(::std::borrow::Cow::Borrowed(&$value))
    };
    ( == $value:expr) => {
        ::vate::EqualTo(::std::borrow::Cow::Owned($value))
    };
    ( != & $value:expr) => {
        ::vate::NotEqualTo(::std::borrow::Cow::Borrowed(&$value))
    };
    ( != $value:expr) => {
        ::vate::NotEqualTo(::std::borrow::Cow::Owned($value))
    };
}

pub struct LessThan<'a, T: Clone>(pub Cow<'a, T>);

impl<T, D, E, U> Validator<T, D, E> for LessThan<'_, U>
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

pub struct LessThanOrEqualTo<'a, T: Clone>(pub Cow<'a, T>);

impl<T, D, E, U> Validator<T, D, E> for LessThanOrEqualTo<'_, U>
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

pub struct GreaterThan<'a, T: Clone>(pub Cow<'a, T>);

impl<T, D, E, U> Validator<T, D, E> for GreaterThan<'_, U>
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

pub struct GreaterThanOrEqualTo<'a, T: Clone>(pub Cow<'a, T>);

impl<T, D, E, U> Validator<T, D, E> for GreaterThanOrEqualTo<'_, U>
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

pub struct EqualTo<'a, T: Clone>(pub Cow<'a, T>);

impl<T, D, E, U> Validator<T, D, E> for EqualTo<'_, U>
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

pub struct NotEqualTo<'a, T: Clone>(pub Cow<'a, T>);

impl<T, D, E, U> Validator<T, D, E> for NotEqualTo<'_, U>
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
