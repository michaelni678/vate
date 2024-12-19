//! Nested validators.

use crate::core::*;

pub struct Nested;

impl Nested {
    pub const VALIDATOR_IDENT: &'static str = "Nested";
}

impl<T, C, E> Validator<&T, C, E> for Nested
where
    T: Validate<Context = C, Error = E>,
{
    fn run<D, R>(
        self,
        validating_args: ValidatingArgs<&T, C>,
        invalid: Invalid,
        InterpretingArgs { interpreter, data }: InterpretingArgs<D>,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let mut split = report.split();
        let result = validating_args.target.validate(
            validating_args.context,
            InterpretingArgs { interpreter, data },
            &mut split,
        );
        let is_valid = split.is_valid();
        report.merge(split);

        if is_valid {
            return result;
        }

        let control_flow_1 = report.add_invalid(
            invalid.push(
                ValidatorIdent::default().set_name(Nested::VALIDATOR_IDENT),
                Details::default(),
            ),
            InterpretingArgs { interpreter, data },
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
