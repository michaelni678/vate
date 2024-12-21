//! String validators.

use crate::core::*;

#[cfg(feature = "regex")]
use crate::extras::Regex;

/// Validates the target contains only alphabetic characters.
pub struct Alphabetic;

impl Alphabetic {
    pub const DEFAULT_VTAG: ValidatorTag = "m=string;v=Alphabetic";
    pub const TARGET_VALUE_DIDX: usize = 0;
}

impl<T, C, E> Validator<T, C, E> for Alphabetic
where
    T: AsRef<str>,
{
    fn run<D, R>(
        &self,
        target: T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let target = target.as_ref();

        if target.chars().all(char::is_alphabetic) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    Alphabetic::DEFAULT_VTAG,
                    Detailer::default().set_detail(Alphabetic::TARGET_VALUE_DIDX, &target),
                ),
                interpreter,
                data,
            ))
        }
    }
}

/// Validates the target contains only alphanumeric characters.
pub struct Alphanumeric;

impl Alphanumeric {
    pub const DEFAULT_VTAG: ValidatorTag = "m=string;v=Alphanumeric";
    pub const TARGET_VALUE_DIDX: usize = 0;
}

impl<T, C, E> Validator<T, C, E> for Alphanumeric
where
    T: AsRef<str>,
{
    fn run<D, R>(
        &self,
        target: T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let target = target.as_ref();

        if target.chars().all(char::is_alphanumeric) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    Alphanumeric::DEFAULT_VTAG,
                    Detailer::default().set_detail(Alphanumeric::TARGET_VALUE_DIDX, &target),
                ),
                interpreter,
                data,
            ))
        }
    }
}

/// Validates the target contains only ASCII characters.
pub struct ASCII;

impl ASCII {
    pub const DEFAULT_VTAG: ValidatorTag = "m=string;v=ASCII";
    pub const TARGET_VALUE_DIDX: usize = 0;
}

impl<T, C, E> Validator<T, C, E> for ASCII
where
    T: AsRef<str>,
{
    fn run<D, R>(
        &self,
        target: T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let target = target.as_ref();

        if target.is_ascii() {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    ASCII::DEFAULT_VTAG,
                    Detailer::default().set_detail(ASCII::TARGET_VALUE_DIDX, &target),
                ),
                interpreter,
                data,
            ))
        }
    }
}

/// Validates the target contains only lowercase characters.
pub struct Lowercase;

impl Lowercase {
    pub const DEFAULT_VTAG: ValidatorTag = "m=string;v=Lowercase";
    pub const TARGET_VALUE_DIDX: usize = 0;
}

impl<T, C, E> Validator<T, C, E> for Lowercase
where
    T: AsRef<str>,
{
    fn run<D, R>(
        &self,
        target: T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let target = target.as_ref();

        if target.chars().all(char::is_lowercase) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    Lowercase::DEFAULT_VTAG,
                    Detailer::default().set_detail(Lowercase::TARGET_VALUE_DIDX, &target),
                ),
                interpreter,
                data,
            ))
        }
    }
}

/// Validates the target contains only uppercase characters.
pub struct Uppercase;

impl Uppercase {
    pub const DEFAULT_VTAG: ValidatorTag = "m=string;v=Uppercase";
    pub const TARGET_VALUE_DIDX: usize = 0;
}

impl<T, C, E> Validator<T, C, E> for Uppercase
where
    T: AsRef<str>,
{
    fn run<D, R>(
        &self,
        target: T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let target = target.as_ref();

        if target.chars().all(char::is_uppercase) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    Uppercase::DEFAULT_VTAG,
                    Detailer::default().set_detail(Uppercase::TARGET_VALUE_DIDX, &target),
                ),
                interpreter,
                data,
            ))
        }
    }
}

/// Forwards the length of the target to the inner validator.
pub enum Length<V> {
    Bytes(V),
    Chars(V),
}

impl Length<()> {
    pub const BYTES_VTAG: ValidatorTag = "m=string;v=Length;t=Bytes";
    pub const BYTES_TARGET_LENGTH_DIDX: usize = 0;

    pub const CHARS_VTAG: ValidatorTag = "m=string;v=Length;t=Chars";
    pub const CHARS_TARGET_LENGTH_DIDX: usize = 0;
}

impl<T, C, E, V> Validator<T, C, E> for Length<V>
where
    T: AsRef<str>,
    for<'a> V: Validator<&'a usize, C, E>,
{
    fn run<D, R>(
        &self,
        target: T,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let target = target.as_ref();

        match self {
            Self::Bytes(v) => {
                let len = target.len();

                v.run(
                    &len,
                    context,
                    invalid.push_validation(
                        Length::BYTES_VTAG,
                        Detailer::default().set_detail(Length::BYTES_TARGET_LENGTH_DIDX, &len),
                    ),
                    interpreter,
                    data,
                    report,
                )
            }
            Self::Chars(v) => {
                let len = target.chars().count();

                v.run(
                    &len,
                    context,
                    invalid.push_validation(
                        Length::CHARS_VTAG,
                        Detailer::default().set_detail(Length::CHARS_TARGET_LENGTH_DIDX, &len),
                    ),
                    interpreter,
                    data,
                    report,
                )
            }
        }
    }
}

/// Validates the target matches the given [`Regex`].
#[cfg(feature = "regex")]
pub struct MatchesRegex<'a>(pub &'a Regex);

#[cfg(feature = "regex")]
impl MatchesRegex<'_> {
    pub const DEFAULT_VTAG: ValidatorTag = "m=string;v=MatchesRegex";
    pub const TARGET_VALUE_DIDX: usize = 0;
    pub const REGEX_DIDX: usize = 0;
}

#[cfg(feature = "regex")]
impl<T, C, E> Validator<T, C, E> for MatchesRegex<'_>
where
    T: AsRef<str>,
{
    fn run<D, R>(
        &self,
        target: T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let regex = self.0;
        let target = target.as_ref();

        if regex.is_match(target) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    MatchesRegex::DEFAULT_VTAG,
                    Detailer::default()
                        .set_detail(MatchesRegex::TARGET_VALUE_DIDX, &target)
                        .set_detail(MatchesRegex::REGEX_DIDX, &regex),
                ),
                interpreter,
                data,
            ))
        }
    }
}
