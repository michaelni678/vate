//! Built-in interpretations.

use crate::{
    core::{Interpreter, Invalid},
    validators::{
        compare::{Within, EQ, GE, GT, LE, LT, NE},
        nested::Nested,
        option::{Nothing, Something},
        string::{Alphabetic, Alphanumeric, Length, Lowercase, Uppercase, ASCII},
    },
};

/// Add built-in interpretations to the given interpreter.
pub fn add_builtin_interpretations<D>(interpreter: &mut Interpreter<D>) {
    interpreter.set_normal_function_once(vec![LT::DEFAULT_VTAG], |invalid: &Invalid, _data: &D| {
        Some(format!(
            "{} must be less than {}",
            invalid.field_ident,
            invalid.detailers[0].get_detail(LT::OTHER_VALUE_DIDX),
        ))
    });

    interpreter.set_normal_function_once(vec![LE::DEFAULT_VTAG], |invalid: &Invalid, _data: &D| {
        Some(format!(
            "{} must be less than or equal to {}",
            invalid.field_ident,
            invalid.detailers[0].get_detail(LE::OTHER_VALUE_DIDX),
        ))
    });

    interpreter.set_normal_function_once(vec![GT::DEFAULT_VTAG], |invalid: &Invalid, _data: &D| {
        Some(format!(
            "{} must be greater than {}",
            invalid.field_ident,
            invalid.detailers[0].get_detail(GT::OTHER_VALUE_DIDX),
        ))
    });

    interpreter.set_normal_function_once(vec![GE::DEFAULT_VTAG], |invalid: &Invalid, _data: &D| {
        Some(format!(
            "{} must be greater than or equal to {}",
            invalid.field_ident,
            invalid.detailers[0].get_detail(GE::OTHER_VALUE_DIDX),
        ))
    });

    interpreter.set_normal_function_once(vec![EQ::DEFAULT_VTAG], |invalid: &Invalid, _data: &D| {
        Some(format!(
            "{} must be equal to {}",
            invalid.field_ident,
            invalid.detailers[0].get_detail(EQ::OTHER_VALUE_DIDX),
        ))
    });

    interpreter.set_normal_function_once(vec![NE::DEFAULT_VTAG], |invalid: &Invalid, _data: &D| {
        Some(format!(
            "{} must not be equal to {}",
            invalid.field_ident,
            invalid.detailers[0].get_detail(NE::OTHER_VALUE_DIDX),
        ))
    });

    interpreter.set_normal_function_once(
        vec![Within::RANGE_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must be greater than or equal to {} and less than {}",
                invalid.field_ident,
                invalid.detailers[0].get_detail(Within::RANGE_START_VALUE_DIDX),
                invalid.detailers[0].get_detail(Within::RANGE_END_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Within::RANGE_FROM_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must be greater than or equal to {}",
                invalid.field_ident,
                invalid.detailers[0].get_detail(Within::RANGE_FROM_START_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Within::RANGE_INCLUSIVE_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must be greater than or equal to {} and less than or equal to {}",
                invalid.field_ident,
                invalid.detailers[0].get_detail(Within::RANGE_INCLUSIVE_START_VALUE_DIDX),
                invalid.detailers[0].get_detail(Within::RANGE_INCLUSIVE_END_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Within::RANGE_TO_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must be less than {}",
                invalid.field_ident,
                invalid.detailers[0].get_detail(Within::RANGE_TO_END_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Within::RANGE_TO_INCLUSIVE_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must be less than or equal to {}",
                invalid.field_ident,
                invalid.detailers[0].get_detail(Within::RANGE_TO_INCLUSIVE_END_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Alphabetic::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} cannot contain non-alphabetic characters",
                invalid.field_ident,
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Alphanumeric::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} cannot contain non-alphanumeric characters",
                invalid.field_ident,
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![ASCII::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} cannot contain non-ASCII characters",
                invalid.field_ident,
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Lowercase::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} cannot contain uppercase characters",
                invalid.field_ident,
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Uppercase::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} cannot contain lowercase characters",
                invalid.field_ident,
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::BYTES_VTAG, LT::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length less than {} bytes",
                invalid.field_ident,
                invalid.detailers[1].get_detail(LT::OTHER_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::BYTES_VTAG, LE::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length less than or equal to {} bytes",
                invalid.field_ident,
                invalid.detailers[1].get_detail(LE::OTHER_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::BYTES_VTAG, GT::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length greater than {} bytes",
                invalid.field_ident,
                invalid.detailers[1].get_detail(GT::OTHER_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::BYTES_VTAG, GE::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length greater than or equal to {} bytes",
                invalid.field_ident,
                invalid.detailers[1].get_detail(GE::OTHER_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::BYTES_VTAG, EQ::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length equal to {} bytes",
                invalid.field_ident,
                invalid.detailers[1].get_detail(EQ::OTHER_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::BYTES_VTAG, NE::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length not equal to {} bytes",
                invalid.field_ident,
                invalid.detailers[1].get_detail(NE::OTHER_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::BYTES_VTAG, Within::RANGE_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length greater than or equal to {} and less than {} bytes",
                invalid.field_ident,
                invalid.detailers[1].get_detail(Within::RANGE_START_VALUE_DIDX),
                invalid.detailers[1].get_detail(Within::RANGE_END_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::BYTES_VTAG, Within::RANGE_FROM_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length greater than or equal to {} bytes",
                invalid.field_ident,
                invalid.detailers[1].get_detail(Within::RANGE_FROM_START_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::BYTES_VTAG, Within::RANGE_INCLUSIVE_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
            "{} must have a length greater than or equal to {} and less than or equal to {} bytes",
            invalid.field_ident,
            invalid.detailers[1].get_detail(Within::RANGE_INCLUSIVE_START_VALUE_DIDX),
            invalid.detailers[1].get_detail(Within::RANGE_INCLUSIVE_END_VALUE_DIDX),
        ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::BYTES_VTAG, Within::RANGE_TO_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length less than {} bytes",
                invalid.field_ident,
                invalid.detailers[1].get_detail(Within::RANGE_TO_END_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::BYTES_VTAG, Within::RANGE_TO_INCLUSIVE_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length less than or equal to {} bytes",
                invalid.field_ident,
                invalid.detailers[1].get_detail(Within::RANGE_TO_INCLUSIVE_END_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::CHARS_VTAG, LT::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length less than {} characters",
                invalid.field_ident,
                invalid.detailers[1].get_detail(LT::OTHER_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::CHARS_VTAG, LE::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length less than or equal to {} characters",
                invalid.field_ident,
                invalid.detailers[1].get_detail(LE::OTHER_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::CHARS_VTAG, GT::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length greater than {} characters",
                invalid.field_ident,
                invalid.detailers[1].get_detail(GT::OTHER_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::CHARS_VTAG, GE::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length greater than or equal to {} characters",
                invalid.field_ident,
                invalid.detailers[1].get_detail(GE::OTHER_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::CHARS_VTAG, EQ::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length equal to {} characters",
                invalid.field_ident,
                invalid.detailers[1].get_detail(EQ::OTHER_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::CHARS_VTAG, NE::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length not equal to {} characters",
                invalid.field_ident,
                invalid.detailers[1].get_detail(NE::OTHER_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::CHARS_VTAG, Within::RANGE_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length greater than or equal to {} and less than {} characters",
                invalid.field_ident,
                invalid.detailers[1].get_detail(Within::RANGE_START_VALUE_DIDX),
                invalid.detailers[1].get_detail(Within::RANGE_END_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::CHARS_VTAG, Within::RANGE_FROM_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length greater than or equal to {} characters",
                invalid.field_ident,
                invalid.detailers[1].get_detail(Within::RANGE_FROM_START_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(vec![Length::CHARS_VTAG, Within::RANGE_INCLUSIVE_VTAG], |invalid: &Invalid, _data: &D|
        Some(format!(
            "{} must have a length greater than or equal to {} and less than or equal to {} characters",
            invalid.field_ident,
            invalid.detailers[1].get_detail(Within::RANGE_INCLUSIVE_START_VALUE_DIDX),
            invalid.detailers[1].get_detail(Within::RANGE_INCLUSIVE_END_VALUE_DIDX),
        ))
    );

    interpreter.set_normal_function_once(
        vec![Length::CHARS_VTAG, Within::RANGE_TO_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length less than {} characters",
                invalid.field_ident,
                invalid.detailers[1].get_detail(Within::RANGE_TO_END_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Length::CHARS_VTAG, Within::RANGE_TO_INCLUSIVE_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!(
                "{} must have a length less than or equal to {} characters",
                invalid.field_ident,
                invalid.detailers[1].get_detail(Within::RANGE_TO_INCLUSIVE_END_VALUE_DIDX),
            ))
        },
    );

    interpreter.set_normal_function_once(
        vec![Nested::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!("{} must not contain invalids", invalid.field_ident))
        },
    );

    interpreter.set_normal_function_once(
        vec![Something::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| Some(format!("{} must be defined", invalid.field_ident)),
    );

    interpreter.set_normal_function_once(
        vec![Nothing::DEFAULT_VTAG],
        |invalid: &Invalid, _data: &D| {
            Some(format!("{} must not be defined", invalid.field_ident,))
        },
    );
}
