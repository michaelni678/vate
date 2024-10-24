use std::fmt::{self, Display, Formatter};

/// Allows the implementor to be validated.
pub trait Validate {
    /// Custom data type passed to validators.
    type Data;
    /// Error data type that validators may use.
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
        target: T,
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
    /// The children of this report.
    children: Vec<Report<E>>,
}

impl<E> Report<E> {
    /// Create a new validation report.
    pub fn new(accessor: Accessor) -> Self {
        Self {
            accessor,
            validity: Ok(true),
            children: Vec::new(),
        }
    }

    /// Get the accessor of the report.
    pub fn get_accessor(&self) -> &Accessor {
        &self.accessor
    }

    /// Set the validity.
    pub fn set_validity(&mut self, validity: Result<bool, E>) {
        self.validity = validity;
    }

    /// Set the validity to valid.
    pub fn set_valid(&mut self) {
        self.set_validity(Ok(true));
    }

    /// Set the validity to invalid.
    pub fn set_invalid(&mut self) {
        self.set_validity(Ok(false));
    }

    /// Set the validity to an error.
    pub fn set_error(&mut self, error: E) {
        self.set_validity(Err(error));
    }

    /// Get the validity.
    pub fn get_validity(&self) -> &Result<bool, E> {
        &self.validity
    }

    /// Check if validity is valid.
    pub fn is_valid(&self) -> bool {
        matches!(self.validity, Ok(true))
    }

    /// Check if validity is invalid.
    pub fn is_invalid(&self) -> bool {
        matches!(self.validity, Ok(false))
    }

    /// Check if validity is an error.
    pub fn is_error(&self) -> bool {
        self.validity.is_err()
    }

    /// Push a child report to this report.
    pub fn push_child(&mut self, child: Report<E>) {
        self.children.push(child);
    }

    /// Get the children of this report.
    pub fn get_children(&self) -> impl Iterator<Item = &Report<E>> {
        self.children.iter()
    }

    /// Get the children given an accessor.
    pub fn get_children_at_accessor<'a>(
        &'a self,
        accessor: &'a Accessor,
    ) -> impl Iterator<Item = &'a Report<E>> {
        self.get_children()
            .filter(move |child| child.get_accessor() == accessor)
    }

    /// Get children given a path. The current report's accessor should be
    /// at the front of the path.
    pub fn get_children_at_path<'a>(
        &'a self,
        path: &'a [Accessor],
    ) -> impl Iterator<Item = &Report<E>> {
        let mut found = Vec::new();
        if let Some((first, rest)) = path.split_first() {
            if let Some(next) = rest.first() {
                for child in self.get_children_at_accessor(next) {
                    found.extend(child.get_children_at_path(rest));
                }
            } else if first == self.get_accessor() {
                found.push(self)
            }
        }
        found.into_iter()
    }

    /// Get the leaves of the report.
    pub fn get_leaves(&self) -> impl Iterator<Item = &Report<E>> {
        let mut found = Vec::new();
        if self.children.is_empty() {
            found.push(self);
        } else {
            for child in self.get_children() {
                found.extend(child.get_leaves());
            }
        }
        found.into_iter()
    }
}

/// A segment of a path to a validated target.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Accessor {
    /// The root accessor, which is the original caller of `Validate::validate`.
    Root(&'static str),
    /// An enum variant.
    Variant(&'static str),
    /// A field.
    Field(&'static str),
    /// A tuple index.
    TupleIndex(usize),
    /// An index.
    Index(usize),
    /// A key.
    Key(String),
}

impl Display for Accessor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    /// Apply the collector.
    fn apply(parent: &mut Report<E>, child: Report<E>) -> Result<(), Exit<E>>;
}

/// An exit "error" that acts as a control flow within validators, collectors, etc.
#[derive(Debug)]
pub enum Exit<E> {
    /// Exit gracefully.
    Gracefully,
    /// Exit with an error.
    WithError(E),
}
