<h1 align=center><code>Vate</code></h1>
<h3 align=center>Rust data validation framework</h3>

## Overview
- **Struct and Enum Validation:** Automatically derive validation logic for structs and enums using the `Validate` procedural macro.
- **Built-in Validators:** Utilize the built-in validators for common use cases.
- **Custom Validators:** Define your own validators tailored to your specific needs.
- **Custom Errors:** Declare your own validation error messages.

## Installation
Currently not on crates.io. Specify the dependency using this git repository instead.
```
vate = { git = "https://github.com/michaelni678/vate" }
```

## Usage
```rust
/// A registration request.
#[derive(Validate)]
struct Register {
    /// The user's username.
    ///
    /// Must be alphanumeric and between 4 and 20 characters long.
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

// Create a report and validate the instance of `Register`.
let mut report = BasicReport::default();
let _ = create_user
    .validate(context, interpreter, data, &mut report)
    .unwrap();
```