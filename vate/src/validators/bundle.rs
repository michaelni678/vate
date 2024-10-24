use crate::{Accessor, Collector, Exit, Report, Validator};

#[doc(hidden)]
#[macro_export]
macro_rules! _Bundle {
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {
        $crate::Bundle2($a, $b)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::Bundle2($a, ::vate::Bundle!($($rest)*))
    };
}

#[doc(inline)]
pub use _Bundle as Bundle;

pub struct Bundle2<V1, V2>(pub V1, pub V2);

impl<T, D, E, V1, V2> Validator<T, D, E> for Bundle2<V1, V2>
where
    T: Copy,
    V1: Validator<T, D, E>,
    V2: Validator<T, D, E>,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: T,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(validator1, validator2) = self;

        validator1.run::<C>(accessor.clone(), target, data, parent_report)?;
        validator2.run::<C>(accessor, target, data, parent_report)
    }
}
