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

## Built-in Validators

### Boolean
`True` and `False` validate that a boolean is `true` or `false`.
```rust
#[vate(True)]
a: bool,
#[vate(False)]
b: bool,
```

### Bundle
`Bundle!` is a macro that allows multiple validators at the same level. The two examples below are technically equivalent, however the first would require unwrapping the option for both `StringAlphabetic` and `StringAscii` validations, whereas the second example would only require a single unwrap.
```rust
#[vate(OptionSomeThen(StringAlphabetic), OptionSomeThen(StringAscii))]
a: Option<String>,
#[vate(OptionSomeThen(Bundle!(StringAlphabetic, StringAscii)))]
b: Option<String>,
```

### Collection
`CollectionIterate` iterates a collection, running its inner validator with the iterator.
```rust
#[vate(CollectionIterate(IteratorIndexed(Alphabetic)))]
a: Vec<String>,
```

### Compare
`Compare!` is a macro that simplifies generating the other validators listed below.
```rust
#[vate(Compare!( < 5 ))]
a: u32,
#[vate(Compare! ( == &self.a ))]
b: u32,
```

`CompareLessThan`, `CompareLessThanOrEqualTo`, `CompareGreaterThan`, `CompareGreaterThanOrEqualTo`, `CompareEqualTo`, and `CompareNotEqualTo` are all validators for comparing one value with another. 
```rust
#[vate(CompareLessThan(Cow::Owned(5)))]
a: u32,
#[vate(CompareEqualTo(Cow::Borrowed(&self.a)))]
b: u32,
```

### Iterator
`IteratorIndexed` and `IteratorKeyed` will iterate over an iterator, passing iterated items to the inner validator. `IteratorIndexed` will keep track of the indices of items, generating `Accessor::Index`. `IteratorKeyed` expects a key / value tuple pair, where the key generates `Accessor::Key`, and the value is passed to the inner validator.
```rust
#[vate(CollectionIterate(IteratorIndexed(Alphabetic)))]
a: Vec<String>,
#[vate(CollectionIterate(IteratorKeyed(Alphabetic)))]
b: HashMap<String, String>,
```

`IteratorLengthEquals` counts the number of items in an iterator. When an iterator implements `ExactSizeIterator`, prefer the `ExactSizeIteratorLengthEquals` validator, which also returns the length of the iterator.
```rust
#[vate(CollectionIterate(IteratorLengthEquals(5)))]
a: Vec<String>,
#[vate(CollectionIterate(ExactSizeIteratorLengthEquals(5)))]
b: HashMap<String, u32>,
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
`OptionSome` and `OptionNone` validate if the option variant is the `Some` or `None` variant.
```rust
#[vate(OptionSome)]
a: Option<u32>,
#[vate(OptionNone)]
b: Option<String>,
```

`OptionSomeThen` will run the inner validator with the unwrapped value if it exists. Otherwise, nothing is validated.
```rust
#[vate(OptionSomeThen(StringAlphabetic))]
a: Option<String>,
```

### String
`StringAlphabetic`, `StringAlphanumeric`, and `StringAscii` check if all characters in a string are alphabetic, alphanumeric, or ascii.
```rust
#[vate(StringAlphabetic)]
a: String,
#[vate(StringAlphanumeric)]
b: String,
#[vate(StringAscii)]
c: String,
```
At the moment, `vate` supports the string units:
- Bytes
- Chars

`StringLengthEquals` checks if the length of a string is equal to the specified size. 
```rust
#[vate(StringLengthEquals::Bytes(4))]
a: String,
#[vate(StringLengthEquals::Chars(8))]
b: String,
```

`StringLengthRange` checks if the length of a string is between `min` and `max` units.
```rust
#[vate(StringLengthRange::Bytes { min: 4, max: 7 })]
a: String,
#[vate(StringLengthRange::Chars { min: 2, max: usize::MAX })]
b: String,
```

`StringMatchesRegex` checks if a string matches the specified regex.
```rust
use std::once_cell::sync::Lazy;

use vate::extras::Regex;

static DNA_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^[ACGT]+$"));

#[vate(RegexMatchesString(&DNA_REGEX))]
a: String,
```
