use crate::{Accessor, Collector, Exit, Report, Validator};

/// Validates a boolean is `true`.
///
/// # Examples
/// ```rust
/// use vate::{path, Accessor, BooleanTrue, Everything, Report, Validate};
///
/// /// An electronic waiver of liability.
/// #[derive(Validate)]
/// struct WaiverOfLiability {
///     /// Whether the waiver has been completely scrolled through.
///     #[vate(BooleanTrue)]
///     scrolled_to_bottom: bool,
///     /// Whether the waiver has been acknowledged.
///     #[vate(BooleanTrue)]
///     is_acknowledged: bool,
/// }
///
/// let waiver = WaiverOfLiability {
///     scrolled_to_bottom: true,
///     is_acknowledged: false,
/// };
///
/// let mut report = Report::new(Accessor::Root("waiver"));
///
/// let _ = waiver.validate::<Everything>(&(), &mut report);
///
/// assert_eq!(report.get_leaves().count(), 2);
///
/// assert_eq!(report.get_children_at_path(&path!(waiver.scrolled_to_bottom)).count(), 1);
/// assert!(report.get_children_at_path(&path!(waiver.scrolled_to_bottom)).all(|child| child.is_valid()));
///
/// assert_eq!(report.get_children_at_path(&path!(waiver.is_acknowledged)).count(), 1);
/// assert!(report.get_children_at_path(&path!(waiver.is_acknowledged)).all(|child| child.is_invalid()));
/// ```
pub struct BooleanTrue;

impl<D, E> Validator<&bool, D, E> for BooleanTrue {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &bool,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if *target {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
        }

        C::apply(parent_report, child_report)
    }
}

/// Validates a boolean is `false`.
///
/// # Examples
/// ```rust
/// use vate::{path, Accessor, BooleanFalse, Everything, Report, Validate};
///
/// /// An application to rent an apartment.
/// #[derive(Validate)]
/// struct RentalApplication {
///     /// Whether the applicant has a criminal record.
///     #[vate(BooleanFalse)]
///     criminal_record: bool,
///     /// Whether the applicant has a history of eviction.
///     #[vate(BooleanFalse)]
///     history_of_eviction: bool,
/// }
///
/// let application = RentalApplication {
///     criminal_record: false,
///     history_of_eviction: true,
/// };
///
/// let mut report = Report::new(Accessor::Root("application"));
///
/// let _ = application.validate::<Everything>(&(), &mut report);
///
/// assert_eq!(report.get_leaves().count(), 2);
///
/// assert_eq!(report.get_children_at_path(&path!(application.criminal_record)).count(), 1);
/// assert!(report.get_children_at_path(&path!(application.criminal_record)).all(|child| child.is_valid()));
///
/// assert_eq!(report.get_children_at_path(&path!(application.history_of_eviction)).count(), 1);
/// assert!(report.get_children_at_path(&path!(application.history_of_eviction)).all(|child| child.is_invalid()));
/// ```
pub struct BooleanFalse;

impl<D, E> Validator<&bool, D, E> for BooleanFalse {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &bool,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if !*target {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
        }

        C::apply(parent_report, child_report)
    }
}
