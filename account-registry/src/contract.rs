#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[openbrush::implementation(Ownable)]
#[openbrush::contract]
pub mod account_registry {
    use crate::{
        account::{
            AccountData,
            get_account, 
            save_account_data, 
            add_local_credentials, 
            remove_local_credentials, 
        }, 
        error::ContractError
    };

    use ink::storage::Mapping;
    use openbrush::{
        contracts::{proxy, ownable}, traits::Storage
    };
    use smart_account_auth::{
        CredentialData, CredentialId, Verifiable, CredentialWrapper
    };
    use azero_smart_account::AccountContractRef;


    #[ink(storage)] // needed for the ink! contract storage struct
    // this will implement traits needed for OB standards to work with the contract storage struct
    #[derive(Storage, Default)]
    pub struct RegistryContract {
      /// links the main credential to Smart Account Data
      pub accounts         : Mapping<CredentialId, AccountData>,
      /// maps secondary credentials to the main one
      pub credential_ids   : Mapping<CredentialId, CredentialId>,

      pub account_hash     : Hash,

      #[storage_field]
      proxy: proxy::Data,
      #[storage_field]
      ownable: ownable::Data
    }


    impl RegistryContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            instance.accounts = Mapping::default();
            instance
        }

        #[ink(message)]
        pub fn get_account(&self, creds: CredentialData) -> Option<AccountData> {
            if creds.with_caller.is_some() && creds.with_caller.unwrap() {
                get_account(self, &creds.with_caller_ink(&self.env().caller()))
            } else {
                get_account(self, &creds)
            }
        }

        #[ink(message, payable)]
        pub fn create_account(&mut self, creds: CredentialData) -> Result<(), ContractError> {

            let verified = creds.verified_ink(&self.env())?;
            
            // TODO: create account and get address
            let new_acc = AccountContractRef::new(verified.credentials.clone())
                .code_hash(self.account_hash)
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();
            
            save_account_data(self, &verified, new_acc)?;

            Ok(())
        }


        #[ink(message, payable)]
        pub fn update_credentials(
            &mut self, 
            auth_credentials: CredentialData,
            add_credentials:  CredentialData
        ) -> Result<(), ContractError> {
            auth_credentials.verify()?;

            let primary_id = &auth_credentials.primary_id();
            let account_data = self.accounts.get(primary_id).ok_or(ContractError::AccountNotExist)?;

            let _new_ids = add_local_credentials(
                self, 
                primary_id, 
                &account_data, 
                &add_credentials.secondary_ids()
            )?;

            //AccountContractRef::forward()?;
            // TODO: update account contract with new credentials

            Ok(())
        }

        #[ink(message)]
        pub fn remove_credentials(
            &mut self, 
            auth_credentials:    CredentialData,
            remove_credentials:  CredentialData
        ) -> Result<(), ContractError> {
            auth_credentials.verify()?;
            let primary_id = &auth_credentials.primary_id();
            let account_data = self.accounts.get(primary_id).ok_or(ContractError::AccountNotExist)?;

            let _new_ids = remove_local_credentials(
                self, 
                primary_id, 
                &account_data, 
                &remove_credentials.secondary_ids()
            )?;

            // TODO: update account contract with new credentials
            Ok(())
        }

    }
}
