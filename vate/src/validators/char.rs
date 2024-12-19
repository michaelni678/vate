//! Character validators.

use crate::core::*;

/// Validates the target is alphabetic.
pub struct Alphabetic;

impl Alphabetic {
    pub const VALIDATOR_NAME: &'static str = "Alphabetic";

    pub const DEFAULT_VALIDATOR_VARIANT: u8 = 0;
}

impl<C, E> Validator<char, C, E> for Alphabetic {
    fn run<D, R>(
        self,
        target: char,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        interpreter_data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        if target.is_alphabetic() {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.add_invalid(
                invalid.add_validation(
                    ValidatorIdent {
                        name: Alphabetic::VALIDATOR_NAME,
                        variant: Alphabetic::DEFAULT_VALIDATOR_VARIANT,
                    },
                    Details::default(),
                ),
                interpreter,
                interpreter_data,
            ))
        }
    }
}
