use crate::{Accessor, Collector, Exit, Report, Validator};

pub struct StringAlphabetic;

impl<T: AsRef<str>, D, E> Validator<T, D, E> for StringAlphabetic {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if target.as_ref().chars().all(char::is_alphabetic) {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
        }

        C::apply(parent_report, child_report)
    }
}

pub struct StringAlphanumeric;

impl<T: AsRef<str>, D, E> Validator<T, D, E> for StringAlphanumeric {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if target.as_ref().chars().all(char::is_alphanumeric) {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
        }

        C::apply(parent_report, child_report)
    }
}

pub struct StringAscii;

impl<T: AsRef<str>, D, E> Validator<T, D, E> for StringAscii {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if target.as_ref().is_ascii() {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
        }

        C::apply(parent_report, child_report)
    }
}

pub struct StringAsciiAlphabetic;

impl<T: AsRef<str>, D, E> Validator<T, D, E> for StringAsciiAlphabetic {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if target
            .as_ref()
            .chars()
            .all(|c| char::is_ascii_alphabetic(&c))
        {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
        }

        C::apply(parent_report, child_report)
    }
}

pub struct StringAsciiDigit;

impl<T: AsRef<str>, D, E> Validator<T, D, E> for StringAsciiDigit {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if target.as_ref().chars().all(|c| char::is_ascii_digit(&c)) {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
        }

        C::apply(parent_report, child_report)
    }
}

pub struct StringLowercase;

impl<T: AsRef<str>, D, E> Validator<T, D, E> for StringLowercase {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if target.as_ref().chars().all(char::is_lowercase) {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
        }

        C::apply(parent_report, child_report)
    }
}

pub struct StringUppercase;

impl<T: AsRef<str>, D, E> Validator<T, D, E> for StringUppercase {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if target.as_ref().chars().all(char::is_uppercase) {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
        }

        C::apply(parent_report, child_report)
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
        target: T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let s = target.as_ref();
        let (required_len, target_len) = match *self {
            Self::Bytes(required_len) => (required_len, s.len()),
            Self::Chars(required_len) => (required_len, s.chars().count()),
        };

        let mut child_report = Report::new(accessor);

        if required_len == target_len {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
        }

        C::apply(parent_report, child_report)
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
        target: T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let s = target.as_ref();
        let (min, max, target_len) = match self {
            Self::Bytes { min, max } => (min, max, s.len()),
            Self::Chars { min, max } => (min, max, s.chars().count()),
        };

        let mut child_report = Report::new(accessor);

        if target_len >= *min && target_len <= *max {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
        }

        C::apply(parent_report, child_report)
    }
}

#[cfg(feature = "regex")]
pub struct StringMatchesRegex<'a>(pub &'a crate::extras::Regex);

#[cfg(feature = "regex")]
impl<'a, T: AsRef<str>, D, E> Validator<T, D, E> for StringMatchesRegex<'a> {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        target: T,
        _data: &D,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let Self(regex) = self;
        let target = target.as_ref();

        let mut child_report = Report::new(accessor);

        if regex.is_match(target) {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
        }

        C::apply(parent_report, child_report)
    }
}
