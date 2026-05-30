pub use proc_macros::EnumValues;

pub trait EnumValues {
    /// Return all the possible values from an enum.
    fn values() -> impl Iterator<Item=Self>;
}
