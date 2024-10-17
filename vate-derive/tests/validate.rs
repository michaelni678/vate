#![allow(dead_code)]

use vate::{path, Accessor, Everything, Report, StringAlphabetic, Validate};

#[test]
fn unit_struct() {
    #[derive(Validate)]
    struct Example;
}

#[test]
fn regular_struct_empty() {
    #[derive(Validate)]
    struct Example {}
}

#[test]
fn regular_struct_no_validations() {
    #[derive(Validate)]
    struct Example {
        a: String,
    }
}

#[test]
fn regular_struct() {
    #[derive(Validate)]
    struct Example {
        #[vate(StringAlphabetic)]
        a: String,
    }

    let example = Example {
        a: String::from("!!!"),
    };

    let mut report = Report::new(Accessor::Root("example"));
    let _ = example.validate::<Everything>(&(), &mut report);

    assert!(report.is_any_invalid_at_path(path!(example.a)).unwrap());
}

#[test]
fn tuple_struct_empty() {
    #[derive(Validate)]
    struct Example();
}

#[test]
fn tuple_struct_no_validations() {
    #[derive(Validate)]
    struct Example(String);
}

#[test]
fn tuple_struct() {
    #[derive(Validate)]
    struct Example(#[vate(StringAlphabetic)] String);

    let example = Example(String::from("!!!"));

    let mut report = Report::new(Accessor::Root("example"));
    let _ = example.validate::<Everything>(&(), &mut report);

    assert!(report.is_any_invalid_at_path(path!(example.0)).unwrap());
}
