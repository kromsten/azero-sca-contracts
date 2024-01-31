use openbrush::contracts::ownable::OwnableError;
use ink::prelude::string::String;

#[derive(Debug, PartialEq, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ContractError {

    Unauthorized(String),

    EmptyCredentials,

    AccountExists,

    AccountNotExist,

    CredentialExists,

    NoCredentials,

    TransactionFailed,

    VerifiableAuth(saa::AuthError),
}

impl From<OwnableError> for ContractError {
    fn from(error: OwnableError) -> Self {
        match error {
            OwnableError::CallerIsNotOwner => ContractError::Unauthorized("Callet is not owner".into()),
            OwnableError::NewOwnerIsNotSet => ContractError::Unauthorized("New onwer is not set".into()),
        }
    }
}

impl From<saa::AuthError> for ContractError {
    fn from(error: saa::AuthError) -> Self {
        ContractError::VerifiableAuth(error)
    }
}