//! String validators.

use std::str::Chars;

use crate::core::*;

/// Forwards a strings characters to the inner validator.
pub struct Characters<V>(pub V);

impl Characters<()> {
    pub const VALIDATOR_NAME: &'static str = "Characters";

    pub const DEFAULT_VALIDATOR_VARIANT: u8 = 0;
}

impl<T, C, E, V> Validator<T, C, E> for Characters<V>
where
    T: AsRef<str>,
    for<'a> V: Validator<Chars<'a>, C, E>,
{
    fn run<D, R>(
        self,
        target: T,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        interpreter_data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        self.0.run(
            target.as_ref().chars(),
            context,
            invalid.add_validation(
                ValidatorIdent {
                    name: Characters::VALIDATOR_NAME,
                    variant: Characters::DEFAULT_VALIDATOR_VARIANT,
                },
                Details::default(),
            ),
            interpreter,
            interpreter_data,
            report,
        )
    }
}
