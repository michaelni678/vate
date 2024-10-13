<h1 align=center><code>vate</code></h1>
<h3 align=center>Rust data validation crate</h3>

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
- Documentation comments.
- Tests for everything.
- Email validator.
- Phone number validator.
- URL validator.
- Credit card validator.
- Enum validation.
