//! IP validators.

use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use crate::core::*;

/// Validates the target is a valid IP address.
pub struct IP;

impl IP {
    pub const DEFAULT_VTAG: ValidationTag = "m=ip;v=IP";
    pub const TARGET_VALUE_DIDX: usize = 0;
}

impl<T, C, E> Validator<T, C, E> for IP
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

        if IpAddr::from_str(target).is_ok() {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    IP::DEFAULT_VTAG,
                    Detailer::default().set_detail(IP::TARGET_VALUE_DIDX, &target),
                ),
                interpreter,
                data,
            ))
        }
    }
}

/// Validates the target is a valid IPv4 address.
pub struct IPv4;

impl IPv4 {
    pub const DEFAULT_VTAG: ValidationTag = "m=ip;v=IPv4";
    pub const TARGET_VALUE_DIDX: usize = 0;
}

impl<T, C, E> Validator<T, C, E> for IPv4
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

        if Ipv4Addr::from_str(target).is_ok() {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    IPv4::DEFAULT_VTAG,
                    Detailer::default().set_detail(IPv4::TARGET_VALUE_DIDX, &target),
                ),
                interpreter,
                data,
            ))
        }
    }
}

/// Validates the target is a valid IPv6 address.
pub struct IPv6;

impl IPv6 {
    pub const DEFAULT_VTAG: ValidationTag = "m=ip;v=IPv6";
    pub const TARGET_VALUE_DIDX: usize = 0;
}

impl<T, C, E> Validator<T, C, E> for IPv6
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

        if Ipv6Addr::from_str(target).is_ok() {
            Ok(ControlFlow::Continue)
        } else {
            Ok(report.push_invalid(
                invalid.push_validation(
                    IPv6::DEFAULT_VTAG,
                    Detailer::default().set_detail(IPv6::TARGET_VALUE_DIDX, &target),
                ),
                interpreter,
                data,
            ))
        }
    }
}
