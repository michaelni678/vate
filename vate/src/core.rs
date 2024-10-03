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
    pub children: Vec<Self>,
}

impl<E> Report<E> {
    pub fn new(accessor: Accessor) -> Self {
        Self {
            accessor,
            validity: Ok(true),
            message: String::new(),
            children: Vec::new(),
        }
    }
    pub fn push_child<C: Collector<E>>(&mut self, child: Self) -> Result<(), Exit<E>> {
        C::apply(self, child)
    }
    /// Check if the validity of the path is valid.
    /// If the path isn't found, `None` is returned. If the path isn't found,
    /// this does NOT mean the struct does not have this path. It just means it is
    /// not in the report. This can be due to many reasons, such as because nothing on
    /// that path was validated, the validation was skipped, etc.
    pub fn is_valid_at_path(&self, path: impl AsRef<[Accessor]>) -> Option<bool> {
        if let Ok(validity) = self.validity_at_path(path)? {
            return Some(*validity);
        }
        Some(false)
    }
    /// Check if the validity of the path is invalid.
    /// If the path isn't found, `None` is returned. If the path isn't found,
    /// this does NOT mean the struct does not have this path. It just means it is
    /// not in the report. This can be due to many reasons, such as because nothing on
    /// that path was validated, the validation was skipped, etc.
    pub fn is_invalid_at_path(&self, path: impl AsRef<[Accessor]>) -> Option<bool> {
        if let Ok(validity) = self.validity_at_path(path)? {
            return Some(!*validity);
        }
        Some(false)
    }
    /// Check if the validity of the path is an error.
    /// If the path isn't found, `None` is returned. If the path isn't found,
    /// this does NOT mean the struct does not have this path. It just means it is
    /// not in the report. This can be due to many reasons, such as because nothing on
    /// that path was validated, the validation was skipped, etc.
    pub fn is_error_at_path(&self, path: impl AsRef<[Accessor]>) -> Option<bool> {
        if (self.validity_at_path(path)?).is_err() {
            return Some(true);
        }
        Some(false)
    }
    /// Get the validity of a path in the report.
    /// If the path isn't found, `None` is returned. If the path isn't found,
    /// this does NOT mean the struct does not have this path. It just means it is
    /// not in the report. This can be due to many reasons, such as because nothing on
    /// that path was validated, the validation was skipped, etc.
    pub fn validity_at_path(&self, path: impl AsRef<[Accessor]>) -> Option<&Result<bool, E>> {
        let (first, rest) = path.as_ref().split_first()?;
        if first != &self.accessor {
            return None;
        } else if rest.is_empty() {
            return Some(&self.validity);
        }
        self.children
            .iter()
            .find_map(|child| child.validity_at_path(rest))
    }
}

#[derive(Clone, Debug, PartialEq)]
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
