use crate::{Collector, Exit, Report};

pub struct InvalidsAndErrors;

impl<E> Collector<E> for InvalidsAndErrors {
    fn apply(parent: &mut Report<E>, child: Report<E>) -> Result<(), Exit<E>> {
        match child.get_validity() {
            Ok(true) => {}
            Ok(false) => {
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

pub struct FirstInvalidAndPrecedingErrors;

impl<E> Collector<E> for FirstInvalidAndPrecedingErrors {
    fn apply(parent: &mut Report<E>, child: Report<E>) -> Result<(), Exit<E>> {
        match child.get_validity() {
            Ok(true) => {}
            Ok(false) => {
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

pub struct Everything;

impl<E> Collector<E> for Everything {
    fn apply(parent: &mut Report<E>, child: Report<E>) -> Result<(), Exit<E>> {
        if let Ok(false) = child.get_validity() {
            if parent.is_valid() {
                parent.set_invalid();
            }
        }
        parent.push_child(child);
        Ok(())
    }
}
