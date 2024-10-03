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
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message = String::from("is false");
        }

        parent_report.push_child::<C>(child_report)
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
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message = String::from("is true");
        }

        parent_report.push_child::<C>(child_report)
    }
}
