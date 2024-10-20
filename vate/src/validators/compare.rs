use std::{borrow::Cow, fmt::Display};

use crate::{Accessor, Collector, Exit, Report, Validator};

// Note: This macro's name is `UpperCamelCase`, which doesn't conform with typical macro naming conventions.
// However, it was done to match the naming convention of normal validators.
//
/// Validates the specified comparison.
///
/// This is a convenience macro for generating comparison validators
/// `CompareLessThan`, `CompareLessThanOrEqualTo`, `CompareGreaterThan`,
/// `CompareGreaterThanOrEqualTo`, `CompareEqualTo`, and `CompareNotEqualTo`.
///
/// ```rust, text, ignore
/// Compare!( < 5 ); // Generates CompareLessThan(Cow::Owned(5))
/// Compare!( == &self.a ); // Generates CompareEqualTo(Cow::Borrowed(&self.a))
/// ```
///
/// This macro is purely syntactical! Something like...
/// ```rust, text, ignore
/// let x = &5;
/// Compare!( < x ); // Generates CompareLessThan(Cow::Owned(&5)), which is (probably) not what you want.
/// ```
/// ... may not work.
///
/// Requires the target type to be an implementor of `PartialOrd<U>`
/// and `Display`, where U is the type of the literal / variable being
/// compared to.
///
/// The arguments consist of any of the operators <, <=, >, >=, ==, or !=
/// preceding the literal / variable to compare to, which must be
/// comparable to the target type and made into a `Cow`.
///
/// # Examples
/// ```rust
/// use vate::{path, Accessor, Compare, Everything, Report, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(Compare!( > 3 ))]
///     a: i32,
///     #[vate(Compare!( == &self.a ))]
///     b: i32,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: 4,
///     b: 2,
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! _Compare {
    ( < & $value:expr) => {
        ::vate::CompareLessThan(::std::borrow::Cow::Borrowed(&$value))
    };
    ( < $value:expr) => {
        ::vate::CompareLessThan(::std::borrow::Cow::Owned($value))
    };
    ( <= & $value:expr) => {
        ::vate::CompareLessThanOrEqualTo(::std::borrow::Cow::Borrowed(&$value))
    };
    ( <= $value:expr) => {
        ::vate::CompareLessThanOrEqualTo(::std::borrow::Cow::Owned($value))
    };
    ( > & $value:expr) => {
        ::vate::CompareGreaterThan(::std::borrow::Cow::Borrowed(&$value))
    };
    ( > $value:expr) => {
        ::vate::CompareGreaterThan(::std::borrow::Cow::Owned($value))
    };
    ( >= & $value:expr) => {
        ::vate::CompareGreaterThanOrEqualTo(::std::borrow::Cow::Borrowed(&$value))
    };
    ( >= $value:expr) => {
        ::vate::CompareGreaterThanOrEqualTo(::std::borrow::Cow::Owned($value))
    };
    ( == & $value:expr) => {
        ::vate::CompareEqualTo(::std::borrow::Cow::Borrowed(&$value))
    };
    ( == $value:expr) => {
        ::vate::CompareEqualTo(::std::borrow::Cow::Owned($value))
    };
    ( != & $value:expr) => {
        ::vate::CompareNotEqualTo(::std::borrow::Cow::Borrowed(&$value))
    };
    ( != $value:expr) => {
        ::vate::CompareNotEqualTo(::std::borrow::Cow::Owned($value))
    };
}

#[doc(inline)]
pub use _Compare as Compare;

/// Validates the target is less than a value.
///
/// Requires the target type to be an implementor of `PartialOrd<U>`
/// and `Display`, where U is the type of the literal / variable being
/// compared to.
///
/// Takes a `Cow<'_, U>` for field `0`.
///
/// # Examples
/// ```rust
/// use std::borrow::Cow;
///
/// use vate::{path, Accessor, CompareLessThan, Everything, Report, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(CompareLessThan(Cow::Owned(3)))]
///     a: i32,
///     #[vate(CompareLessThan(Cow::Borrowed(&self.a)))]
///     b: i32,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: 1,
///     b: 5,
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
pub struct CompareLessThan<'a, T: Clone>(pub Cow<'a, T>);

impl<T, D, E, U> Validator<T, D, E> for CompareLessThan<'_, U>
where
    T: PartialOrd<U> + Display,
    U: Clone + Display,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(other) = self;

        let mut child_report = Report::new(accessor);

        if target.lt(other) {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message(format!(
                "is \"{target}\", which is not less than \"{other}\""
            ));
        }

        C::apply(parent_report, child_report)
    }
}

/// Validates the target is less than or equal to a value.
///
/// Requires the target type to be an implementor of `PartialOrd<U>`
/// and `Display`, where U is the type of the literal / variable being
/// compared to.
///
/// Takes a `Cow<'_, U>` for field `0`.
///
/// # Examples
/// ```rust
/// use std::borrow::Cow;
///
/// use vate::{path, Accessor, CompareLessThanOrEqualTo, Everything, Report, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(CompareLessThanOrEqualTo(Cow::Owned(3)))]
///     a: i32,
///     #[vate(CompareLessThanOrEqualTo(Cow::Borrowed(&self.a)))]
///     b: i32,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: 3,
///     b: 4,
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
pub struct CompareLessThanOrEqualTo<'a, T: Clone>(pub Cow<'a, T>);

