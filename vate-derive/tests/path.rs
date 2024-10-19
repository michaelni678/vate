use vate::{path, Accessor};

#[test]
fn only_root() {
    let path = path!(a);
    let expected = [Accessor::Root("a")];

    assert_eq!(path, expected);
}

#[test]
fn consecutive_variants() {
    let path = path!(a[Hello][World]);
    let expected = [
        Accessor::Root("a"),
        Accessor::Variant("Hello"),
        Accessor::Variant("World"),
    ];

    assert_eq!(path, expected);
}

#[test]
fn consecutive_fields() {
    let path = path!(a.b.c);
    let expected = [
        Accessor::Root("a"),
        Accessor::Field("b"),
        Accessor::Field("c"),
    ];

    assert_eq!(path, expected);
}

#[test]
fn consecutive_tuple_indices() {
    let path = path!(a.0 .1);
    let expected = [
        Accessor::Root("a"),
        Accessor::TupleIndex(0),
        Accessor::TupleIndex(1),
    ];

    assert_eq!(path, expected);
}

#[test]
fn consecutive_indices() {
    let path = path!(a[0][1]);
    let expected = [Accessor::Root("a"), Accessor::Index(0), Accessor::Index(1)];

    assert_eq!(path, expected);
}

#[test]
fn consecutive_keys() {
    let path = path!(a["Hello"]["World"]);
    let expected = [
        Accessor::Root("a"),
        Accessor::Key(String::from("Hello")),
        Accessor::Key(String::from("World")),
    ];

    assert_eq!(path, expected);
}

#[test]
fn complex() {
    let path = path!(a.b[0].c["Hello"][World].0);

    let expected = [
        Accessor::Root("a"),
        Accessor::Field("b"),
        Accessor::Index(0),
        Accessor::Field("c"),
        Accessor::Key(String::from("Hello")),
        Accessor::Variant("World"),
        Accessor::TupleIndex(0),
    ];

    assert_eq!(path, expected);
}
