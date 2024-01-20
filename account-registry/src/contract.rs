#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[openbrush::implementation(Proxy, Ownable)]
#[openbrush::contract]
pub mod account_registry {
    use crate::{account::{get_account, remove_local_credentials, save_account_data, add_local_credentials, AccountData}, error::ContractError};

    use ink::storage::Mapping;
    use openbrush::{
        contracts::{proxy, ownable}, traits::Storage
    };
    use smart_account_auth::{
        CredentialData, CredentialId, Verifiable
    };


    #[ink(storage)] // needed for the ink! contract storage struct
    // this will implement traits needed for OB standards to work with the contract storage struct
    #[derive(Storage, Default)]
    pub struct RegistryContract {
      /// links the main credential to Smart Account Data
      pub accounts         : Mapping<CredentialId, AccountData>,
      /// maps secondary credentials to the main one
      pub credential_ids   : Mapping<CredentialId, CredentialId>,

      #[storage_field]
      proxy: proxy::Data,
      #[storage_field]
      ownable: ownable::Data
    }


    impl RegistryContract {
        #[ink(constructor)]
        pub fn new(forward_to: Hash) -> Self {
            let mut instance = Self::default();
            proxy::Internal::_init_with_forward_to(&mut instance, forward_to);
            instance.accounts = Mapping::default();
            instance
        }

        #[ink(message)]
        pub fn get_account(&self, creds: CredentialData) -> Option<AccountData> {
            get_account(self, &creds)
        }

        #[ink(message, payable)]
        pub fn create_account(&mut self, creds: CredentialData) -> Result<(), ContractError> {
            creds.verify()?;
            // TODO: create account and get address
            let new_account_address = AccountId::from([0x0; 32]);

            save_account_data(self, &creds, new_account_address)
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


        #[ink(message, payable, selector = _)]
        pub fn forward(&self) {
            proxy::Internal::_fallback(self)
        }
    }
}
