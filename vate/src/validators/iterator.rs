//! Iterator validators.

use crate::core::*;

/// Validates that all items of the target satisfy the inner validator.
pub struct All<V>(pub V);

impl All<()> {
    pub const VALIDATOR_NAME: &'static str = "All";

    pub const ITERATING_VALIDATOR_VARIANT: u8 = 0;
    pub const TARGET_ITEM_INDEX_DETAIL_INDEX: usize = 0;

    pub const FINISHED_VALIDATOR_VARIANT: u8 = 1;
}

impl<T, C, E, V> Validator<T, C, E> for All<V>
where
    T: Iterator,
    for<'a> &'a V: Validator<T::Item, C, E>,
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
        let v = &self.0;

        for (index, target_item) in target.enumerate() {
            let mut split = report.split();

            let result = v.run(
                target_item,
                context,
                invalid.clone().add_validation(
                    ValidatorIdent {
                        name: All::VALIDATOR_NAME,
                        variant: All::ITERATING_VALIDATOR_VARIANT,
                    },
                    Details::default().set_detail(All::TARGET_ITEM_INDEX_DETAIL_INDEX, &index),
                ),
                interpreter,
                interpreter_data,
                &mut split,
            );
            let is_valid = split.is_valid();

            report.merge(split);

            if is_valid {
                return result;
            } else {
                let control_flow = report.add_invalid(
                    invalid.clone().add_validation(
                        ValidatorIdent {
                            name: All::VALIDATOR_NAME,
                            variant: All::FINISHED_VALIDATOR_VARIANT,
                        },
                        Details::default(),
                    ),
                    interpreter,
                    interpreter_data,
                );

                if let ControlFlow::Exit = control_flow {
                    return Ok(ControlFlow::Exit);
                }
            }
        }

        Ok(ControlFlow::Continue)
    }
}
