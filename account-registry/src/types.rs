#[derive(scale::Decode, scale::Encode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout, PartialEq, Eq, Debug, Default, Clone)
)]
pub struct Credential {}


#[derive(scale::Decode, scale::Encode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout, PartialEq, Eq, Debug, Default, Clone)
)]
pub struct AccountData {
    pub credentials: Vec<Credential>
}