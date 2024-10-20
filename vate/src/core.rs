use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// Allows the implementor to be validated.
pub trait Validate {
    /// Custom data type passed to validators.
    type Data;
    /// Error data type that validators may add to a report or return.
    type Error;
    /// Validate the target.
    fn validate<C: Collector<Self::Error>>(
        &self,
        data: &Self::Data,
        parent_report: &mut Report<Self::Error>,
    ) -> Result<(), Exit<Self::Error>>;
}

/// Defines a validator.
pub trait Validator<T, D, E> {
    /// Run the validator.
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>>;
}

/// A validation report.
#[derive(Debug)]
pub struct Report<E> {
    /// The accessor of the report.
    accessor: Accessor,
    /// The validity determined after validating.
    validity: Result<bool, E>,
    /// The message associated with the report.
    message: String,
    /// The children of this report.
    children: Vec<Report<E>>,
}

impl<E> Report<E> {
    /// Create a new report.
    pub fn new(accessor: Accessor) -> Self {
        Self {
            accessor,
            validity: Ok(true),
            message: String::new(),
            children: Vec::new(),
        }
    }

    /// Get the report accessor.
    ///
    /// # Examples
    /// ```rust
    /// use vate::{Accessor, Report};
    ///
    /// let report: Report<()> = Report::new(Accessor::Root("example"));
    /// assert!(matches!(report.get_accessor(), Accessor::Root("example")));
    /// ```
    pub fn get_accessor(&self) -> &Accessor {
        &self.accessor
    }

    /// Set the validity of the report.
    ///
    /// # Examples
    /// ```rust
    /// use vate::{Accessor, Report};
    ///
    /// let mut report: Report<()> = Report::new(Accessor::Root("example"));
    ///
    /// report.set_validity(Ok(true));
    /// assert!(report.is_valid());
    ///
    /// report.set_validity(Ok(false));
    /// assert!(report.is_invalid());
    ///
    /// report.set_validity(Err(()));
    /// assert!(report.is_error());
    /// ```
    pub fn set_validity(&mut self, validity: Result<bool, E>) {
        self.validity = validity;
    }

    /// Set the report validity to valid.
    ///
    /// # Examples
    /// ```rust
    /// use vate::{Accessor, Report};
    ///
    /// let mut report: Report<()> = Report::new(Accessor::Root("example"));
    ///
    /// report.set_valid();
    /// assert!(report.is_valid());
    /// ```
    pub fn set_valid(&mut self) {
        self.set_validity(Ok(true));
    }

    /// Set the report validity to invalid.
    ///
    /// # Examples
    /// ```rust
    /// use vate::{Accessor, Report};
    ///
    /// let mut report: Report<()> = Report::new(Accessor::Root("example"));
    ///
    /// report.set_invalid();
    /// assert!(report.is_invalid());
    /// ```
    pub fn set_invalid(&mut self) {
        self.set_validity(Ok(false));
    }

    /// Set the report validity to an error.
    ///
    /// # Examples
    /// ```rust
    /// use vate::{Accessor, Report};
    ///
    /// let mut report: Report<()> = Report::new(Accessor::Root("example"));
    ///
    /// report.set_error(());
    /// assert!(report.is_error());
    /// ```
    pub fn set_error(&mut self, error: E) {
        self.set_validity(Err(error));
    }

    /// Get the validity of this report.
    ///
    /// # Examples
    /// ```rust
    /// use vate::{Accessor, Report};
    ///
    /// struct Error;
    ///
    /// let mut report: Report<Error> = Report::new(Accessor::Root("example"));
    ///
    /// report.set_validity(Ok(true));
    /// assert!(matches!(report.get_validity(), Ok(true)));
    ///
    /// report.set_validity(Ok(false));
    /// assert!(matches!(report.get_validity(), Ok(false)));
    ///
    /// report.set_validity(Err(Error));
    /// assert!(matches!(report.get_validity(), Err(Error)));
    /// ```
    pub fn get_validity(&self) -> &Result<bool, E> {
        &self.validity
    }

