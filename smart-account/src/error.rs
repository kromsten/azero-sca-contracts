use thiserror::Error;

#[derive(Debug, PartialEq, scale::Encode, scale::Decode, Error)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ContractError {
    #[error("Account already exists")]
    AccountExists,

    #[error("Account doesn't exist")]
    AccountNotExist,

    #[error("Credential already exists")]
    CredentialExists,

    #[error("The list of credentials is empty")]
    NoCredentials,

    #[error("{0}")]
    VerifiableAuth(#[from] smart_account_auth::AuthError),
}