//! Bundle validators.

use crate::core::*;

/// Expands into (possibly nested) clusters of [`Bundle2`].
#[doc(hidden)]
#[macro_export]
macro_rules! _Bundle {
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {
        $crate::validators::bundle::Bundle2($a, $b)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::validators::bundle::Bundle2($a, $crate::validators::bundle::Bundle!($($rest)*))
    };
}

#[doc(inline)]
pub use _Bundle as Bundle;

/// Validates by running the two inner validators.
pub struct Bundle2<V1, V2>(pub V1, pub V2);

impl<T, C, E, V1, V2> Validator<T, C, E> for Bundle2<V1, V2>
where
    T: Copy,
    for<'a> &'a C: Copy,
    V1: Validator<T, C, E>,
    V2: Validator<T, C, E>,
{
    fn run<D, R>(
        self,
        target: T,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let result = self
            .0
            .run(target, context, invalid.clone(), interpreter, data, report);

        if matches!(result, Ok(ControlFlow::Continue)) {
            return self
                .1
                .run(target, context, invalid, interpreter, data, report);
        }

        result
    }
}
