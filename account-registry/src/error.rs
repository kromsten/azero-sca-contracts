use thiserror::Error;

#[derive(Debug, PartialEq, scale::Encode, scale::Decode, Error)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ContractError {
    #[error("Account already exists")]
    AccountExists,

    #[error("Account doesn't exist")]
    AccountNotExist,

    #[error("Account already has one of provided credentials")]
    HasCredentials,

    #[error("Account doesn't have one of provided credentials")]
    HasNotCredentials,

    #[error("Credential is already linked to another account")]
    CredentialExists,

    #[error("The list of credentials is empty")]
    NoCredentials,

    #[error("{0}")]
    VerifiableAuth(#[from] smart_account_auth::AuthError),
}