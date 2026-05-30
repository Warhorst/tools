use proc_macro::TokenStream;

mod derive_enum_values;

#[proc_macro_derive(EnumValues)]
pub fn derive_enum_values(item: TokenStream) -> TokenStream {
    derive_enum_values::create(item)
}
