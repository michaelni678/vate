use crate::{Accessor, Collector, Exit, Report, Validator};

pub struct StringAlphabetic;

impl<T: AsRef<str>, D, E> Validator<T, D, E> for StringAlphabetic {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if target.as_ref().chars().all(char::is_alphabetic) {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message = String::from("contains non-alphabetic characters");
        }

        parent_report.push_child::<C>(child_report)
    }
}

pub struct StringAlphanumeric;

impl<T: AsRef<str>, D, E> Validator<T, D, E> for StringAlphanumeric {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if target.as_ref().chars().all(char::is_alphanumeric) {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message = String::from("contains non-alphanumeric characters");
        }

        parent_report.push_child::<C>(child_report)
    }
}

pub struct StringAscii;

impl<T: AsRef<str>, D, E> Validator<T, D, E> for StringAscii {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if target.as_ref().is_ascii() {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message = String::from("contains non-ascii characters");
        }

        parent_report.push_child::<C>(child_report)
    }
}

pub enum StringLengthEquals {
    Bytes(usize),
    Chars(usize),
}

impl<T: AsRef<str>, D, E> Validator<T, D, E> for StringLengthEquals {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let s = target.as_ref();
        let (unit, required_len, target_len) = match *self {
            Self::Bytes(required_len) => ("byte", required_len, s.len()),
            Self::Chars(required_len) => ("character", required_len, s.chars().count()),
        };

        let mut child_report = Report::new(accessor);

        if required_len == target_len {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message = format!("is not {required_len} {unit}s long");
        }

        parent_report.push_child::<C>(child_report)
    }
}

pub enum StringLengthRange {
    Bytes { min: usize, max: usize },
    Chars { min: usize, max: usize },
}

impl<T: AsRef<str>, D, E> Validator<T, D, E> for StringLengthRange {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: &T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let s = target.as_ref();
        let (unit, min, max, target_len) = match *self {
            Self::Bytes { min, max } => ("byte", min, max, s.len()),
            Self::Chars { min, max } => ("character", min, max, s.chars().count()),
        };

        let mut child_report = Report::new(accessor);

        if target_len >= min && target_len <= max {
            child_report.validity = Ok(true);
        } else {
            child_report.validity = Ok(false);
            child_report.message = format!("is not between {min} and {max} {unit}s long");
        }

        parent_report.push_child::<C>(child_report)
    }
}
