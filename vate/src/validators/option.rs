use crate::{Accessor, Collector, Exit, Report, Validator};

/// # Description
/// Validates a option is the `Some` variant.
/// # Target Type
/// `Option<T>`, where T is generic.
/// # Arguments
/// None
/// # Feature Flag
/// None
/// # Usage
/// ```rust
/// use vate::{path, Accessor, Everything, OptionSome, Report, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(OptionSome)]
///     a: Option<()>,
///     #[vate(OptionSome)]
///     b: Option<()>,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: Some(()),
///     b: None,
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
pub struct OptionSome;

impl<T, D, E> Validator<Option<T>, D, E> for OptionSome {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &Option<T>,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if target.is_some() {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message("is missing");
        }

        C::apply(parent_report, child_report)
    }
}

/// # Description
/// Validates a option is the `None` variant.
/// # Target Type
/// `Option<T>`, where T is generic.
/// # Arguments
/// None
/// # Feature Flag
/// None
/// # Usage
/// ```rust
/// use vate::{path, Accessor, Everything, OptionNone, Report, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(OptionNone)]
///     a: Option<()>,
///     #[vate(OptionNone)]
///     b: Option<()>,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: None,
///     b: Some(()),
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
pub struct OptionNone;

impl<T, D, E> Validator<Option<T>, D, E> for OptionNone {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &Option<T>,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if target.is_none() {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message("is not missing");
        }

        C::apply(parent_report, child_report)
    }
}

/// # Description
/// If the option is the `Some` variant, runs the inner validator with the option's unwrapped value.
/// # Target Type
/// `Option<T>`, where T is generic.
/// # Arguments
/// `0`: the inner validator that runs if the option is `Some`.
/// # Feature Flag
/// None
/// # Usage
/// ```rust
/// use vate::{path, Accessor, Everything, OptionSomeThen, Report, StringAscii, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(OptionSomeThen(StringAscii))]
///     a: Option<&'static str>,
///     #[vate(OptionSomeThen(StringAscii))]
///     b: Option<&'static str>,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: Some("hello world!"),
///     b: None,
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_empty_at_path(path!(example.b)));
/// ```
pub struct OptionSomeThen<V>(pub V);

impl<T, D, E, V: Validator<T, D, E>> Validator<Option<T>, D, E> for OptionSomeThen<V> {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &Option<T>,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(validator) = self;

        if let Some(target_inner) = target {
            validator.run::<C>(accessor, target_inner, data, parent_report)?;
        }

        Ok(())
    }
}
