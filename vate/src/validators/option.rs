use crate::{Accessor, Collector, Exit, Report, Validator};

pub struct NotMissing;

impl<T, D, E> Validator<Option<T>, D, E> for NotMissing {
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
            child_report.message = format!("is missing");
        }

        parent_report.push_child::<C>(child_report)
    }
}

pub struct Missing;

impl<T, D, E> Validator<Option<T>, D, E> for Missing {
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
            child_report.message = format!("is not missing");
        }

        parent_report.push_child::<C>(child_report)
    }
}

pub struct NotMissingThen<V>(pub V);

impl<T, D, E, V: Validator<T, D, E>> Validator<Option<T>, D, E> for NotMissingThen<V> {
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