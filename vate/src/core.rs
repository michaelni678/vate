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
}

#[derive(Clone, Debug)]
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
