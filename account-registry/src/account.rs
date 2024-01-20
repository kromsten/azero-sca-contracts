use ink::primitives::AccountId;
use smart_account_auth::{CredentialData, CredentialId};
use crate::{contract::account_registry::RegistryContract, error::ContractError};


#[derive(scale::Decode, scale::Encode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct AccountData {
    pub address: AccountId,
    pub credential_ids: Vec<CredentialId>
}

impl AccountData {
    pub fn new(
        address: AccountId,
        credential_ids: Vec<CredentialId>
    ) -> Self {
        Self {
            address,
            credential_ids
        }
    }

    pub fn has_credentials(&self, ids: &Vec<CredentialId>) -> bool {
        self.credential_ids.iter().all(|id| {
            ids.iter().any(|cred| cred == id)
        })
    }
}

impl Default for AccountData {
    fn default() -> Self {
        Self {
            address: AccountId::from([0x0; 32]),
            credential_ids: Vec::new()
        }
    }
}


pub fn get_account(contract: &RegistryContract, creds: &CredentialData) -> Option<AccountData> {
    let data =  contract.accounts.get(&creds.primary_id());
    if data.is_some() {
        return data;
    }

    let data : Option<AccountData> = creds
        .secondary_ids()
        .iter()
        .find_map(|id| {
            match contract.credential_ids.get(&id) {
                Some(id) => {
                    contract.accounts.get(&id).map(|data| {
                        return data;
                    })
                },
                None => None
            }
    });

    if let Some(data) = data {
        if data.has_credentials(&creds.secondary_ids()) {
            return Some(data);
        }
    }

    None

}


pub fn account_exists(contract: &RegistryContract, creds: &CredentialData) -> bool {
    contract.accounts.get(&creds.primary_id()).is_some()
}

pub fn credential_exists(contract: &RegistryContract, id: &CredentialId) -> bool {
    contract.credential_ids.get(&id).is_some()
}


pub fn save_credentials(
    contract:       &mut RegistryContract, 
    primary_id:     &CredentialId,
    credential_ids: &Vec<CredentialId>
) -> Result<(), ContractError> {

    let account_data = contract.accounts.get(primary_id);
    if let None = account_data {
        return Err(ContractError::AccountNotExist);
    }

    for id in credential_ids {
        if let Some(_) = contract.credential_ids.get(&id) {
            return  Err(ContractError::CredentialExists);
        }
        contract.credential_ids.insert(id, primary_id);
    }
    Ok(())
}


pub fn save_account(
    contract: &mut RegistryContract, 
    creds: &CredentialData,
    address: AccountId
) -> Result<(), ContractError> {

    contract.accounts.get(&creds.primary_id()).map(|data| data);
    if let Some(_) = get_account(contract, creds) {
        return Err(ContractError::AccountExists);
    }

    let account_id = creds.primary_id();
    let data = AccountData {
        address,
        credential_ids:     creds.ids()
    };

    contract.accounts.insert(&account_id, &data);
    save_credentials(contract, &account_id, &creds.secondary_ids())?;
    Ok(())
}