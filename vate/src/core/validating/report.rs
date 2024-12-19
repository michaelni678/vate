use crate::core::{ControlFlow, InterpretingArgs, Invalid};

/// Defines a report that holds invalid validations.
pub trait Report {
    /// Check if the report is valid.
    fn is_valid(&self) -> bool;

    /// Check if the report is invalid.
    fn is_invalid(&self) -> bool {
        !self.is_valid()
    }

    /// Split the current report.
    fn split(&mut self) -> Self;

    /// Merge a report with another.
    fn merge(&mut self, other: Self);

    /// Add an invalid to the report.
    fn add_invalid<D>(
        &mut self,
        invalid: Invalid,
        interpreting_args: InterpretingArgs<D>,
    ) -> ControlFlow;
}
