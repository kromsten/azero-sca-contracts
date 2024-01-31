
#[derive(Debug, PartialEq, scale::Encode, scale::Decode, Clone)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ContractError {
    AccountExists,
    AccountNotExist,
    HasCredentials,
    HasNotCredentials,
    CredentialExists,
    NoCredentials,
    VerifiableAuth(smart_account_auth::AuthError),
}

impl From<smart_account_auth::AuthError> for ContractError {
    fn from(err: smart_account_auth::AuthError) -> Self {
        ContractError::VerifiableAuth(err)
    }
}