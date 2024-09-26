use crate::{Collector, Exit, Report};

pub struct InvalidsAndErrors;

impl<E> Collector<E> for InvalidsAndErrors {
    fn apply(parent_report: &mut Report<E>, child_report: Report<E>) -> Result<(), Exit<E>> {
        match child_report.validity {
            Ok(true) => {}
            Ok(false) => {
                if let Ok(parent_validity) = &mut parent_report.validity {
                    *parent_validity = false;
                }
                parent_report.children.push(child_report);
            }
            Err(_) => {
                parent_report.children.push(child_report);
            }
        }
        Ok(())
    }
}