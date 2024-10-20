#![allow(dead_code, unused_variables)]

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
fn nested_regular_structs() {
    #[derive(Validate)]
    struct Example1 {
        #[vate(Nested)]
        example2: Example2,
    }

    #[derive(Validate)]
    struct Example2 {
        #[vate(StringAlphabetic)]
        a: String,
    }

    let example1 = Example1 {
        example2: Example2 {
            a: String::from("!!!"),
        },
    };

    let mut report = Report::new(Accessor::Root("example1"));
    let _ = example1.validate::<Everything>(&(), &mut report);

    assert!(report
        .is_any_invalid_at_path(path!(example1.example2.a))
        .unwrap());
}

#[test]
fn nested_tuple_structs() {
    #[derive(Validate)]
    struct Example1(#[vate(Nested)] Example2);

    #[derive(Validate)]
    struct Example2(#[vate(StringAlphabetic)] String);

    let example1 = Example1(Example2(String::from("!!!")));

    let mut report = Report::new(Accessor::Root("example1"));
    let _ = example1.validate::<Everything>(&(), &mut report);

    assert!(report.is_any_invalid_at_path(path!(example1.0 .0)).unwrap());
}

#[test]
fn nested_regular_and_tuple_structs() {
    #[derive(Validate)]
    struct Example1 {
        #[vate(Nested)]
        example2: Example2,
    }

    #[derive(Validate)]
    struct Example2(#[vate(StringAlphabetic)] String);

    let example1 = Example1 {
        example2: Example2(String::from("!!!")),
    };

    let mut report = Report::new(Accessor::Root("example1"));
    let _ = example1.validate::<Everything>(&(), &mut report);

    assert!(report
        .is_any_invalid_at_path(path!(example1.example2.0))
        .unwrap());
}

#[test]
fn nested_tuple_and_regular_structs() {
    #[derive(Validate)]
    struct Example1(#[vate(Nested)] Example2);

    #[derive(Validate)]
    struct Example2 {
        #[vate(StringAlphabetic)]
        a: String,
    }

    let example1 = Example1(Example2 {
        a: String::from("!!!"),
    });

    let mut report = Report::new(Accessor::Root("example1"));
    let _ = example1.validate::<Everything>(&(), &mut report);

    assert!(report.is_any_invalid_at_path(path!(example1.0.a)).unwrap());
}

#[test]
fn regular_struct_multiple_fields() {
    #[derive(Validate)]
    struct Example {
        #[vate(StringAlphabetic)]
        a: String,
        #[vate(StringAlphabetic)]
        b: String,
    }

    let example = Example {
        a: String::from("!!!"),
        b: String::from("@@@"),
    };

    let mut report = Report::new(Accessor::Root("example"));
    let _ = example.validate::<Everything>(&(), &mut report);

    assert!(report.is_any_invalid_at_path(path!(example.a)).unwrap());
    assert!(report.is_any_invalid_at_path(path!(example.b)).unwrap());
}

#[test]
fn tuple_struct_multiple_fields() {
    #[derive(Validate)]
    struct Example(
        #[vate(StringAlphabetic)] String,
        #[vate(StringAlphabetic)] String,
    );

    let example = Example(String::from("Hello"), String::from("!!!"));

    let mut report = Report::new(Accessor::Root("example"));
    let _ = example.validate::<Everything>(&(), &mut report);

    assert!(report.is_all_valid_at_path(path!(example.0)).unwrap());
    assert!(report.is_any_invalid_at_path(path!(example.1)).unwrap());
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

    println!("{report:#?}");
    println!("{:?}", path!(example[Variant].0));

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
