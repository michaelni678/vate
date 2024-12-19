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
        $crate::validators::Bundle2($a, $b)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::validators::Bundle2($a, $crate::validators::Bundle!($($rest)*))
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
        ValidatingArgs { target, context }: ValidatingArgs<T, C>,
        invalid: Invalid,
        InterpretingArgs { interpreter, data }: InterpretingArgs<D>,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        match self.0.run(
            ValidatingArgs { target, context },
            invalid.clone(),
            InterpretingArgs { interpreter, data },
            report,
        ) {
            Ok(ControlFlow::Continue) => self.1.run(
                ValidatingArgs { target, context },
                invalid.clone(),
                InterpretingArgs { interpreter, data },
                report,
            ),
            ret => ret,
        }
    }
}
