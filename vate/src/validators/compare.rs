//! Comparison validators.

use std::borrow::Borrow;

use crate::core::*;

/// Validates the target is less than field `0`.
pub struct LT<T>(pub T);

impl LT<()> {
    pub const VALIDATOR_NAME: &'static str = "LT";

    pub const DEFAULT_VALIDATOR_VARIANT: u8 = 0;
    pub const OTHER_VALUE_DETAIL_INDEX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for LT<U>
where
    T: PartialOrd,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        self,
        target: &T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        interpreter_data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let other = self.0;

        if target.lt(other.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.add_invalid(
                invalid.add_validation(
                    ValidatorIdent {
                        name: LT::VALIDATOR_NAME,
                        variant: LT::DEFAULT_VALIDATOR_VARIANT,
                    },
                    Details::default().set_detail(LT::OTHER_VALUE_DETAIL_INDEX, &other),
                ),
                interpreter,
                interpreter_data,
            ))
        }
    }
}

/// Validates the target is less than or equal to field `0`.
pub struct LE<T>(pub T);

impl LE<()> {
    pub const VALIDATOR_NAME: &'static str = "LE";

    pub const DEFAULT_VALIDATOR_VARIANT: u8 = 0;
    pub const OTHER_VALUE_DETAIL_INDEX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for LE<U>
where
    T: PartialOrd,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        self,
        target: &T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        interpreter_data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let other = self.0;

        if target.le(other.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.add_invalid(
                invalid.add_validation(
                    ValidatorIdent {
                        name: LE::VALIDATOR_NAME,
                        variant: LE::DEFAULT_VALIDATOR_VARIANT,
                    },
                    Details::default().set_detail(LE::OTHER_VALUE_DETAIL_INDEX, &other),
                ),
                interpreter,
                interpreter_data,
            ))
        }
    }
}

/// Validates the target is greater than field `0`.
pub struct GT<T>(pub T);

impl GT<()> {
    pub const VALIDATOR_NAME: &'static str = "GT";

    pub const DEFAULT_VALIDATOR_VARIANT: u8 = 0;
    pub const OTHER_VALUE_DETAIL_INDEX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for GT<U>
where
    T: PartialOrd,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        self,
        target: &T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        interpreter_data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let other = self.0;

        if target.gt(other.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.add_invalid(
                invalid.add_validation(
                    ValidatorIdent {
                        name: GT::VALIDATOR_NAME,
                        variant: GT::DEFAULT_VALIDATOR_VARIANT,
                    },
                    Details::default().set_detail(GT::OTHER_VALUE_DETAIL_INDEX, &other),
                ),
                interpreter,
                interpreter_data,
            ))
        }
    }
}

/// Validates the target is greater than or equal to field `0`.
pub struct GE<T>(pub T);

impl GE<()> {
    pub const VALIDATOR_NAME: &'static str = "GE";

    pub const DEFAULT_VALIDATOR_VARIANT: u8 = 0;
    pub const OTHER_VALUE_DETAIL_INDEX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for GE<U>
where
    T: PartialOrd,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        self,
        target: &T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        interpreter_data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let other = self.0;

        if target.ge(other.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.add_invalid(
                invalid.add_validation(
                    ValidatorIdent {
                        name: GE::VALIDATOR_NAME,
                        variant: GE::DEFAULT_VALIDATOR_VARIANT,
                    },
                    Details::default().set_detail(GE::OTHER_VALUE_DETAIL_INDEX, &other),
                ),
                interpreter,
                interpreter_data,
            ))
        }
    }
}

/// Validates the target is equal to field `0`.
pub struct EQ<T>(pub T);

impl EQ<()> {
    pub const VALIDATOR_NAME: &'static str = "EQ";

    pub const DEFAULT_VALIDATOR_VARIANT: u8 = 0;
    pub const OTHER_VALUE_DETAIL_INDEX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for EQ<U>
where
    T: PartialEq,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        self,
        target: &T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        interpreter_data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let other = self.0;

        if target.eq(other.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.add_invalid(
                invalid.add_validation(
                    ValidatorIdent {
                        name: EQ::VALIDATOR_NAME,
                        variant: EQ::DEFAULT_VALIDATOR_VARIANT,
                    },
                    Details::default().set_detail(EQ::OTHER_VALUE_DETAIL_INDEX, &other),
                ),
                interpreter,
                interpreter_data,
            ))
        }
    }
}

/// Validates the target is not equal to field `0`.
pub struct NE<T>(pub T);

impl NE<()> {
    pub const VALIDATOR_NAME: &'static str = "NE";

    pub const DEFAULT_VALIDATOR_VARIANT: u8 = 0;
    pub const OTHER_VALUE_DETAIL_INDEX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for NE<U>
where
    T: PartialEq,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        self,
        target: &T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        interpreter_data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let other = self.0;

        if target.ne(other.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.add_invalid(
                invalid.add_validation(
                    ValidatorIdent {
                        name: NE::VALIDATOR_NAME,
                        variant: NE::DEFAULT_VALIDATOR_VARIANT,
                    },
                    Details::default().set_detail(NE::OTHER_VALUE_DETAIL_INDEX, &other),
                ),
                interpreter,
                interpreter_data,
            ))
        }
    }
}

/// Validates the target is between `min` and `max`.
pub struct Within<T> {
    pub min: T,
    pub max: T,
}

impl Within<()> {
    pub const VALIDATOR_NAME: &'static str = "Within";

    pub const DEFAULT_VALIDATOR_VARIANT: u8 = 0;
    pub const MIN_VALUE_DETAIL_INDEX: usize = 0;
    pub const MAX_VALUE_DETAIL_INDEX: usize = 1;
}

impl<T, C, E, U> Validator<&T, C, E> for Within<U>
where
    T: PartialOrd,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        self,
        target: &T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        interpreter_data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let Self { min, max } = &self;

        if target.ge(min.borrow()) && target.le(max.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.add_invalid(
                invalid.add_validation(
                    ValidatorIdent {
                        name: Within::VALIDATOR_NAME,
                        variant: Within::DEFAULT_VALIDATOR_VARIANT,
                    },
                    Details::default()
                        .set_detail(Within::MIN_VALUE_DETAIL_INDEX, min)
                        .set_detail(Within::MAX_VALUE_DETAIL_INDEX, max),
                ),
                interpreter,
                interpreter_data,
            ))
        }
    }
}
