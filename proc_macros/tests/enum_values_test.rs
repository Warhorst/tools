use tools::enums::EnumValues;

#[test]
fn test() {
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
