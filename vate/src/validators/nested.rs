//! Nested validators.

use crate::core::*;

/// Validates an implementor of [`Validate`].
pub struct Nested;

impl Nested {
    pub const DEFAULT_VTAG: ValidationTag = "m=nested;v=Nested";
}

impl<T, C, E> Validator<&T, C, E> for Nested
where
    T: Validate<Context = C, Error = E>,
{
    fn run<D, R>(
        &self,
        target: &T,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let num_invalids_before = report.num_invalids();

        let result = target.validate(context, interpreter, data, report);

        let num_invalids_after = report.num_invalids();

        if num_invalids_before == num_invalids_after {
            return result;
        }

        let cf2 = report.push_invalid(
            invalid.push_validation(Nested::DEFAULT_VTAG, Detailer::default()),
            interpreter,
            data,
        );
        let cf1 = result?;

        let either_exit = matches!(cf1, ControlFlow::Exit) || matches!(cf2, ControlFlow::Exit);
        if either_exit {
            Ok(ControlFlow::Exit)
        } else {
            Ok(ControlFlow::Continue)
        }
    }
}
