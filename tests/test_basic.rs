use default_new::DefaultNew;

#[derive(DefaultNew)]
struct Struct {
    value: i32,
}

impl Struct {
    fn new() -> Self {
        Struct { value: 1 }
    }
}

#[derive(DefaultNew)]
#[cfg(true)]
#[cfg_attr(true, derive(Clone))]
struct StructWithAttributes {
    value: i32,
}

impl StructWithAttributes {
    fn new() -> Self {
        StructWithAttributes { value: 2 }
    }
}

#[test]
fn test_struct() {
    assert_eq!(Struct::default().value, 1);
}

#[test]
fn test_struct_with_attributes() {
    assert_eq!(StructWithAttributes::default().value, 2);
}
