use crate::{
    core::{Accessor, Collector, Exit, Report, Validator},
    Validate,
};

pub struct Nested;

impl<T: Validate<Data = D, Error = E>, D, E> Validator<T, D, E> for Nested {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);
        let child_result = target.validate::<C>(data, &mut child_report);
        let parent_result = C::apply(parent_report, child_report);
        child_result?;
        parent_result
    }
}

#[cfg(test)]
mod tests {
    use vate::{path, Accessor, Everything, Nested, Report, StringAlphabetic, Validate};

    #[test]
    fn nested() {
        #[derive(Validate)]
        struct Example1 {
            #[vate(Nested)]
            example2: Example2,
        }

        #[derive(Validate)]
        struct Example2 {
            #[vate(StringAlphabetic)]
            a: String,
        }

        let example1 = Example1 {
            example2: Example2 {
                a: String::from("0"),
            },
        };

        let mut report = Report::new(Accessor::Root("example1"));
        let _ = example1.validate::<Everything>(&(), &mut report);

        assert!(report
            .is_any_invalid_at_path(path!(example1.example2.a))
            .unwrap());
    }
}
