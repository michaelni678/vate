use std::fmt::Debug;

use vate::{Accessor, Report};

pub fn invalid_at_path<E: Debug + PartialEq>(report: &Report<E>, path: impl AsRef<[Accessor]>) {
    assert_eq!(report.validity_at_path(path), Some(&Ok(false)),);
}
