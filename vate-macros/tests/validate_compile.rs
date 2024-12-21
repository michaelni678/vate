#![allow(dead_code, unused_variables)]

use vate::{
    core::Validate,
    validators::{compare::EQ, nested::Nested},
};

#[test]
fn unit_struct() {
    #[derive(Validate)]
    struct Test;
}

#[test]
fn regular_struct_empty() {
    #[derive(Validate)]
    struct Test {}
}

#[test]
fn regular_struct_no_validations() {
    #[derive(Validate)]
    struct Test {
        a: i32,
    }
}

#[test]
fn regular_struct() {
    #[derive(Validate)]
    struct Test {
        #[vate(EQ(5))]
        a: i32,
    }
}

#[test]
fn tuple_struct_empty() {
    #[derive(Validate)]
    struct Test();
}

#[test]
fn tuple_struct_no_validations() {
    #[derive(Validate)]
    struct Test(i32);
}

#[test]
fn tuple_struct() {
    #[derive(Validate)]
    struct Test(#[vate(EQ(5))] i32);
}

#[test]
fn nested_regular_structs() {
    #[derive(Validate)]
    struct Test1 {
        #[vate(Nested)]
        test2: Test2,
    }

    #[derive(Validate)]
    struct Test2 {
        #[vate(EQ(5))]
        a: i32,
    }
}

#[test]
fn nested_tuple_structs() {
    #[derive(Validate)]
    struct Test1(#[vate(Nested)] Test2);

    #[derive(Validate)]
    struct Test2(#[vate(EQ(5))] i32);
}

#[test]
fn nested_regular_and_tuple_structs() {
    #[derive(Validate)]
    struct Test1 {
        #[vate(Nested)]
        test2: Test2,
    }

    #[derive(Validate)]
    struct Test2(#[vate(EQ(5))] i32);
}

#[test]
fn nested_tuple_and_regular_structs() {
    #[derive(Validate)]
    struct Test1(#[vate(Nested)] Test2);

    #[derive(Validate)]
    struct Test2 {
        #[vate(EQ(5))]
        a: i32,
    }
}

#[test]
fn regular_struct_multiple_fields() {
    #[derive(Validate)]
    struct Test {
        #[vate(EQ(5))]
        a: i32,
        #[vate(EQ(5))]
        b: i32,
    }
}

#[test]
fn tuple_struct_multiple_fields() {
    #[derive(Validate)]
    struct Test(#[vate(EQ(5))] i32, #[vate(EQ(5))] i32);
}

#[test]
fn regular_struct_field_exposure() {
    #[derive(Validate)]
    struct Test {
        a: i32,
        #[vate(EQ(a))]
        b: i32,
    }
}

#[test]
fn tuple_struct_field_exposure() {
    #[derive(Validate)]
    struct Test(i32, #[vate(EQ(fields.0))] i32);
}

#[test]
fn unit_enum() {
    #[derive(Validate)]
    enum Test {}
}

#[test]
fn unit_variant_enum() {
    #[derive(Validate)]
    enum Test {
        A,
    }
}

#[test]
fn named_variant_enum_no_validations() {
    #[derive(Validate)]
    enum Test {
        A { x: i32 },
    }
}

#[test]
fn unnamed_variant_enum_no_validations() {
    #[derive(Validate)]
    enum Test {
        A(i32),
    }
}

#[test]
fn named_variant_enum() {
    #[derive(Validate)]
    enum Test {
        A {
            #[vate(EQ(5))]
            x: i32,
        },
    }
}

#[test]
fn unnamed_variant_enum() {
    #[derive(Validate)]
    enum Test {
        A(#[vate(EQ(5))] i32),
    }
}

#[test]
fn named_variant_enum_multiple_fields() {
    #[derive(Validate)]
    enum Test {
        A {
            #[vate(EQ(5))]
            x: i32,
            #[vate(EQ(5))]
            y: i32,
        },
    }
}

#[test]
fn unnamed_variant_enum_multiple_fields() {
    #[derive(Validate)]
    enum Test {
        A(#[vate(EQ(5))] i32, #[vate(EQ(5))] i32),
    }
}

#[test]
fn assorted_variant_enum() {
    #[derive(Validate)]
    enum Test {
        A,
        B {
            #[vate(EQ(5))]
            x: i32,
        },
        C(#[vate(EQ(5))] i32),
    }
}

#[test]
fn nested_enum() {
    #[derive(Validate)]
    enum Test1 {
        Test2(#[vate(Nested)] Test2),
    }

    #[derive(Validate)]
    enum Test2 {
        A {
            #[vate(EQ(5))]
            x: i32,
        },
    }
}

#[test]
fn named_variant_enum_field_exposure() {
    #[derive(Validate)]
    enum Test {
        A {
            a: i32,
            #[vate(EQ(a))]
            b: i32,
        },
    }
}

#[test]
fn unnamed_variant_enum_field_exposure() {
    #[derive(Validate)]
    enum Test {
        A(i32, #[vate(EQ(fields.0))] i32),
    }
}

#[test]
fn struct_context_type() {
    #[derive(Validate)]
    #[vate(context = i32)]
    struct Test;
}

#[test]
fn enum_context_type() {
    #[derive(Validate)]
    #[vate(context = i32)]
    enum Test {}
}

#[test]
fn struct_error_type() {
    #[derive(Validate)]
    #[vate(error = i32)]
    struct Test;
}

#[test]
fn enum_error_type() {
    #[derive(Validate)]
    #[vate(error = i32)]
    enum Test {}
}
