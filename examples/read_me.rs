use vate::{
    core::{Interpreter, Validate},
    interpretations::add_builtin_interpretations,
    reports::BasicReport,
    validators::{
        compare::{Within, EQ, GE},
        string::{Alphanumeric, Length, ASCII},
    },
};

/// A registration request.
#[derive(Validate)]
struct Register {
    /// The user's username.
    ///
    /// Must be alphanumeric and between 4 and 20 characters.
    #[vate(Alphanumeric, Length::Chars(Within(4..=20)))]
    username: String,

    /// The user's password.
    ///
    /// Must be ASCII and greater than 8 characters long.
    #[vate(ASCII, Length::Chars(GE(8)))]
    password: String,

    /// The password confirmation.
    ///
    /// Must be equal to `password`.
    #[vate(EQ(password))]
    confirm_password: String,
}

fn main() {
    // Create an instance of `Register` to be validated.
    let create_user = Register {
        username: String::from("u$ername"), // Not alphanumeric.
        password: String::from("password123"),
        confirm_password: String::from("password456"), // Not equal to `password`.
    };

    // Create an interpreter and add the built-in interpretations.
    let mut interpreter = Interpreter::default();
    add_builtin_interpretations(&mut interpreter);

    // Create a report and validate the instance of `Register`.
    let mut report = BasicReport::default();
    report.limit = usize::MAX;
    let _ = create_user
        .validate(&(), &interpreter, &(), &mut report)
        .unwrap();

    // Serialize to a JSON and print it.
    let json = serde_json::to_string_pretty(&report).unwrap();
    println!("{json}");
}
