use crate::{Accessor, Collector, Exit, Report, Validator};

pub struct Bundle2<V1, V2>(pub V1, pub V2);

impl<T, D, E, V1, V2> Validator<T, D, E> for Bundle2<V1, V2>
where
    V1: Validator<T, D, E>,
    V2: Validator<T, D, E>,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(validator1, validator2) = self;

        validator1.run::<C>(accessor.clone(), target, data, parent_report)?;
        validator2.run::<C>(accessor, target, data, parent_report)
    }
}

// Note: This macro's name is `UpperCamelCase`, which doesn't conform with typical macro naming conventions.
// However, it was done to match the naming convention of normal validators.
#[macro_export]
macro_rules! Bundle {
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {
        $crate::Bundle2($a, $b)
    };
    ($a:expr, $($rest:tt)*) => {
        $crate::Bundle2($a, Bundle!($($rest)*))
    };
}
