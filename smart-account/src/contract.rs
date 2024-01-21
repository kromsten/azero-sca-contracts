// This will add the default implementation of PSP22 and PSP22Mintable
#[openbrush::implementation()]
// This macro will collect the traits and override them. Make sure it comes after the implementation macro!
#[openbrush::contract]
pub mod abstract_account {
  use openbrush::traits::Storage;
  use smart_account_auth::Credentials;
  
  #[ink(storage)] // needed for the ink! contract storage struct
  // this will implement traits needed for OB standards to work with the contract storage struct
  #[derive(Storage, Default)]
  pub struct AccountContract {
    // we have to add the data structs needed to work with the implemented traits to the storage
    // the fields need to be marked with this attribute in order for the contract to implement neede traits
    pub credentials   : Credentials,


  }

  
  impl AccountContract {
    #[ink(constructor)]
    pub fn new(creds: Credentials) -> Self {
        let mut instance = Self::default();

        instance.credentials = creds;
        instance
    }

    #[ink(message)]
    pub fn forward(&self) {}
  }
}
