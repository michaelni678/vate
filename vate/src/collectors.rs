use crate::{Collector, Exit, Report};

/// Collects all invalid reports and error reports.
pub struct InvalidsAndErrors;

impl<E> Collector<E> for InvalidsAndErrors {
    fn apply(parent: &mut Report<E>, child: Report<E>) -> Result<(), Exit<E>> {
        match child.get_validity() {
            Ok(true) => {}
            Ok(false) => {
                // If the parent validity is valid, set it to invalid, since the child is invalid.
                // If the parent validity is an error, this collector will respect that error and not
                // overwrite it.
                if parent.is_valid() {
                    parent.set_invalid();
                }
                parent.push_child(child);
            }
            Err(_) => {
                parent.push_child(child);
            }
        }
        Ok(())
    }
}

/// Collects only errors and the first invalid report is encountered, exiting immediately afterward.
pub struct FirstInvalidAndPrecedingErrors;

impl<E> Collector<E> for FirstInvalidAndPrecedingErrors {
    fn apply(parent: &mut Report<E>, child: Report<E>) -> Result<(), Exit<E>> {
        match child.get_validity() {
            Ok(true) => {}
            Ok(false) => {
                // If the parent validity is valid, set it to invalid, since the child is invalid.
                // If the parent validity is an error, this collector will respect that error and not
                // overwrite it.
                if parent.is_valid() {
                    parent.set_invalid();
                }
                parent.push_child(child);
                return Err(Exit::Gracefully);
            }
            Err(_) => {
                parent.push_child(child);
            }
        }
        Ok(())
    }
}

/// Collects everything.
pub struct Everything;

impl<E> Collector<E> for Everything {
    fn apply(parent: &mut Report<E>, child: Report<E>) -> Result<(), Exit<E>> {
        if child.is_invalid() {
            // If the parent validity is valid, set it to invalid, since the child is invalid.
            // If the parent validity is an error, this collector will respect that error and not
            // overwrite it.
            if parent.is_valid() {
                parent.set_invalid();
            }
        }
        parent.push_child(child);
        Ok(())
    }
}
