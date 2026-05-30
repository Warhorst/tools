use proc_macro::TokenStream;

mod derive_enum_values;

/// Generates an implementation of EnumValues for the item. This is only supported
/// on enums which only consist of unit variants.
#[proc_macro_derive(EnumValues)]
pub fn derive_enum_values(item: TokenStream) -> TokenStream {
    derive_enum_values::create(item)
}
