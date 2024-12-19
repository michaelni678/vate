//! Comparison validators.

use std::borrow::Borrow;

use crate::core::*;

/// Validates the target is less than field `0`.
pub struct CompareLT<T>(pub T);

impl CompareLT<()> {
    pub const VALIDATOR_IDENT: &'static str = "CompareLT";
    pub const OTHER_VALUE_DETAIL_INDEX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for CompareLT<U>
where
    T: PartialOrd,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        self,
        validating_args: ValidatingArgs<&T, C>,
        invalid: Invalid,
        interpreting_args: InterpretingArgs<D>,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let other = self.0;

        if validating_args.target.lt(other.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.add_invalid(
                invalid.push(
                    ValidatorIdent::default().set_name(CompareLT::VALIDATOR_IDENT),
                    Details::default().set_detail(CompareLT::OTHER_VALUE_DETAIL_INDEX, &other),
                ),
                interpreting_args,
            ))
        }
    }
}

/// Validates the target is less than or equal to field `0`.
pub struct CompareLE<T>(pub T);

impl CompareLE<()> {
    pub const VALIDATOR_IDENT: &'static str = "CompareLE";
    pub const OTHER_VALUE_DETAIL_INDEX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for CompareLE<U>
where
    T: PartialOrd,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        self,
        validating_args: ValidatingArgs<&T, C>,
        invalid: Invalid,
        interpreting_args: InterpretingArgs<D>,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let other = self.0;

        if validating_args.target.le(other.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.add_invalid(
                invalid.push(
                    ValidatorIdent::default().set_name(CompareLE::VALIDATOR_IDENT),
                    Details::default().set_detail(CompareLE::OTHER_VALUE_DETAIL_INDEX, &other),
                ),
                interpreting_args,
            ))
        }
    }
}

/// Validates the target is greater than field `0`.
pub struct CompareGT<T>(pub T);

impl CompareGT<()> {
    pub const VALIDATOR_IDENT: &'static str = "CompareGT";
    pub const OTHER_VALUE_DETAIL_INDEX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for CompareGT<U>
where
    T: PartialOrd,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        self,
        validating_args: ValidatingArgs<&T, C>,
        invalid: Invalid,
        interpreting_args: InterpretingArgs<D>,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let other = self.0;

        if validating_args.target.gt(other.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.add_invalid(
                invalid.push(
                    ValidatorIdent::default().set_name(CompareGT::VALIDATOR_IDENT),
                    Details::default().set_detail(CompareGT::OTHER_VALUE_DETAIL_INDEX, &other),
                ),
                interpreting_args,
            ))
        }
    }
}

/// Validates the target is greater than or equal to field `0`.
pub struct CompareGE<T>(pub T);

impl CompareGE<()> {
    pub const VALIDATOR_IDENT: &'static str = "CompareGE";
    pub const OTHER_VALUE_DETAIL_INDEX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for CompareGE<U>
where
    T: PartialOrd,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        self,
        validating_args: ValidatingArgs<&T, C>,
        invalid: Invalid,
        interpreting_args: InterpretingArgs<D>,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let other = self.0;

        if validating_args.target.ge(other.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.add_invalid(
                invalid.push(
                    ValidatorIdent::default().set_name(CompareGE::VALIDATOR_IDENT),
                    Details::default().set_detail(CompareGE::OTHER_VALUE_DETAIL_INDEX, &other),
                ),
                interpreting_args,
            ))
        }
    }
}

/// Validates the target is equal to field `0`.
pub struct CompareEQ<T>(pub T);

impl CompareEQ<()> {
    pub const VALIDATOR_IDENT: &'static str = "CompareEQ";
    pub const OTHER_VALUE_DETAIL_INDEX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for CompareEQ<U>
where
    T: PartialOrd + PartialEq,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        self,
        validating_args: ValidatingArgs<&T, C>,
        invalid: Invalid,
        interpreting_args: InterpretingArgs<D>,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let other = self.0;

        if validating_args.target.eq(other.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.add_invalid(
                invalid.push(
                    ValidatorIdent::default().set_name(CompareEQ::VALIDATOR_IDENT),
                    Details::default().set_detail(CompareEQ::OTHER_VALUE_DETAIL_INDEX, &other),
                ),
                interpreting_args,
            ))
        }
    }
}

/// Validates the target is not equal to field `0`.
pub struct CompareNE<T>(pub T);

impl CompareNE<()> {
    pub const VALIDATOR_IDENT: &'static str = "CompareNE";
    pub const OTHER_VALUE_DETAIL_INDEX: usize = 0;
}

impl<T, C, E, U> Validator<&T, C, E> for CompareNE<U>
where
    T: PartialOrd + PartialEq,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        self,
        validating_args: ValidatingArgs<&T, C>,
        invalid: Invalid,
        interpreting_args: InterpretingArgs<D>,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let other = self.0;

        if !validating_args.target.eq(other.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.add_invalid(
                invalid.push(
                    ValidatorIdent::default().set_name(CompareNE::VALIDATOR_IDENT),
                    Details::default().set_detail(CompareNE::OTHER_VALUE_DETAIL_INDEX, &other),
                ),
                interpreting_args,
            ))
        }
    }
}

/// Validates the target is within `min` and `max`.
pub struct CompareWithin<T> {
    pub min: T,
    pub max: T,
}

impl CompareWithin<()> {
    pub const VALIDATOR_IDENT: &'static str = "CompareWithin";
    pub const MIN_VALUE_DETAIL_INDEX: usize = 0;
    pub const MAX_VALUE_DETAIL_INDEX: usize = 1;
}

impl<T, C, E, U> Validator<&T, C, E> for CompareWithin<U>
where
    T: PartialOrd,
    U: Borrow<T> + ToString,
{
    fn run<D, R>(
        self,
        validating_args: ValidatingArgs<&T, C>,
        invalid: Invalid,
        interpreting_args: InterpretingArgs<D>,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report,
    {
        let CompareWithin { min, max } = self;

        if validating_args.target.ge(min.borrow()) && validating_args.target.le(max.borrow()) {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.add_invalid(
                invalid.push(
                    ValidatorIdent::default().set_name(CompareWithin::VALIDATOR_IDENT),
                    Details::default()
                        .set_detail(CompareWithin::MIN_VALUE_DETAIL_INDEX, &min)
                        .set_detail(CompareWithin::MAX_VALUE_DETAIL_INDEX, &max),
                ),
                interpreting_args,
            ))
        }
    }
}
