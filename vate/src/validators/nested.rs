//! Nested validators.

use crate::core::*;

/// Validates an implementor of [`Validate`].
pub struct Nested;

impl Nested {
    pub const VALIDATOR_NAME: &'static str = "Nested";

    pub const DEFAULT_VALIDATOR_VARIANT: u8 = 0;
}

impl<T, C, E> Validator<&T, C, E> for Nested
where
    T: Validate<Context = C, Error = E>,
{
    fn run<D, R>(
        self,
        target: &T,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        interpreter_data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let mut split = report.split();

        let result = target.validate(context, interpreter, interpreter_data, &mut split);
        let is_valid = split.is_valid();

        report.merge(split);

        if is_valid {
            return result;
        }

        let control_flow_1 = report.add_invalid(
            invalid.add_validation(
                ValidatorIdent {
                    name: Nested::VALIDATOR_NAME,
                    variant: Nested::DEFAULT_VALIDATOR_VARIANT,
                },
                Details::default(),
            ),
            interpreter,
            interpreter_data,
        );

        let control_flow_2 = result?;

        if matches!(control_flow_1, ControlFlow::Exit)
            || matches!(control_flow_2, ControlFlow::Exit)
        {
            Ok(ControlFlow::Exit)
        } else {
            Ok(ControlFlow::Continue)
        }
    }
}
