#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap};

use vate::{Accessor, Alphabetic, InvalidsAndErrors, Iterate, Report, Validate};

#[derive(Validate)]
struct Collections {
    #[vate(Iterate(Alphabetic))]
    vec: Vec<&'static str>,
    #[vate(Iterate(Alphabetic))]
    map: BTreeMap<&'static str, &'static str>,
    #[vate(Iterate(Alphabetic))]
    hashmap: HashMap<&'static str, &'static str>,
}

fn main() {
    let collections = Collections {
        vec: vec!["hello", "world", "!"],
        map: BTreeMap::from([("a", "hello"), ("b", "world"), ("c", "!")]),
        hashmap: HashMap::from([("a", "hello"), ("b", "world"), ("c", "!")]),
    };

    let mut report = Report::new(Accessor::Root("collections"));
    let _ = collections.validate::<InvalidsAndErrors>(&(), &mut report);
    println!("{report:#?}");
}
