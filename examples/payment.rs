#![allow(dead_code)]

use vate::{
    Accessor, Collector, Compare, Everything, Exit, Nested, Report, StringAsciiDigit,
    StringLengthRange, Validate, Validator,
};

#[derive(Validate)]
#[vate(data = PaymentManager)]
struct Payment {
    #[vate(Nested)]
    credit_card: CreditCard,
}

#[derive(Validate)]
#[vate(data = PaymentManager)]
struct CreditCard {
    /// The credit card number.
    number: String,
    /// The cardholder name.
    /// This is not validated.
    name: String,
    /// The credit card expiration date.
    /// Must be before or equal to `latest_mmdd` in the payment manager.
    #[vate(Nested, NotExpired)]
    expiry: MMDD,
    /// The credit card security code. Must be only numerical and between 3 and 4 characters.
    #[vate(StringAsciiDigit, StringLengthRange::Chars { min: 3, max: 4 })]
    csc: String,
}

#[derive(Validate)]
#[vate(data = PaymentManager)]
struct MMDD {
    #[vate(Compare!( <= 12 ))]
    mm: u8,
    #[vate(Compare!( <= 99 ))]
    yy: u8,
}

/// Custom validator to check if an MMDD is after another MMDD.
struct NotExpired;

impl<E> Validator<MMDD, PaymentManager, E> for NotExpired {
    fn run<C: Collector<E>>(
        &self,
        accessor: Accessor,
        date: &MMDD,
        pm: &PaymentManager,
        parent_report: &mut Report<E>,
    ) -> Result<(), Exit<E>> {
        let mut child_report = Report::new(accessor);

        if pm.latest_mmdd.yy < date.yy
            || (pm.latest_mmdd.yy == date.yy && pm.latest_mmdd.mm <= date.mm)
        {
            child_report.set_valid();
        } else {
            child_report.set_invalid();
            child_report.set_message("is expired");
        }

        C::apply(parent_report, child_report)
    }
}

/// The data passed into the validators.
struct PaymentManager {
    /// The latest valid MMDD date, which is updated every new month.
    latest_mmdd: MMDD,
}

fn main() {
    let pm = PaymentManager {
        latest_mmdd: MMDD {
            // Mock todays date as October '24 (10/24).
            mm: 10,
            yy: 24,
        },
    };

    let payment = Payment {
        credit_card: CreditCard {
            number: String::from("1234 1234 1234 1234"),
            name: String::from("Brooke Azelle"),
            expiry: MMDD {
                mm: 11,
                yy: 23, // Card is expired.
            },
            csc: String::from("888"),
        },
    };

    let mut report = Report::new(Accessor::Root("payment"));

    let _ = payment.validate::<Everything>(&pm, &mut report);

    println!("{report}");
}
