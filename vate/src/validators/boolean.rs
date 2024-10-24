use crate::{Accessor, Collector, Exit, Report, Validator};

pub struct BooleanTrue;

impl<D, E> Validator<bool, D, E> for BooleanTrue {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: bool,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if target {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
        }

        C::apply(parent_report, child_report)
    }
}

pub struct BooleanFalse;

impl<D, E> Validator<bool, D, E> for BooleanFalse {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: bool,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if !target {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
        }

        C::apply(parent_report, child_report)
    }
}

pub struct Boolean(pub bool);

impl<T, D, E> Validator<T, D, E> for Boolean {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        _target: T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(validity) = *self;

        let mut child_report = Report::new(accessor);

        child_report.set_validity(Ok(validity));

        C::apply(parent_report, child_report)
    }
}
