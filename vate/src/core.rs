use std::{
    borrow::Borrow,
    collections::HashSet,
    hash::{Hash, Hasher},
    ops::Deref,
};

pub trait Validate {
    type Data;
    type Error;
    fn validate<C: Collector<Self::Error>>(
        &self,
        data: &Self::Data,
        parent_report: &mut Report<Self::Error>,
    ) -> Result<(), Exit<Self::Error>>;
}

pub trait Validator<T, D, E> {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>>;
}

#[derive(Debug)]
pub struct Report<E> {
    pub accessor: Accessor,
    pub validity: Result<bool, E>,
    pub message: String,
    pub children: HashSet<ReportHasher<E>>,
}

impl<E> Report<E> {
    pub fn new(accessor: Accessor) -> Self {
        Self {
            accessor,
            validity: Ok(true),
            message: String::new(),
            children: HashSet::new(),
        }
    }
    pub fn push_child<C: Collector<E>>(&mut self, child: Self) -> Result<(), Exit<E>> {
        C::apply(self, child)
    }
    /// Get the validity of a path in the report.
    /// If the path isn't found, `None` is returned. If the path isn't found,
    /// this does NOT mean the struct does not have this path. It just means it is
    /// not in the report. This can be due to many reasons, such as because nothing on
    /// that path was validated, the validation was skipped, etc.
    pub fn validity_at_path(&self, path: impl AsRef<[Accessor]>) -> Option<&Result<bool, E>> {
        let (first, rest) = path.as_ref().split_first()?;
        if let Some(next) = rest.first() {
            self.children.get(next)?.validity_at_path(rest)
        } else {
            (*first == self.accessor).then_some(&self.validity)
        }
    }
}

impl<E> From<Report<E>> for ReportHasher<E> {
    fn from(report: Report<E>) -> Self {
        Self(report)
    }
}

#[derive(Debug)]
pub struct ReportHasher<E>(pub Report<E>);

impl<E> Deref for ReportHasher<E> {
    type Target = Report<E>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E> PartialEq for ReportHasher<E> {
    fn eq(&self, other: &Self) -> bool {
        self.accessor == other.accessor
    }
}

impl<E> Eq for ReportHasher<E> {}

impl<E> Hash for ReportHasher<E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.accessor.hash(state);
    }
}

impl<E> Borrow<Accessor> for ReportHasher<E> {
    fn borrow(&self) -> &Accessor {
        &self.accessor
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Accessor {
    Root(&'static str),
    Field(&'static str),
    Index(usize),
    Key(String),
}

pub trait Collector<E> {
    fn apply(parent: &mut Report<E>, child: Report<E>) -> Result<(), Exit<E>>;
}

#[derive(Debug)]
pub enum Exit<E> {
    Gracefully,
    WithError(E),
}
