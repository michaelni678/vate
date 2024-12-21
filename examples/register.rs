//! Registration example.

use once_cell::sync::Lazy;
use vate::{
    core::{Interpreter, Validate},
    extras::Regex,
    reports::ComprehensiveReport,
    validators::{
        bundle::Bundle,
        compare::{Within, EQ, GE},
        nested::Nested,
        option::SomethingThen,
        string::{Alphabetic, Alphanumeric, Length, MatchesRegex, ASCII},
    },
};

/// The required age to create an account.
const REQUIRED_AGE: u8 = 18;

/// The regex for validating company names.
static COMPANY_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[A-Z]([a-zA-Z0-9]|[- @\.#&!])*$").unwrap());

/// A request for registration.
#[derive(Validate)]
struct Register {
    /// The profile of the user.
    #[vate(Nested)]
    profile: Profile,

    /// The login credentials of the user.
    #[vate(Nested)]
    credentials: Credentials,
}

/// User profile.
#[derive(Validate)]
struct Profile {
    /// The user's name.
    #[vate(Nested)]
    name: Name,

    /// The user's age.
    ///
    /// Age must be over [`REQUIRED_AGE`] to create the account.
    #[vate(GE(REQUIRED_AGE))]
    age: u8,

    /// The user's company.
    ///
    /// The company must match [`COMPANY_REGEX`] if [`Some`].
    #[vate(SomethingThen(MatchesRegex(&COMPANY_REGEX)))]
    company: Option<String>,
}

/// A name.
#[derive(Validate)]
struct Name {
    /// The user's first name.
    ///
    /// Must be alphabetic and between 2 and 32 characters.
    #[vate(Alphabetic, Length::Chars(Within(2..=32)))]
    first: String,

    /// The user's middle name.
    ///
    /// This is optional, but if provided it must be alphabetic and between 2 and 32 characters.
    #[vate(SomethingThen(Bundle!(Alphabetic, Length::Chars(Within(2..=32)))))]
    middle: Option<String>,

    /// The user's last name.
    ///
    /// Must be alphabetic and between 2 and 32 characters.
    #[vate(Alphabetic, Length::Chars(Within(2..=32)))]
    last: String,
}

/// Credentials.
#[derive(Validate)]
struct Credentials {
    /// The user's username
    ///
    /// Usernames must be alphanumeric and between 4 and 20 characters.
    /// The username must also be available.
    #[vate(Alphanumeric, Length::Chars(Within(4..=20)))]
    username: String,

    /// The user's password.
    ///
    /// Passwords must be ASCII and greater than 8 characters long.
    #[vate(ASCII, Length::Chars(GE(8)))]
    password: String,

    /// The password confirmation.
    ///
    /// This must be equivalent to `password`.
    #[vate(EQ(password))]
    confirm_password: String,
}

fn main() {
    // Create an instance of `Register` to be validated.
    let register = Register {
        profile: Profile {
            name: Name {
                first: String::from("John"),
                middle: Some(String::from("0")), // Not alphabetic nor between 2 and 32 characters.
                last: String::from("Smith"),
            },
            age: 29,
            company: Some(String::from("Vate")),
        },
        credentials: Credentials {
            username: String::from("u$ername"), // Not alphanumeric.
            password: String::from("password123"),
            confirm_password: String::from("password456"), // Not equal to `password`.
        },
    };

    let mut interpreter = Interpreter::default();

    // Create a report with a high limit.
    let mut report = ComprehensiveReport::default();
    report.limit = usize::MAX;

    // Validate the request.
    // None of the validators used in this example returns an error, so
    // it is fine to unwrap the result.
    let _ = register
        .validate(&(), &interpreter, &(), &mut report)
        .unwrap();

    // Serialize to a json and print it.
    let json = serde_json::to_string_pretty(&report).unwrap();
    println!("{json}");
}
