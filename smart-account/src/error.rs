use openbrush::contracts::upgradeable::OwnableError;
use thiserror::Error;

#[derive(Debug, PartialEq, Clone, scale::Encode, scale::Decode, Error)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ContractError {

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Empty credential")]
    EmptyCredentials,

    #[error("Account already exists")]
    AccountExists,

    #[error("Account doesn't exist")]
    AccountNotExist,

    #[error("Credential already exists")]
    CredentialExists,

    #[error("The list of credentials is empty")]
    NoCredentials,

    #[error("Transaction failed")]
    TransactionFailed,

    #[error("{0}")]
    VerifiableAuth(#[from] smart_account_auth::AuthError),
}

impl From<OwnableError> for ContractError {
    fn from(error: OwnableError) -> Self {
        match error {
            OwnableError::CallerIsNotOwner => ContractError::Unauthorized("Callet is not owner".to_string()),
            OwnableError::NewOwnerIsNotSet => ContractError::Unauthorized("New onwer is not set".to_string()),
        }
    }
}