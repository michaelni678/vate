use crate::{Accessor, Collector, Exit, Report, Validator};

pub struct True;

impl<D, E> Validator<bool, D, E> for True {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &bool,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if *target {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message("is false");
        }

        C::apply(parent_report, child_report)
    }
}

pub struct False;

impl<D, E> Validator<bool, D, E> for False {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &bool,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if !*target {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message("is true");
        }

        C::apply(parent_report, child_report)
    }
}
