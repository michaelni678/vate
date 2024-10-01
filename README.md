<h1 align=center><code>vate</code></h1>
<h3 align=center>Rust data validation library</h3>

## Installation
Currently not on crates.io. Specify the dependency using this git repository instead.
```
vate = { git = "https://github.com/michaelni678/vate" }
```

## Usage
```rust
#[derive(Validate)]
struct CreateUser {
    #[vate(Alphanumeric, LengthRange::Chars { min: 4, max: 20 })]
    username: String,
    #[vate(Ascii, LengthRange::Chars { min: 8, max: usize::MAX })]
    password: String,
    #[vate(EqualTo(Cow::Borrowed(&self.password)))]
    confirm_password: String,
}

let data = ();
let mut report = Report::new(Accessor::Root("create_user"));
let _ = create_user.validate::<InvalidsAndErrors>(&data, &mut report);
```

## Built-in Validators

### Bundle
`Bundle!` is a macro that allows multiple validators at the same level. The two examples below are technically equivalent, however the first would require unwrapping the option for both `Alphabetic` and `Ascii` validations, whereas the second example would only require a single unwrap.
```rust
#[vate(NotMissingThen(Alphabetic), NotMissingThen(Ascii))]
a: Option<String>,
#[vate(NotMissingThen(Bundle!(Alphabetic, Ascii)))]
b: Option<String>,
```

### Compare
`LessThan`, `LessThanOrEqualTo`, `GreaterThan`, `GreaterThanOrEqualTo`, `EqualTo`, and `NotEqualTo` are all validators for comparing one value with another. 
```rust
#[vate(LessThan(Cow::Owned(5)))]
a: u32,
#[vate(EqualTo(Cow::Borrowed(&self.a)))]
b: u32,
```

### Iterate
`Iterate` converts a collection to an iterator, running the inner validator.
`Indexed` and `Keyed` will iterate over an iterator, passing iterated items to the inner validator. `Indexed` will keep track of the indices of items, generating `Accessor::Index`. `Keyed` expects a key / value tuple pair, where the key generates `Accessor::Key`, and the value is passed to the inner validator.
```rust
#[vate(Iterate(Indexed(Alphabetic)))]
a: Vec<String>,
#[vate(Iterate(Keyed(Alphabetic)))]
b: HashMap<String, String>,
```

### Nested
`Nested` simply validates a nested struct.
```rust
#[derive(Validate)]
struct A {
    #[vate(Nested)]
    b: B,
}

#[derive(Validate)]
struct B { ... }
```

### Option
`NotMissing` and `Missing` validate if the option variant is the `Some` or `None` variant.
```rust
#[vate(NotMissing)]
a: Option<u32>,
#[vate(Missing)]
b: Option<String>,
```
`NotMissingThen` will run the inner validator with the unwrapped value if it exists. Otherwise, nothing is validated.
```rust
#[vate(NotMissingThen(Alphabetic))]
a: Option<String>,
```

### String
`Alphabetic`, `Alphanumeric`, and `Ascii` check if all characters in a string are alphabetic, alphanumeric, or ascii.
```rust
#[vate(Alphabetic)]
a: String,
#[vate(Alphanumeric)]
b: String,
#[vate(Ascii)]
c: String,
```
At the moment, `vate` supports the string units:
- Bytes
- Chars

`LengthEquals` checks if the length of a string is equal to the specified size. 
```rust
#[vate(LengthEquals::Bytes(4))]
a: String,
#[vate(LengthEquals::Chars(8))]
b: String,
```
`LengthRange` checks if the length of a string is between `min` and `max` units.
```rust
#[vate(LengthRange::Bytes { min: 4, max: 7 })]
a: String,
#[vate(LengthRange::Chars { min: 2, max: usize::MAX })]
b: String,
```