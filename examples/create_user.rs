#![allow(dead_code)]

use std::collections::HashMap;

use vate::{
    path, Accessor, Alphabetic, Alphanumeric, Ascii, Bundle, EqualTo, GreaterThanOrEqualTo,
    Indexed, InvalidsAndErrors, Iterate, Keyed, LengthRange, LessThanOrEqualTo, Nested,
    NotMissingThen, Report, Validate,
};

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
    /// The user's age. Must be over 18 to create the account.
    #[vate(GreaterThanOrEqualTo(18))]
    age: u8,
    /// The user's company. This is not validated.
    company: Option<String>,
    /// The user's hobbies. All hobby names must be ascii.
    #[vate(Iterate(Indexed(Ascii)))]
    hobbies: Vec<String>,
    /// The user's languages mapped to fluency. Fluency should be between 1 and 10.
    #[vate(Iterate(Keyed(Bundle!(GreaterThanOrEqualTo(1), LessThanOrEqualTo(10)))))]
    languages: HashMap<String, u8>,
}

/// A name.
#[derive(Validate)]
struct Name {
    /// The user's first name, which must be alphabetic and between 2 and 32 characters.
    #[vate(Alphabetic, LengthRange::Chars { min: 2, max: 32 })]
    first: String,
    /// The user's middle name. This is optional, but if provided it must be alphabetic and between 2 and 32 characters.
    #[vate(NotMissingThen(Bundle!(Alphabetic, LengthRange::Chars { min: 2, max: 32 })))]
    middle: Option<String>,
    /// The user's last name, which must be alphabetic and between 2 and 32 characters.
    #[vate(Alphabetic, LengthRange::Chars { min: 2, max: 32 })]
    last: String,
}

/// Credentials.
#[derive(Validate)]
struct Credentials {
    /// The user's username, which must be alphanumeric and between 4 and 20 characters.
    #[vate(Alphanumeric, LengthRange::Chars { min: 4, max: 20 })]
    username: String,
    /// The user's password, which must be ascii and at least 8 characters long.
    #[vate(Ascii, LengthRange::Chars { min: 8, max: usize::MAX })]
    password: String,
    /// The password confirmation, which must be equal to the password.
    #[vate(EqualTo(&self.password))]
    confirm_password: String,
}

fn main() {
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
        },
    };

    let mut report = Report::new(Accessor::Root("create_user"));
    let _ = create_user.validate::<InvalidsAndErrors>(&(), &mut report);

    report.is_invalid_at_path(path!(create_user.profile.name.middle));
    report.is_invalid_at_path(path!(create_user.profile.hobbies[1]));
    report.is_invalid_at_path(path!(create_user.profile.languages["English"]));
    report.is_invalid_at_path(path!(create_user.credentials.username));
    report.is_invalid_at_path(path!(create_user.credentials.confirm_password));

    println!("{report:#?}");
}
