#![cfg_attr(not(feature = "std"), no_std, no_main)]

// This will add the default implementation of PSP22 and PSP22Mintable
#[openbrush::implementation(Proxy, Ownable)]
// This macro will collect the traits and override them. Make sure it comes after the implementation macro!
#[openbrush::contract]
pub mod smart_account_example {
  use openbrush::contracts::proxy;
  use openbrush::traits::Storage;
  
  #[ink(storage)] // needed for the ink! contract storage struct
  // this will implement traits needed for OB standards to work with the contract storage struct
  #[derive(Storage, Default)]
  pub struct Contract {
    // we have to add the data structs needed to work with the implemented traits to the storage
    // the fields need to be marked with this attribute in order for the contract to implement neede traits
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
        ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());
        instance
    }

    #[ink(message, payable, selector = _)]
    pub fn forward(&self) {
        proxy::Internal::_fallback(self)
    }
  }
}