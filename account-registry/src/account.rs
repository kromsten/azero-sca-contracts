use smart_account_auth::{ensure, CredentialData, CredentialId, CredentialsWrapper};
use crate::{contract::account_registry::RegistryContract, error::ContractError};

use azero_smart_account::AccountContractRef as AccountRef;



#[derive(Clone, PartialEq, Debug, scale::Decode, scale::Encode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
pub struct AccountData {
    pub account: AccountRef,
    pub credential_ids: Vec<CredentialId>
}

impl AccountData {
    pub fn new(
        account: AccountRef,
        credential_ids: Vec<CredentialId>
    ) -> Self {
        Self {
            account,
            credential_ids
        }
    }

    pub fn has_credentials(&self, ids: &Vec<CredentialId>) -> bool {
        self.credential_ids.iter().all(|id| {
            ids.iter().any(|cred| cred == id)
        })
    }
}



pub fn get_account(contract: &RegistryContract, creds: &CredentialData) -> Option<AccountData> {

    if creds.credentials.is_empty() {
        return None;
    }


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



pub fn save_account_data(
    contract: &mut RegistryContract, 
    creds: &CredentialData,
    account: AccountRef
) -> Result<(), ContractError> {
    ensure!(contract.accounts.contains(&creds.primary_id()), ContractError::AccountExists);

    let primary_id = &creds.primary_id();

    let data = AccountData {
        account,
        credential_ids:     creds.ids()
    };

    contract.accounts.insert(primary_id, &data);
    
    for id in creds.secondary_ids().iter() {
        ensure!(!contract.credential_ids.contains(&id), ContractError::CredentialExists);
        contract.credential_ids.insert(id, primary_id);
    }

    Ok(())
}



pub fn add_local_credentials(
    contract:           &mut RegistryContract, 
    primary_id:         &CredentialId,
    account_data:       &AccountData,
    add_credentials:    &Vec<CredentialId>
) -> Result<Vec<CredentialId>, ContractError> {
    ensure!(!account_data.has_credentials(add_credentials), ContractError::HasCredentials);
    let mut new_ids : Vec<CredentialId> = Vec::with_capacity(
        account_data.credential_ids.len() + add_credentials.len()
    );
    
    new_ids.extend(account_data.credential_ids.iter().cloned());

    for id in add_credentials {
        if !contract.credential_ids.contains(&id) {
            contract.credential_ids.insert(id, primary_id);
        }
        new_ids.push(id.clone());
    }

    contract.accounts.insert(primary_id, &AccountData {
        account: account_data.account.clone(),
        credential_ids: new_ids.clone()
    });

    Ok(new_ids)
}


pub fn remove_local_credentials(
    contract:               &mut RegistryContract, 
    primary_id:             &CredentialId,
    account_data:           &AccountData,
    remove_credentials:     &Vec<CredentialId>
) -> Result<Vec<CredentialId>, ContractError> {
    ensure!(account_data.has_credentials(remove_credentials), ContractError::HasNotCredentials);

    let mut new_ids : Vec<CredentialId> = Vec::with_capacity(
        account_data.credential_ids.len() - remove_credentials.len()
    );

    for id in remove_credentials {
        if account_data.credential_ids.contains(&id) {
            contract.credential_ids.remove(id);
        } else {
            new_ids.push(id.clone());
        }
    }

    contract.accounts.insert(primary_id, &AccountData {
        account: account_data.account.clone(),
        credential_ids: new_ids.clone()
    });

    Ok(new_ids)
}