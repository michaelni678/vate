#![allow(dead_code)]

use vate::{path, Accessor, Everything, Nested, Report, StringAlphabetic, Validate};

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

#[test]
fn unit_enum() {
    #[derive(Validate)]
    enum Example {}
}

#[test]
fn unit_variant_enum() {
    #[derive(Validate)]
    enum Example {
        Variant,
    }
}

#[test]
fn named_variant_enum() {
    #[derive(Validate)]
    enum Example {
        Variant {
            #[vate(StringAlphabetic)]
            a: String,
        },
    }

    let example = Example::Variant {
        a: String::from("!!!"),
    };

    let mut report = Report::new(Accessor::Root("example"));
    let _ = example.validate::<Everything>(&(), &mut report);

    assert!(report
        .is_any_invalid_at_path(path!(example[Variant].a))
        .unwrap());
}

#[test]
fn unnamed_variant_enum() {
    #[derive(Validate)]
    enum Example {
        Variant(#[vate(StringAlphabetic)] String),
    }

    let example = Example::Variant(String::from("!!!"));

    let mut report = Report::new(Accessor::Root("example"));
    let _ = example.validate::<Everything>(&(), &mut report);

    assert!(report
        .is_any_invalid_at_path(path!(example[Variant].0))
        .unwrap());
}

#[test]
fn assorted_variant_enum() {
    #[derive(Validate)]
    enum Example {
        Variant1,
        Variant2 {
            #[vate(StringAlphabetic)]
            a: String,
        },
        Variant3(#[vate(StringAlphabetic)] String),
    }

    let example = Example::Variant3(String::from("!!!"));

    let mut report = Report::new(Accessor::Root("example"));
    let _ = example.validate::<Everything>(&(), &mut report);

    assert!(report
        .is_any_invalid_at_path(path!(example[Variant3].0))
        .unwrap());
}

#[test]
fn nested_enum() {
    #[derive(Validate)]
    enum Example1 {
        Example2(#[vate(Nested)] Example2),
    }

    #[derive(Validate)]
    enum Example2 {
        Variant {
            #[vate(StringAlphabetic)]
            a: String,
        },
    }

    let example1 = Example1::Example2(Example2::Variant {
        a: String::from("!!!"),
    });

    let mut report = Report::new(Accessor::Root("example1"));
    let _ = example1.validate::<Everything>(&(), &mut report);

    assert!(report
        .is_any_invalid_at_path(path!(example1[Example2].0[Variant].a))
        .unwrap());
}
