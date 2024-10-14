#![allow(dead_code)]

use std::collections::HashMap;

use once_cell::sync::Lazy;
use vate::{
    extras::Regex, path, Accessor, Bundle, CollectionIterate, Compare, InvalidsAndErrors,
    IteratorIndexed, IteratorKeyed, Nested, OptionSomeThen, Report, PasswordStrong, StringAlphabetic,
    StringAlphanumeric, StringAscii, StringLengthRange, StringMatchesRegex, Validate,
};

/// The required age to create an account.
const REQUIRED_AGE: u8 = 18;

/// The regex for validating company names.
static COMPANY_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"^[A-Z]([a-zA-Z0-9]|[- @\.#&!])*$"#).unwrap());

/// A request to create a user.
#[derive(Validate)]
struct CreateUser {
    /// The profile of the user to create.
    #[vate(Nested)]
    profile: Profile,
    /// The login credentials of the user to create.
    #[vate(Nested)]
    credentials: Credentials,
}

/// User profile.
#[derive(Validate)]
struct Profile {
    /// The user's name.
    #[vate(Nested)]
    name: Name,
    /// The user's age. Must be over `REQUIRED_AGE` to create the account.
    #[vate(Compare!( >= REQUIRED_AGE ))]
    age: u8,
    /// The user's company, which must match `COMPANY_REGEX` if `Some`.
    #[vate(OptionSomeThen(StringMatchesRegex(&COMPANY_REGEX)))]
    company: Option<String>,
    /// The user's hobbies. All hobby names must be ascii.
    #[vate(CollectionIterate(IteratorIndexed(StringAscii)))]
    hobbies: Vec<String>,
    /// The user's languages mapped to fluency. Fluency should be between 1 and 10.
    #[vate(CollectionIterate(IteratorKeyed(Bundle!(Compare!( >= 1 ), Compare!( <= 10 )))))]
    languages: HashMap<String, u8>,
}

/// A name.
#[derive(Validate)]
struct Name {
    /// The user's first name, which must be alphabetic and between 2 and 32 characters.
    #[vate(StringAlphabetic, StringLengthRange::Chars { min: 2, max: 32 })]
    first: String,
    /// The user's middle name. This is optional, but if provided it must be alphabetic and between 2 and 32 characters.
    #[vate(OptionSomeThen(Bundle!(StringAlphabetic, StringLengthRange::Chars { min: 2, max: 32 })))]
    middle: Option<String>,
    /// The user's last name, which must be alphabetic and between 2 and 32 characters.
    #[vate(StringAlphabetic, StringLengthRange::Chars { min: 2, max: 32 })]
    last: String,
}

/// Credentials.
#[derive(Validate)]
struct Credentials {
    /// The user's username, which must be alphanumeric and between 4 and 20 characters.
    #[vate(StringAlphanumeric, StringLengthRange::Chars { min: 4, max: 20 })]
    username: String,
    /// The user's password, which must be ascii, at least 8 characters long, and strong.
    /// The username is passed into the password strength validator to ensure the password is not 
    /// similar to the username.
    #[vate(StringAscii, StringLengthRange::Chars { min: 8, max: usize::MAX }, PasswordStrong([&self.username]))]
    password: String,
    /// The password confirmation, which must be equal to the password.
    #[vate(Compare!( == &self.password ))]
    confirm_password: String,
    /// The ID of the device that attempted to create this user. This is purposely not
    /// validated to demonstrate that not all fields need to be validated.
    device_id: String,
}

fn main() {
    // Create an instance of `CreateUser`.
    let create_user = CreateUser {
        profile: Profile {
            name: Name {
                first: String::from("Gintoki"),
                middle: Some(String::from("0")), // Not alphabetic nor between 2 and 32 characters.
                last: String::from("Sakata"),
            },
            age: 29,
            company: Some(String::from("Yorozuya")),
            hobbies: vec![
                String::from("Eating sweets"),
                String::from("\u{03A9}"), // Not ascii.
                String::from("Reading manga"),
            ],
            languages: HashMap::from([
                (String::from("Japanese"), 10),
                (String::from("English"), 0), // Not greater than or equal to 1.
            ]),
        },
        credentials: Credentials {
            username: String::from("u$ername"), // Not alphanumeric.
            password: String::from("health me"),
            confirm_password: String::from("pulp fiction"), // Not equal to `password`.
            device_id: String::from("ZuR4-j4N41_kaT5UrA-d4"),
        },
    };

    // Create the root report.
    let mut report = Report::new(Accessor::Root("create_user"));

    // Validate `create_user`, ignoring the return value since the validations
    // done by this example do not produce any errors.
    let _ = create_user.validate::<InvalidsAndErrors>(&(), &mut report);

    // The report should contain 7 leaves. All leaves should be invalid reports, since the
    // collector `InvalidsAndErrors` only collects invalid and erroneous reports, and
    // the validations done by this example do not produce any errors.
    assert_eq!(report.count_leaves(), 7);

    // The report should be invalid at create_user.profile.name.middle,
    // since "0" is not alphabetic nor between 2 and 32 characters.
    assert!(report
        .is_any_invalid_at_path(path!(create_user.profile.name.middle))
        .unwrap());

    // The report should have two leaves at create_user.profile.name.middle,
    // one for the alphabetic validation and one for the string length validation.
    assert_eq!(
        report.count_leaves_at_path(path!(create_user.profile.name.middle)),
        2
    );

    // The report should be invalid at create_user.profile.hobbies[1],
    // since "\u{03A9}" is not ascii.
    assert!(report
        .is_any_invalid_at_path(path!(create_user.profile.hobbies[1]))
        .unwrap());

    // The report should be invalid at create_user.profile.languages["English"],
    // since 0 is not greater than or equal to 1.
    assert!(report
        .is_any_invalid_at_path(path!(create_user.profile.languages["English"]))
        .unwrap());

    // The report should be invalid at create_user.credentials.username,
    // since "u$ername" is not alphanumeric.
    assert!(report
        .is_any_invalid_at_path(path!(create_user.credentials.username))
        .unwrap());

    // The report should be invalid at create_user.credentials.password,
    // since "health me" is not a strong password.
    assert!(report
        .is_any_invalid_at_path(path!(create_user.credentials.password))
        .unwrap());

    // The report should be invalid at create_user.credentials.confirm_password,
    // since "pulp fiction" is not equal to "health me".
    assert!(report
        .is_any_invalid_at_path(path!(create_user.credentials.confirm_password))
        .unwrap());

    // Print the report.
    println!("{report}");
}
