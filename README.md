<h1 align=center><code>vate</code></h1>
<h3 align=center>Rust data validation crate</h3>

## Overview
- **Struct and Enum Validation:** Automatically derive validation logic for structs and enums using the `Validate` procedural macro.
- **Built-in Validators:** Utilize the built-in validators for common use cases.
- **Custom Validators:** Define your own validators tailored to your specific needs, whether for validating custom data types, reading custom contexts, or applying custom error handling.

## Installation
Currently not on crates.io. Specify the dependency using this git repository instead.
```
vate = { git = "https://github.com/michaelni678/vate" }
```

## Usage
```rust
#[derive(Validate)]
struct CreateUser {
    #[vate(StringAlphanumeric, StringLengthRange::Chars { min: 4, max: 20 })]
    username: String,
    #[vate(StringAscii, StringLengthRange::Chars { min: 8, max: usize::MAX })]
    password: String,
    #[vate(Compare!( == &self.password ))]
    confirm_password: String,
}

let data = ();
let mut report = Report::new(Accessor::Root("create_user"));
let _ = create_user.validate::<InvalidsAndErrors>(&data, &mut report);
```

## To-Do
*Feel free to contribute or suggest more features by creating an issue :)*
- Documentation and tests.
- Email validator.
- Phone number validator.
- URL validator.
- Credit card validator.
- Password strength validator.
- Compare range validator with `Compare` macro support.
