#![allow(dead_code)]
use vate::{Accessor, Alphabetic, Alphanumeric, Ascii, Bundle, EqualTo, GreaterThanOrEqualTo, InvalidsAndErrors, LengthRange, Nested, NotMissingThen, Report, Validate};

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
}

/// A name.
#[derive(Validate)]
struct Name {
    /// The user's first name, which must be alphabetic and between 2 and 32 chars.
    #[vate(Alphabetic, LengthRange::Chars { min: 2, max: 32 })]
    first: String,
    /// The user's middle name. This is optional, but if provided it must be alphabetic and between 2 and 32 chars.
    #[vate(NotMissingThen(Bundle!(Alphabetic, LengthRange::Chars { min: 2, max: 32 })))]
    middle: Option<String>,
    /// The user's last name, which must be alphabetic and between 2 and 32 chars.
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
    // CreateUser request.
    let create_user = CreateUser {
        profile: Profile {
            name: Name {
                first: String::from("Gintoki"),
                middle: Some(String::from("0")), // Not alphabetic nor between 2 and 32 characters.
                last: String::from("Sakata"),
            },
            age: 29,
            company: Some(String::from("Yorozuya")),
        },
        credentials: Credentials {
            username: String::from("u$ername"), // Not alphanumeric.
            password: String::from("health me"),
            confirm_password: String::from("pulp fiction"), // Not equal to `password`.
        },
    };

    // Dummy data.
    let data = ();

    // Validate `create_user`, collecting the invalids and errors.
    let mut report = Report::new(Accessor::Root);
    let _ = create_user.validate::<InvalidsAndErrors>(&data, &mut report);

    println!("{report:#?}");
}