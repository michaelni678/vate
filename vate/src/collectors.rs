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
                parent.children.insert(child.into());
            }
            Err(_) => {
                parent.children.insert(child.into());
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
                parent.children.insert(child.into());
                return Err(Exit::Gracefully);
            }
            Err(_) => {
                parent.children.insert(child.into());
            }
        }
        Ok(())
    }
}

pub struct Everything;

impl<E> Collector<E> for Everything {
    fn apply(parent: &mut Report<E>, child: Report<E>) -> Result<(), Exit<E>> {
        if let Ok(false) = child.validity {
            if let Ok(parent_validity) = &mut parent.validity {
                *parent_validity = false;
            }
        }
        parent.children.insert(child.into());
        Ok(())
    }
}
