use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, Item, parse_macro_input, spanned::Spanned};

pub(crate) fn create(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);

    let item_enum = if let Item::Enum(item_enum) = item {
        item_enum
    } else {
        return syn::Error::new(
            item.span(),
            "The enum_values macro is only supportet on enums",
        )
        .to_compile_error()
        .into();
    };

    let all_unit = item_enum.variants.iter().all(|v| {
        matches!(v.fields, Fields::Unit)
    });

    if !all_unit {
        return syn::Error::new(
            item_enum.span(),
            "EnumValues can only be implemented for enums with all unit variants.",
        )
        .to_compile_error()
        .into();
    }

    let ident = item_enum.ident;
    let variants = item_enum.variants.iter();

    quote! {
    impl EnumValues for #ident {
        fn values() -> impl Iterator<Item=Self> {
            use #ident::*;
            [#(#variants),*].into_iter()
        }
    }}
    .into()
}
