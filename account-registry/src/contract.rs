#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[openbrush::implementation(Proxy, Ownable)]
#[openbrush::contract]
pub mod account_registry {
    
    use ink::storage::Mapping;
    use openbrush::contracts::{proxy, ownable};
    use openbrush::traits::Storage;

    use crate::types::AccountData;

    #[ink(storage)] // needed for the ink! contract storage struct
    // this will implement traits needed for OB standards to work with the contract storage struct
    #[derive(Storage, Default)]
    pub struct Contract {

      accounts: Mapping<AccountId, AccountData>,

      #[storage_field]
      proxy: proxy::Data,
      #[storage_field]
      ownable: ownable::Data
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(forward_to: Hash) -> Self {
            let mut instance = Self::default();
            proxy::Internal::_init_with_forward_to(&mut instance, forward_to);
            instance.accounts = Mapping::default();
            instance
        }

        #[ink(message)]
        pub fn get_account(&self) -> Option<AccountData> {
            let caller = self.env().caller();
            self.accounts.get(caller)
        }

        #[ink(message, payable)]
        pub fn create_account(&mut self, data: AccountData) {
            let caller = self.env().caller();
            self.accounts.insert(caller, &data);
        }

        #[ink(message, payable, selector = _)]
        pub fn forward(&self) {
            proxy::Internal::_fallback(self)
        }
    }
}