    /// Check if the validity of this report is valid.
    pub fn is_valid(&self) -> bool {
        matches!(self.get_validity(), Ok(true))
    }

    /// Check if the validity of this report is invalid.
    pub fn is_invalid(&self) -> bool {
        matches!(self.get_validity(), Ok(false))
    }

    /// Check if the validity of this report is an error.
    pub fn is_error(&self) -> bool {
        self.get_validity().is_err()
    }

    /// Set the message of this report.
    pub fn set_message(&mut self, message: impl Into<String>) {
        self.message = message.into();
    }

    /// Get the message of this report.
    pub fn get_message(&self) -> &String {
        &self.message
    }

    /// Push a child report to this report.
    ///
    /// This is typically called by collectors!
    pub fn push_child(&mut self, child: Report<E>) {
        self.children.push(child);
    }

    /// Get child reports given an accessor.
    pub fn get_children_at_accessor<'a>(
        &'a self,
        accessor: &'a Accessor,
    ) -> impl Iterator<Item = &'a Report<E>> + '_ {
        self.children
            .iter()
            .filter(move |child| child.get_accessor() == accessor)
    }

    /// Get child reports given a path.
    pub fn get_children_at_path<'a>(&'a self, path: &'a [Accessor]) -> Vec<&'a Report<E>> {
        let mut children = Vec::new();
        if let Some((first, rest)) = path.as_ref().split_first() {
            if let Some(next) = rest.first() {
                for child in self.get_children_at_accessor(next) {
                    children.extend(child.get_children_at_path(rest));
                }
            } else if *first == self.accessor {
                children.push(self)
            }
        }
        children
    }

    /// Get the validities of a path in the report.
    ///
    /// If the path isn't found, an empty vec is returned.
    /// This does NOT mean the path doesn't exist. It just means it is
    /// not in the report. This can be due to many reasons, such as because nothing on
    /// that path was validated, the validation was skipped, etc.
    pub fn get_validities_at_path<'a>(&'a self, path: &'a [Accessor]) -> Vec<&'a Result<bool, E>> {
        self.get_children_at_path(path)
            .into_iter()
            .map(|child| child.get_validity())
            .collect()
    }

    /// Check if **all** of the nested reports at the path are valid.
    ///
    /// If the path isn't found, `None` is returned.
    /// This does NOT mean the path doesn't exist. It just means it is
    /// not in the report. This can be due to many reasons, such as because nothing on
    /// that path was validated, the validation was skipped, etc.
    pub fn is_all_valid_at_path(&self, path: &[Accessor]) -> Option<bool> {
        let validities = self.get_validities_at_path(path);
        (!validities.is_empty()).then_some(
            validities
                .iter()
                .all(|validity| matches!(validity, Ok(true))),
        )
    }

    /// Check if **any** of the nested reports at the path are invalid.
    ///
    /// If the path isn't found, `None` is returned.
    /// This does NOT mean the path doesn't exist. It just means it is
    /// not in the report. This can be due to many reasons, such as because nothing on
    /// that path was validated, the validation was skipped, etc.
    pub fn is_any_invalid_at_path(&self, path: &[Accessor]) -> Option<bool> {
        let validities = self.get_validities_at_path(path);
        (!validities.is_empty()).then_some(
            validities
                .iter()
                .any(|validity| matches!(validity, Ok(false))),
        )
    }

    /// Check if **any** of the nested reports at the path are erroneous.
    ///
    /// If the path isn't found, `None` is returned.
    /// This does NOT mean the path doesn't exist. It just means it is
    /// not in the report. This can be due to many reasons, such as because nothing on
    /// that path was validated, the validation was skipped, etc.
    pub fn is_any_error_at_path(&self, path: &[Accessor]) -> Option<bool> {
        let validities = self.get_validities_at_path(path);
        (!validities.is_empty()).then_some(validities.iter().any(|validity| validity.is_err()))
    }

    /// Check if the path is not in the report.
    pub fn is_empty_at_path(&self, path: &[Accessor]) -> bool {
        self.get_validities_at_path(path).is_empty()
    }

    /// Count the number of reports.
    pub fn count_reports(&self) -> usize {
        let mut count = 1;
        for child in self.children.iter() {
            count += child.count_reports();
        }
        count
    }

    /// Get the number of leaf reports.
    ///
    /// # Examples
    /// ```rust
    /// use vate::{path, Accessor, Boolean, Everything, Report, Validate};
    ///
    /// #[derive(Validate)]
    /// struct Example {
    ///     #[vate(Boolean(true), Boolean(false))]
    ///     a: (),
    /// }
    ///
    /// let example = Example {
    ///     a: (),
    /// };
    ///
    /// let mut report = Report::new(Accessor::Root("example"));
    /// let _ = example.validate::<Everything>(&(), &mut report);
    ///
    /// assert_eq!(report.count_leaves(), 2);
    /// ```
    pub fn count_leaves(&self) -> usize {
        if self.children.is_empty() {
            1
        } else {
            let mut count = 0;
            for child in self.children.iter() {
                count += child.count_leaves();
            }
            count
        }
    }

    /// Get the number of leaf reports at a path.
    ///
    /// # Examples
    /// ```rust
    /// use vate::{path, Accessor, Boolean, Everything, Report, Validate};
    ///
    /// #[derive(Validate)]
    /// struct Example {
    ///     #[vate(Boolean(true), Boolean(false))]
    ///     a: (),
    /// }
    ///
    /// let example = Example {
    ///     a: (),
    /// };
    ///
    /// let mut report = Report::new(Accessor::Root("example"));
    /// let _ = example.validate::<Everything>(&(), &mut report);
    ///
    /// assert_eq!(report.count_leaves_at_path(&path!(example)), 2);
    /// assert_eq!(report.count_leaves_at_path(&path!(example.a)), 2);
    /// ```
    pub fn count_leaves_at_path(&self, path: &[Accessor]) -> usize {
        if self.children.is_empty() {
            1
        } else {
            let mut count = 0;
            for child in self.get_children_at_path(path) {
                count += child.count_leaves();
            }
            count
        }
    }

    /// Get the number of leaf reports at a path that are valid.
    ///
    /// # Examples
    /// ```rust
    /// use vate::{path, Accessor, Boolean, Everything, Report, Validate};
    ///
    /// #[derive(Validate)]
    /// struct Example {
    ///     #[vate(Boolean(true), Boolean(false), Boolean(false))]
    ///     a: (),
    /// }
    ///
    /// let example = Example {
    ///     a: (),
    /// };
    ///
    /// let mut report = Report::new(Accessor::Root("example"));
    /// let _ = example.validate::<Everything>(&(), &mut report);
    ///
    /// assert_eq!(report.count_leaves_at_path(&path!(example.a)), 3);
    /// assert_eq!(report.count_valid_leaves_at_path(&path!(example.a)), 1);
    /// ```
    pub fn count_valid_leaves_at_path(&self, path: &[Accessor]) -> usize {
        if self.children.is_empty() && self.is_valid() {
            1
        } else {
            let mut count = 0;
            for child in self.get_children_at_path(path) {
                count += child.count_valid_leaves_at_path(path);
            }
            count
        }
    }

    /// Get the number of leaf reports at a path that are invalid.
    ///
    /// # Examples
    /// ```rust
    /// use vate::{path, Accessor, Boolean, Everything, Report, Validate};
    ///
    /// #[derive(Validate)]
    /// struct Example {
    ///     #[vate(Boolean(true), Boolean(true), Boolean(false))]
    ///     a: (),
    /// }
    ///
    /// let example = Example {
    ///     a: (),
    /// };
    ///
    /// let mut report = Report::new(Accessor::Root("example"));
    /// let _ = example.validate::<Everything>(&(), &mut report);
    ///
    /// assert_eq!(report.count_leaves_at_path(&path!(example.a)), 3);
    /// assert_eq!(report.count_invalid_leaves_at_path(&path!(example.a)), 1);
    /// ```
    pub fn count_invalid_leaves_at_path(&self, path: &[Accessor]) -> usize {
        if self.children.is_empty() && self.is_invalid() {
            1
        } else {
            let mut count = 0;
            for child in self.get_children_at_path(path) {
                count += child.count_invalid_leaves_at_path(path);
            }
            count
        }
    }

    /// Get the number of leaf reports at a path that are valid.
    ///
    /// # Examples
    /// ```rust
    /// use vate::{path, Accessor, Report};
    ///
    /// struct Error;
    ///
    /// let mut report = Report::new(Accessor::Root("example"));
    ///
    /// let mut child_report = Report::new(Accessor::Field("a"));
    /// child_report.set_error(Error);
    /// report.push_child(child_report);
    ///
    /// report.push_child(Report::new(Accessor::Field("a")));
    /// report.push_child(Report::new(Accessor::Field("a")));
    ///
    /// assert_eq!(report.count_leaves_at_path(&path!(example.a)), 3);
    /// assert_eq!(report.count_error_leaves_at_path(&path!(example.a)), 1);
    /// ```
    pub fn count_error_leaves_at_path(&self, path: &[Accessor]) -> usize {
        if self.children.is_empty() && self.is_error() {
            1
        } else {
            let mut count = 0;
            for child in self.get_children_at_path(path) {
                count += child.count_error_leaves_at_path(path);
            }
            count
        }
    }

    /// A method used by `<Report as Display>::fmt` to stringify the report.
    fn stringify(&self, current_path: Option<Vec<&Accessor>>) -> String {
        let mut stringified = String::new();

        let mut current_path = current_path.unwrap_or_default();
        current_path.push(&self.accessor);

        if !self.get_message().is_empty() {
            let mut current_path_string = String::new();
            for accessor in current_path.iter() {
                current_path_string.push_str(&accessor.to_string());
            }
            stringified.push_str(&format!("{current_path_string} {}\n", self.get_message()));
        }

        for child in self.children.iter() {
            stringified.push_str(&child.stringify(Some(current_path.clone())));
        }

        stringified
    }
}

