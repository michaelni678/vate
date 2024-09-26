use std::{borrow::Borrow, fmt::Display};

use crate::{Accessor, Collector, Exit, Report, Validator};

pub struct LessThan<T>(pub T);

impl<T, D, E, U> Validator<T, D, E> for LessThan<U>
where
    T: PartialOrd + Display,
    U: Borrow<T> + Display,
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

        if target < other.borrow() {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message = format!("is \"{target}\", which is not less than \"{other}\"");
        }

        parent_report.push_child::<C>(child_report)
    }
}

pub struct LessThanOrEqualTo<T>(pub T);

impl<T, D, E, U> Validator<T, D, E> for LessThanOrEqualTo<U>
where
    T: PartialOrd + Display,
    U: Borrow<T> + Display,
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

        if target <= other.borrow() {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message =
                format!("is \"{target}\", which is not less than or equal to \"{other}\"");
        }

        parent_report.push_child::<C>(child_report)
    }
}

pub struct GreaterThan<T>(pub T);

impl<T, D, E, U> Validator<T, D, E> for GreaterThan<U>
where
    T: PartialOrd + Display,
    U: Borrow<T> + Display,
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

        if target > other.borrow() {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message =
                format!("is \"{target}\", which is not greater than \"{other}\"");
        }

        parent_report.push_child::<C>(child_report)
    }
}

pub struct GreaterThanOrEqualTo<T>(pub T);

impl<T, D, E, U> Validator<T, D, E> for GreaterThanOrEqualTo<U>
where
    T: PartialOrd + Display,
    U: Borrow<T> + Display,
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

        if target >= other.borrow() {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message =
                format!("is \"{target}\", which is not greater than or equal to \"{other}\"");
        }

        parent_report.push_child::<C>(child_report)
    }
}

pub struct EqualTo<T>(pub T);

impl<T, D, E, U> Validator<T, D, E> for EqualTo<U>
where
    T: PartialEq + Display,
    U: Borrow<T> + Display,
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

        if target == other.borrow() {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message = format!("is \"{target}\", which is not equal to \"{other}\"");
        }

        parent_report.push_child::<C>(child_report)
    }
}

pub struct NotEqualTo<T>(pub T);

impl<T, D, E, U> Validator<T, D, E> for NotEqualTo<U>
where
    T: PartialEq + Display,
    U: Borrow<T> + Display,
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

        if target != other.borrow() {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message = format!("is \"{target}\", which is equal to \"{other}\"");
        }

        parent_report.push_child::<C>(child_report)
    }
}
