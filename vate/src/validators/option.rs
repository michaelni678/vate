use crate::{Accessor, Collector, Exit, Report, Validator};

pub struct OptionSome;

impl<T, D, E> Validator<Option<T>, D, E> for OptionSome {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &Option<T>,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if target.is_some() {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message = String::from("is missing");
        }

        parent_report.push_child::<C>(child_report)
    }
}

pub struct OptionNone;

impl<T, D, E> Validator<Option<T>, D, E> for OptionNone {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &Option<T>,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if target.is_none() {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message = String::from("is not missing");
        }

        parent_report.push_child::<C>(child_report)
    }
}

pub struct OptionSomeThen<V>(pub V);

impl<T, D, E, V: Validator<T, D, E>> Validator<Option<T>, D, E> for OptionSomeThen<V> {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &Option<T>,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(validator) = self;

        if let Some(target_inner) = target {
            validator.run::<C>(accessor, target_inner, data, parent_report)?;
        }

        Ok(())
    }
}