impl<T, D, E, U> Validator<T, D, E> for CompareLessThanOrEqualTo<'_, U>
where
    T: PartialOrd<U> + Display,
    U: Clone + Display,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(other) = self;

        let mut child_report = Report::new(accessor);

        if target.le(other) {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message(format!(
                "is \"{target}\", which is not less than or equal to \"{other}\""
            ));
        }

        C::apply(parent_report, child_report)
    }
}

/// Validates the target is greater than a value.
///
/// Requires the target type to be an implementor of `PartialOrd<U>`
/// and `Display`, where U is the type of the literal / variable being
/// compared to.
///
/// Takes a `Cow<'_, U>` for field `0`.
///
/// # Examples
/// ```rust
/// use std::borrow::Cow;
///
/// use vate::{path, Accessor, CompareGreaterThan, Everything, Report, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(CompareGreaterThan(Cow::Owned(3)))]
///     a: i32,
///     #[vate(CompareGreaterThan(Cow::Borrowed(&self.a)))]
///     b: i32,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: 5,
///     b: 1,
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
pub struct CompareGreaterThan<'a, T: Clone>(pub Cow<'a, T>);

impl<T, D, E, U> Validator<T, D, E> for CompareGreaterThan<'_, U>
where
    T: PartialOrd<U> + Display,
    U: Clone + Display,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(other) = self;

        let mut child_report = Report::new(accessor);

        if target.gt(other) {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message(format!(
                "is \"{target}\", which is not greater than \"{other}\""
            ));
        }

        C::apply(parent_report, child_report)
    }
}

/// Validates the target is greater than or equal to a value.
///
/// Requires the target type to be an implementor of `PartialOrd<U>`
/// and `Display`, where U is the type of the literal / variable being
/// compared to.
///
/// Takes a `Cow<'_, U>` for field `0`.
///
/// # Fields / Arguments
/// `0`: the value the target is compared to, which must be an implementor of `Clone` and `Display`.
///
/// # Examples
/// ```rust
/// use std::borrow::Cow;
///
/// use vate::{path, Accessor, CompareGreaterThanOrEqualTo, Everything, Report, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(CompareGreaterThanOrEqualTo(Cow::Owned(3)))]
///     a: i32,
///     #[vate(CompareGreaterThanOrEqualTo(Cow::Borrowed(&self.a)))]
///     b: i32,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: 4,
///     b: 3,
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
pub struct CompareGreaterThanOrEqualTo<'a, T: Clone>(pub Cow<'a, T>);

impl<T, D, E, U> Validator<T, D, E> for CompareGreaterThanOrEqualTo<'_, U>
where
    T: PartialOrd<U> + Display,
    U: Clone + Display,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(other) = self;

        let mut child_report = Report::new(accessor);

        if target.ge(other) {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message(format!(
                "is \"{target}\", which is not greater than or equal to \"{other}\""
            ));
        }

        C::apply(parent_report, child_report)
    }
}

/// Validates the target is equal to a value.
///
/// Requires the target type to be an implementor of `PartialOrd<U>`
/// and `Display`, where U is the type of the literal / variable being
/// compared to.
///
/// Takes a `Cow<'_, U>` for field `0`.
///
/// # Examples
/// ```rust
/// use std::borrow::Cow;
///
/// use vate::{path, Accessor, CompareEqualTo, Everything, Report, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(CompareEqualTo(Cow::Owned(3)))]
///     a: i32,
///     #[vate(CompareEqualTo(Cow::Borrowed(&self.a)))]
///     b: i32,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: 3,
///     b: 4,
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
pub struct CompareEqualTo<'a, T: Clone>(pub Cow<'a, T>);

impl<T, D, E, U> Validator<T, D, E> for CompareEqualTo<'_, U>
where
    T: PartialEq<U> + Display,
    U: Clone + Display,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(other) = self;

        let mut child_report = Report::new(accessor);

        if target.eq(other) {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message(format!(
                "is \"{target}\", which is not equal to \"{other}\""
            ));
        }

        C::apply(parent_report, child_report)
    }
}

/// Validates the target is not equal to a value.
///
/// Requires the target type to be an implementor of `PartialOrd<U>`
/// and `Display`, where U is the type of the literal / variable being
/// compared to.
///
/// Takes a `Cow<'_, U>` for field `0`.
///
/// # Examples
/// ```rust
/// use std::borrow::Cow;
///
/// use vate::{path, Accessor, CompareNotEqualTo, Everything, Report, Validate};
///
/// #[derive(Validate)]
/// struct Example {
///     #[vate(CompareNotEqualTo(Cow::Owned(3)))]
///     a: i32,
///     #[vate(CompareNotEqualTo(Cow::Borrowed(&self.a)))]
///     b: i32,
/// }
///
/// let mut report = Report::new(Accessor::Root("example"));
///
/// let example = Example {
///     a: 4,
///     b: 4,
/// };
///
/// let _ = example.validate::<Everything>(&(), &mut report);
///
/// assert!(report.is_all_valid_at_path(path!(example.a)).unwrap());
/// assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
/// ```
pub struct CompareNotEqualTo<'a, T: Clone>(pub Cow<'a, T>);

impl<T, D, E, U> Validator<T, D, E> for CompareNotEqualTo<'_, U>
where
    T: PartialEq<U> + Display,
    U: Clone + Display,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(other) = self;

        let mut child_report = Report::new(accessor);

        if target.ne(other) {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message(format!("is \"{target}\", which is equal to \"{other}\""));
        }

        C::apply(parent_report, child_report)
    }
}
