//! Built-in reports.

use crate::core::*;

#[cfg(feature = "serialize")]
use serde::Serialize;

/// A report that simply collects interpreted messages.
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct BasicReport {
    /// The maximum number of interpretations collected before exiting.
    #[cfg_attr(feature = "serialize", serde(skip))]
    pub limit: usize,

    /// The number of invalids that were pushed into the report.
    pub num_invalids: usize,

    /// The interpreted validations.
    pub interpretations: Vec<String>,
}

impl Default for BasicReport {
    fn default() -> Self {
        Self {
            limit: 1,
            num_invalids: 0,
            interpretations: Vec::new(),
        }
    }
}

impl Report for BasicReport {
    fn num_invalids(&self) -> usize {
        self.num_invalids
    }

    fn push_invalid<D>(
        &mut self,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
    ) -> ControlFlow {
        let limit = self.limit;

        if limit == 0 {
            return ControlFlow::Exit;
        }

        self.limit -= 1;

        if let Some(interpretation) = interpreter.interpret(&invalid, data) {
            self.interpretations.push(interpretation);
        }

        if self.limit == 0 {
            ControlFlow::Exit
        } else {
            ControlFlow::Continue
        }
    }
}

/// A report that keeps track of validation information along with the messages.
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct ComprehensiveReport {
    /// The maximum number of interpretations collected before exiting.
    #[cfg_attr(feature = "serialize", serde(skip))]
    pub limit: usize,

    /// The number of invalids that were pushed into the report.
    pub num_invalids: usize,

    /// The items of the report.
    pub items: Vec<ComprehensiveReportItem>,
}

impl Default for ComprehensiveReport {
    fn default() -> Self {
        Self {
            limit: 1,
            num_invalids: 0,
            items: Vec::new(),
        }
    }
}

impl Report for ComprehensiveReport {
    fn num_invalids(&self) -> usize {
        self.num_invalids
    }

    fn push_invalid<D>(
        &mut self,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
    ) -> ControlFlow {
        let limit = self.limit;

        if limit == 0 {
            return ControlFlow::Exit;
        }

        self.limit -= 1;

        if let Some(interpretation) = interpreter.interpret(&invalid, data) {
            let item = ComprehensiveReportItem {
                type_ident: invalid.type_ident,
                field_ident: invalid.field_ident,
                vtags: invalid.vtags,
                interpretation,
            };

            self.items.push(item);
        }

        if self.limit == 0 {
            ControlFlow::Exit
        } else {
            ControlFlow::Continue
        }
    }
}

/// An item of [`ComprehensiveReport`].
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct ComprehensiveReportItem {
    /// The type ident.
    pub type_ident: TypeIdent,

    /// The field ident.
    pub field_ident: FieldIdent,

    /// The validation tags.
    pub vtags: Vec<ValidationTag>,

    /// The interpretation.
    pub interpretation: String,
}
