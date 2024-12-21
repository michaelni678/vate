//! Option validators.

use crate::core::*;

/// Validates the target is the [`Some`] variant.
pub struct Something;

impl Something {
    pub const DEFAULT_VTAG: &'static str = "m=option;v=Something";
}

impl<T, C, E> Validator<&Option<T>, C, E> for Something {
    fn run<D, R>(
        self,
        target: &Option<T>,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        if target.is_some() {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(Something::DEFAULT_VTAG, Detailer::default()),
                interpreter,
                data,
            ))
        }
    }
}

/// Forwards the unwrapped value of the target to the inner validator.
///
/// This validator does not produce an invalid validation if the target is [`None`].
pub struct SomethingThen<V>(pub V);

impl<T, C, E, V> Validator<&Option<T>, C, E> for SomethingThen<V>
where
    for<'a> V: Validator<&'a T, C, E>,
{
    fn run<D, R>(
        self,
        target: &Option<T>,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        if let Some(target_unwrapped) = target {
            return self.0.run(
                target_unwrapped,
                context,
                invalid,
                interpreter,
                data,
                report,
            );
        }

        Ok(ControlFlow::Continue)
    }
}

/// Validates the target is the [`None`] variant.
pub struct Nothing;

impl Nothing {
    pub const DEFAULT_VTAG: &'static str = "m=option;v=Nothing";
}

impl<T, C, E> Validator<&Option<T>, C, E> for Nothing {
    fn run<D, R>(
        self,
        target: &Option<T>,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        if target.is_none() {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(Nothing::DEFAULT_VTAG, Detailer::default()),
                interpreter,
                data,
            ))
        }
    }
}
