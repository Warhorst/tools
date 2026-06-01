use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, Item, ItemEnum, parse_macro_input, spanned::Spanned};

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

    if is_all_unit(&item_enum) {
        create_all_unit_impl(item_enum)
    } else if is_recursion_possible(&item_enum) {
        create_recursive_impl(item_enum)
    } else {
        syn::Error::new(
            item_enum.span(),
            "EnumValues can only be implemented for enums with all unit variants.",
        )
        .to_compile_error()
        .into()
    }
}

fn is_all_unit(item_enum: &ItemEnum) -> bool {
    item_enum
        .variants
        .iter()
        .all(|v| matches!(v.fields, Fields::Unit))
}

fn create_all_unit_impl(item_enum: ItemEnum) -> TokenStream {
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

fn is_recursion_possible(item_enum: &ItemEnum) -> bool {
    item_enum.variants.iter().all(|v| match &v.fields {
        // Named variants are not supported
        Fields::Named(_) => false,
        // If it is a unnamed variant, it must have only one field, which we expect to impl EnumValues
        Fields::Unnamed(f) => f.unnamed.len() == 1,
        Fields::Unit => true,
    })
}

fn create_recursive_impl(item_enum: ItemEnum) -> TokenStream {
    let ident = item_enum.ident;
    let variants = item_enum.variants.iter().map(|v| match &v.fields {
        Fields::Unnamed(fields_unnamed) => {
            let ty = &fields_unnamed
                .unnamed
                .first()
                .expect("The variant must only have one field")
                .ty;

            let ident = &v.ident;

            quote! {
                values.extend(#ty::values().map(#ident));
            }
        }
        Fields::Unit => quote! {
            values.extend(Some(#v));
        },
        Fields::Named(_) => unreachable!("Named variants are filtered before this call"),
    });

    quote! {
        impl EnumValues for #ident {
            fn values() -> impl Iterator<Item=Self> {
                use #ident::*;

                let mut values = vec![];

                #(#variants)*

                values.into_iter()

            }
        }
    }
    .into()
}
