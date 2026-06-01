use tools::enums::EnumValues;

/// Check if the EnumValues derive works for an enum with only unit variants
#[test]
fn test_only_unit() {
    #[derive(EnumValues, Eq, PartialEq, Debug)]
    enum Values {
        Foo,
        Bar,
        Baz,
    }

    assert_eq!(
        vec![Values::Foo, Values::Bar, Values::Baz],
        Values::values().collect::<Vec<_>>()
    )
}

/// Check if the EnumValues derive works for an enum with unnamed variants which single parameter type
/// also implements EnumValues.
#[test]
fn test_recursion() {
    #[derive(EnumValues, Eq, PartialEq, Debug)]
    enum ValuesOuter {
        Foo,
        Bar(ValuesInner),
        Baz,
    }

    #[derive(EnumValues, Eq, PartialEq, Debug)]
    enum ValuesInner {
        Oof,
        Rab,
    }

    assert_eq!(
        vec![
            ValuesOuter::Foo,
            ValuesOuter::Bar(ValuesInner::Oof),
            ValuesOuter::Bar(ValuesInner::Rab),
            ValuesOuter::Baz
        ],
        ValuesOuter::values().collect::<Vec<_>>()
    )
}
