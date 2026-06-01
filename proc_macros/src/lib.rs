use proc_macro::TokenStream;

mod derive_enum_values;

/// Generates an implementation of EnumValues for the item.
///
/// This derive is supported on two types of enums:
/// - Enums where all variants are unit variants
/// - Enums where all variants are unit variants or Unnamed variants with exactly one field. In this case, the derive will recursively
///   call `EnumValues::values` on the type of the field.
///
/// Examples:
/// ```rust
/// use tools::enums::EnumValues;
/// 
/// #[derive(EnumValues, Eq, PartialEq, Debug)]
/// enum Letter {
///    A,
///    B,
///    C
/// }
///
/// assert_eq!(
///    vec![Letter::A, Letter::B, Letter::C],
///    Letter::values().collect::<Vec<_>>()
/// );
/// ```
///
/// ```rust
/// use tools::enums::EnumValues;
/// 
/// #[derive(EnumValues, Eq, PartialEq, Debug)]
/// enum Letter {
///    A,
///    B(Number),
///    C
/// }
///
/// #[derive(EnumValues, Eq, PartialEq, Debug)]
/// enum Number {
///    N1,
///    N2
/// }
///
/// assert_eq!(
///    vec![Letter::A, Letter::B(Number::N1), Letter::B(Number::N2), Letter::C],
///    Letter::values().collect::<Vec<_>>()
/// )
/// ```
///
///
#[proc_macro_derive(EnumValues)]
pub fn derive_enum_values(item: TokenStream) -> TokenStream {
    derive_enum_values::create(item)
}
