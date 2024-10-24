use crate::{Accessor, Collector, Exit, Report, Validator};

/// Iterates a collection, passing the iterator into the inner validator.
///
/// # Examples
/// ```rust
/// use std::collections::HashSet;
///
/// use vate::{path, Accessor, CollectionIterate, Collector, Everything, Exit, IteratorKeyed, Report, Validate, Validator};
///
/// /// A blacklisted people for a dinner party.
/// #[derive(Validate)]
/// #[vate(data = HashSet::<String>)]
/// struct Blacklist {
///     /// Names of people who are not allowed to attend the dinner party.
///     #[vate(CollectionIterate(IteratorKeyed(NotAnAttendee)))]
///     names: HashSet<String>,
/// }
///
/// /// Custom validator to check if a name is not in the list of attendees.
/// struct NotAnAttendee;
///
/// impl<E> Validator<&String, HashSet<String>, E> for NotAnAttendee {
///     fn run<C: Collector<E>>(
///         &self,
///         accessor: Accessor,
///         blacklisted_person: &String,
///         attendees: &HashSet<String>,
///         parent_report: &mut Report<E>,
///     ) -> Result<(), Exit<E>> {
///         let mut child_report = Report::new(accessor);
///
///         child_report.set_validity(Ok(!attendees.contains(blacklisted_person)));
///
///         C::apply(parent_report, child_report)
///     }
/// }
///
/// let blacklist = Blacklist {
///     names: HashSet::from([
///         String::from("Rob Burr"),
///         String::from("Van Dull"),
///         String::from("Ms. Conduct"),
///         String::from("Misty Meanor"),
///     ]),
/// };
///
/// let mut report = Report::new(Accessor::Root("blacklist"));
///
/// let attendees = HashSet::from([
///     String::from("Anita Beer"),
///     String::from("Brock O. Lee"),
///     String::from("Van Dull"), // On the blacklist.
///     String::from("Al Cahalek"),
///     String::from("Lynn Guini"),
///     String::from("Cara Melle"),
/// ]);
///
/// let _ = blacklist.validate::<Everything>(&attendees, &mut report);
///
/// assert_eq!(report.get_leaves().count(), 4);
///
/// assert_eq!(report.get_children_at_path(&path!(blacklist.names["Rob Burr"])).count(), 1);
/// assert!(report.get_children_at_path(&path!(blacklist.names["Rob Burr"])).all(|child| child.is_valid()));
///
/// assert_eq!(report.get_children_at_path(&path!(blacklist.names["Van Dull"])).count(), 1);
/// assert!(report.get_children_at_path(&path!(blacklist.names["Van Dull"])).all(|child| child.is_invalid()));
///
/// assert_eq!(report.get_children_at_path(&path!(blacklist.names["Ms. Conduct"])).count(), 1);
/// assert!(report.get_children_at_path(&path!(blacklist.names["Ms. Conduct"])).all(|child| child.is_valid()));
///
/// assert_eq!(report.get_children_at_path(&path!(blacklist.names["Misty Meanor"])).count(), 1);
/// assert!(report.get_children_at_path(&path!(blacklist.names["Misty Meanor"])).all(|child| child.is_valid()));
/// ```
pub struct CollectionIterate<V>(pub V);

impl<T, D, E, V> Validator<&T, D, E> for CollectionIterate<V>
where
    for<'a> &'a T: IntoIterator,
    for<'a> V: Validator<<&'a T as IntoIterator>::IntoIter, D, E>,
{
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(validator) = self;
        validator.run::<C>(accessor, target.into_iter(), data, parent_report)
    }
}
