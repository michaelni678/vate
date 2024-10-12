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
    pub fn get_accessor(&self) -> &Accessor {
        &self.accessor
    }

    /// Set the validity of the report.
    pub fn set_validity(&mut self, validity: Result<bool, E>) {
        self.validity = validity;
    }

    /// Set the report validity to valid.
    pub fn set_valid(&mut self) {
        self.set_validity(Ok(true));
    }

    /// Set the report validity to invalid.
    pub fn set_invalid(&mut self) {
        self.set_validity(Ok(false));
    }

    /// Set the report validity to an error.
    pub fn set_error(&mut self, error: E) {
        self.set_validity(Err(error));
    }

    /// Get the validity of this report.
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
    pub fn push_child(&mut self, child: Report<E>) {
        self.children.push(child);
    }

    /// Get child reports given an accessor.
    pub fn get_children_at_accessor<'a>(&'a self, accessor: &'a Accessor) -> impl Iterator<Item = &'a Report<E>> + '_ {
        self.children
            .iter()
            .filter(move |child| child.get_accessor() == accessor)
    }

    /// Get the validities of a path in the report.
    /// If the path isn't found, an empty vec is returned. 
    /// This does NOT mean the path doesn't exist. It just means it is
    /// not in the report. This can be due to many reasons, such as because nothing on
    /// that path was validated, the validation was skipped, etc.
    pub fn get_validities_at_path<'a>(&'a self, path: &'a [Accessor]) -> Vec<&'a Result<bool, E>> {
        let mut validities = Vec::new();
        if let Some((first, rest)) = path.as_ref().split_first() {
            if let Some(next) = rest.first() {
                for child in self.get_children_at_accessor(next) {
                    validities.extend(child.get_validities_at_path(rest));
                }
            } else {
                if *first == self.accessor {
                    validities.push(self.get_validity())
                }
            }
        }
        validities
    }

    /// Check if ALL of the nested reports at the path are valid. 
    /// If the path isn't found, `None` is returned. 
    /// This does NOT mean the path doesn't exist. It just means it is
    /// not in the report. This can be due to many reasons, such as because nothing on
    /// that path was validated, the validation was skipped, etc.
    pub fn is_all_valid_at_path(&self, path: impl AsRef<[Accessor]>) -> Option<bool> where E: Debug {
        let validities = self.get_validities_at_path(path.as_ref());
        (!validities.is_empty()).then_some(validities.iter().all(|validity| matches!(validity, Ok(true))))
    }

    /// Check if ANY of the nested reports at the path are invalid.
    /// If the path isn't found, `None` is returned. 
    /// This does NOT mean the path doesn't exist. It just means it is
    /// not in the report. This can be due to many reasons, such as because nothing on
    /// that path was validated, the validation was skipped, etc.
    pub fn is_any_invalid_at_path(&self, path: impl AsRef<[Accessor]>) -> Option<bool> {
        let validities = self.get_validities_at_path(path.as_ref());
        (!validities.is_empty()).then_some(validities.iter().any(|validity| matches!(validity, Ok(false))))
    }

    /// Check if ANY of the nested reports at the path are erroneous.
    /// If the path isn't found, `None` is returned. 
    /// This does NOT mean the path doesn't exist. It just means it is
    /// not in the report. This can be due to many reasons, such as because nothing on
    /// that path was validated, the validation was skipped, etc.
    pub fn is_any_error_at_path(&self, path: impl AsRef<[Accessor]>) -> Option<bool> {
        let validities = self.get_validities_at_path(path.as_ref());
        (!validities.is_empty()).then_some(validities.iter().any(|validity| validity.is_err()))
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
    Field(&'static str),
    Index(usize),
    Key(String),
}

impl Display for Accessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Root(root) => write!(f, "{root}"),
            Self::Field(field) => write!(f, ".{field}"),
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
/// For example, the `FirstInvalidAndPrecedingErrors` validator exits gracefully
/// as soon as the first invalid is encountered. The validators following this invalid
/// will not be ran, which can be good for performance if you only want the first invalid
/// anyway.
#[derive(Debug)]
pub enum Exit<E> {
    /// Exit gracefully. Although in the context of `Result<_, Exit<E>>` this is considered
    /// an error, Exit::Gracefully indicates that this behavior was expected.
    Gracefully,
    /// Exit with an error. This is different from pushing an error to a report, and
    /// is intended for force-exiting if a fatal error is encountered.
    WithError(E),
}
