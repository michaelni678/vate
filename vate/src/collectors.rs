use crate::{Collector, Exit, Report};

pub struct InvalidsAndErrors;

impl<E> Collector<E> for InvalidsAndErrors {
    fn apply(parent: &mut Report<E>, child: Report<E>) -> Result<(), Exit<E>> {
        match child.validity {
            Ok(true) => {}
            Ok(false) => {
                if let Ok(parent_validity) = &mut parent.validity {
                    *parent_validity = false;
                }
                parent.children.push(child);
            }
            Err(_) => {
                parent.children.push(child);
            }
        }
        Ok(())
    }
}

pub struct FirstInvalidAndPrecedingErrors;

impl<E> Collector<E> for FirstInvalidAndPrecedingErrors {
    fn apply(parent: &mut Report<E>, child: Report<E>) -> Result<(), Exit<E>> {
        match child.validity {
            Ok(true) => {}
            Ok(false) => {
                if let Ok(parent_validity) = &mut parent.validity {
                    *parent_validity = false;
                }
                parent.children.push(child);
                return Err(Exit::Gracefully);
            }
            Err(_) => {
                parent.children.push(child);
            }
        }
        Ok(())
    }
}
