//! Comparison validators.

use std::{
    borrow::Borrow,
    ops::{Range, RangeBounds, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive},
};

use crate::core::*;

/// Validates the target is less than field `0`.
pub struct LT<T>(pub T);

impl LT<()> {
    pub const DEFAULT_VTAG: &'static str = "m=compare;v=LT";
    pub const OTHER_VALUE_DIDX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for LT<U>
where
    T: PartialOrd,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        &self,
        target: &T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let other = &self.0;

        if target.lt(other.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    LT::DEFAULT_VTAG,
                    Detailer::default().set_detail(LT::OTHER_VALUE_DIDX, other),
                ),
                interpreter,
                data,
            ))
        }
    }
}

/// Validates the target is less than or equal to field `0`.
pub struct LE<T>(pub T);

impl LE<()> {
    pub const DEFAULT_VTAG: &'static str = "m=compare;v=LE";
    pub const OTHER_VALUE_DIDX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for LE<U>
where
    T: PartialOrd,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        &self,
        target: &T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let other = &self.0;

        if target.le(other.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    LE::DEFAULT_VTAG,
                    Detailer::default().set_detail(LE::OTHER_VALUE_DIDX, other),
                ),
                interpreter,
                data,
            ))
        }
    }
}

/// Validates the target is greater than field `0`.
pub struct GT<T>(pub T);

impl GT<()> {
    pub const DEFAULT_VTAG: &'static str = "m=compare;v=GT";
    pub const OTHER_VALUE_DIDX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for GT<U>
where
    T: PartialOrd,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        &self,
        target: &T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let other = &self.0;

        if target.gt(other.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    GT::DEFAULT_VTAG,
                    Detailer::default().set_detail(GT::OTHER_VALUE_DIDX, other),
                ),
                interpreter,
                data,
            ))
        }
    }
}

/// Validates the target is greater than or equal to field `0`.
pub struct GE<T>(pub T);

impl GE<()> {
    pub const DEFAULT_VTAG: &'static str = "m=compare;v=GE";
    pub const OTHER_VALUE_DIDX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for GE<U>
where
    T: PartialOrd,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        &self,
        target: &T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let other = &self.0;

        if target.ge(other.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    GE::DEFAULT_VTAG,
                    Detailer::default().set_detail(GE::OTHER_VALUE_DIDX, other),
                ),
                interpreter,
                data,
            ))
        }
    }
}

/// Validates the target is equal to field `0`.
pub struct EQ<T>(pub T);

impl EQ<()> {
    pub const DEFAULT_VTAG: &'static str = "m=compare;v=EQ";
    pub const OTHER_VALUE_DIDX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for EQ<U>
where
    T: PartialEq,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        &self,
        target: &T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let other = &self.0;

        if target.eq(other.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    EQ::DEFAULT_VTAG,
                    Detailer::default().set_detail(EQ::OTHER_VALUE_DIDX, other),
                ),
                interpreter,
                data,
            ))
        }
    }
}

/// Validates the target is not equal to field `0`.
pub struct NE<T>(pub T);

impl NE<()> {
    pub const DEFAULT_VTAG: &'static str = "m=compare;v=NE";
    pub const OTHER_VALUE_DIDX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for NE<U>
where
    T: PartialEq,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        &self,
        target: &T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let other = &self.0;

        if target.ne(other.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    NE::DEFAULT_VTAG,
                    Detailer::default().set_detail(NE::OTHER_VALUE_DIDX, other),
                ),
                interpreter,
                data,
            ))
        }
    }
}

/// Validates the target is contained in the given range.
pub struct Within<R>(pub R);

impl Within<()> {
    pub const RANGE_VTAG: &'static str = "m=compare;v=Within;t=Range";
    pub const RANGE_START_VALUE_DIDX: usize = 0;
    pub const RANGE_END_VALUE_DIDX: usize = 1;

    pub const RANGE_FROM_VTAG: &'static str = "m=compare;v=Within;t=RangeFrom";
    pub const RANGE_FROM_START_VALUE_DIDX: usize = 0;

    pub const RANGE_INCLUSIVE_VTAG: &'static str = "m=compare;v=Within;t=RangeInclusive";
    pub const RANGE_INCLUSIVE_START_VALUE_DIDX: usize = 0;
    pub const RANGE_INCLUSIVE_END_VALUE_DIDX: usize = 1;

    pub const RANGE_TO_VTAG: &'static str = "m=compare;v=Within;t=RangeTo";
    pub const RANGE_TO_END_VALUE_DIDX: usize = 0;

    pub const RANGE_TO_INCLUSIVE_VTAG: &'static str = "m=compare;v=Within;t=RangeToInclusive";
    pub const RANGE_TO_INCLUSIVE_END_VALUE_DIDX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for Within<Range<U>>
where
    T: PartialOrd<U>,
    U: PartialOrd<T> + ToString,
{
    fn run<D, R>(
        &self,
        target: &T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let range = &self.0;

        if range.contains(target) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    Within::RANGE_VTAG,
                    Detailer::default()
                        .set_detail(Within::RANGE_START_VALUE_DIDX, &range.start)
                        .set_detail(Within::RANGE_END_VALUE_DIDX, &range.end),
                ),
                interpreter,
                data,
            ))
        }
    }
}

impl<T, C, E, U> Validator<&T, C, E> for Within<RangeFrom<U>>
where
    T: PartialOrd<U>,
    U: PartialOrd<T> + ToString,
{
    fn run<D, R>(
        &self,
        target: &T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let range = &self.0;

        if range.contains(target) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    Within::RANGE_FROM_VTAG,
                    Detailer::default()
                        .set_detail(Within::RANGE_FROM_START_VALUE_DIDX, &range.start),
                ),
                interpreter,
                data,
            ))
        }
    }
}

impl<T, C, E, U> Validator<&T, C, E> for Within<RangeInclusive<U>>
where
    T: PartialOrd<U>,
    U: PartialOrd<T> + ToString,
{
    fn run<D, R>(
        &self,
        target: &T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let range = &self.0;

        if range.contains(target) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    Within::RANGE_INCLUSIVE_VTAG,
                    Detailer::default()
                        .set_detail(Within::RANGE_INCLUSIVE_START_VALUE_DIDX, range.start())
                        .set_detail(Within::RANGE_INCLUSIVE_END_VALUE_DIDX, range.end()),
                ),
                interpreter,
                data,
            ))
        }
    }
}

impl<T, C, E, U> Validator<&T, C, E> for Within<RangeTo<U>>
where
    T: PartialOrd<U>,
    U: PartialOrd<T> + ToString,
{
    fn run<D, R>(
        &self,
        target: &T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let range = &self.0;

        if range.contains(target) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    Within::RANGE_TO_VTAG,
                    Detailer::default().set_detail(Within::RANGE_TO_END_VALUE_DIDX, &range.end),
                ),
                interpreter,
                data,
            ))
        }
    }
}

impl<T, C, E, U> Validator<&T, C, E> for Within<RangeToInclusive<U>>
where
    T: PartialOrd<U>,
    U: PartialOrd<T> + ToString,
{
    fn run<D, R>(
        &self,
        target: &T,
        _context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let range = &self.0;

        if range.contains(target) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    Within::RANGE_TO_INCLUSIVE_VTAG,
                    Detailer::default()
                        .set_detail(Within::RANGE_TO_INCLUSIVE_END_VALUE_DIDX, &range.end),
                ),
                interpreter,
                data,
            ))
        }
    }
}
