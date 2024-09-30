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
    pub fn is_valid_at_path(&self, path: impl AsRef<[Accessor]>) -> bool {
        if let Some(Ok(validity)) = self.validity_at_path(path) {
            return *validity;
        }
        false
    }
    pub fn is_invalid_at_path(&self, path: impl AsRef<[Accessor]>) -> bool {
        if let Some(Ok(validity)) = self.validity_at_path(path) {
            return !*validity;
        }
        false
    }
    pub fn is_error_at_path(&self, path: impl AsRef<[Accessor]>) -> bool {
        if let Some(Err(_)) = self.validity_at_path(path) {
            return true;
        }
        false
    }
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
    fn apply(parent: &mut Report<E>, report: Report<E>) -> Result<(), Exit<E>>;
}

#[derive(Debug)]
pub enum Exit<E> {
    Gracefully,
    WithError(E),
}