impl<E> Display for Report<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.stringify(None))
    }
}

/// A segment of a path to a validated target.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Accessor {
    Root(&'static str),
    Variant(&'static str),
    Field(&'static str),
    TupleIndex(usize),
    Index(usize),
    Key(String),
}

impl Display for Accessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Root(root) => write!(f, "{root}"),
            Self::Variant(variant) => write!(f, "[{variant}]"),
            Self::Field(field) => write!(f, ".{field}"),
            Self::TupleIndex(tuple_index) => write!(f, ".{tuple_index}"),
            Self::Index(index) => write!(f, "[{index}]"),
            Self::Key(key) => write!(f, "[\"{key}\"]"),
        }
    }
}

/// Defines how a parent report collects a child report.
pub trait Collector<E> {
    fn apply(parent: &mut Report<E>, child: Report<E>) -> Result<(), Exit<E>>;
}

/// An exit "error" that acts as a control flow within validators, collectors, etc.
///
/// For example, the `FirstInvalidAndPrecedingErrors` validator exits gracefully
/// as soon as the first invalid is encountered. The validators following this invalid
/// will not be ran, which can be good for performance if you only want the first invalid
/// anyway.
#[derive(Debug)]
pub enum Exit<E> {
    /// Exit gracefully.
    Gracefully,
    /// Exit with an error.
    /// This is different from pushing an error to a report, and is intended for force-exiting if a fatal error is encountered.
    WithError(E),
}
