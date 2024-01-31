// This will add the default implementation of PSP22 and PSP22Mintable
#[openbrush::implementation(Ownable)]
// This macro will collect the traits and override them. Make sure it comes after the implementation macro!
#[openbrush::contract]
pub mod abstract_account {
  use ink::env::{call::{build_call, ExecutionInput}, CallFlags};
  use openbrush::{modifiers, traits::Storage};
  use scale::Output;
  use saa::{CredentialData, Credentials, Verifiable};
  use openbrush::contracts::ownable::*;

  use ink::prelude::vec::Vec;

  
  use crate::error::ContractError;
  
  // type TransactionId = u32;

  #[ink(event)]
    pub struct Execution {
        /// Indicates whether the transaction executed successfully. If so the `Ok` value
        /// holds the output in bytes. The Option is `None` when the transaction
        /// was executed through `invoke_transaction` rather than
        /// `evaluate_transaction`.
        #[ink(topic)]
        result: Result<Option<Vec<u8>>, ContractError>,
    }

  struct CallInput<'a>(&'a [u8]);
  impl<'a> scale::Encode for CallInput<'a> {
    fn encode_to<T: Output + ?Sized>(&self, dest: &mut T) {
        dest.write(self.0);
    }
}

  #[derive(scale::Decode, scale::Encode)]
  #[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq,
      scale_info::TypeInfo, ink::storage::traits::StorageLayout
  ))]
  pub struct Transaction {

    /// The `AccountId` of the contract that is called in this transaction.
    pub callee: AccountId,

    /// The selector bytes that identifies the function of the callee that should be
    /// called.
    pub selector: [u8; 4],

    /// The SCALE encoded parameters that are passed to the called function.
    pub input: Vec<u8>,

    /// Gas limit for the execution of the call.
    pub gas_limit: u64,

    /// If set to true the transaction will be allowed to re-enter the multisig
    /// contract. Re-entrancy can lead to vulnerabilities. Use at your own
    /// risk.
    pub allow_reentry: bool,
}


  #[ink(storage)] // needed for the ink! contract storage struct
  // this will implement traits needed for OB standards to work with the contract storage struct
  #[derive(Storage, Default)]
  pub struct AccountContract {
    // we have to add the data structs needed to work with the implemented traits to the storage
    // the fields need to be marked with this attribute in order for the contract to implement neede traits
    pub credentials   : Credentials,

    #[storage_field]
    pub ownable: ownable::Data,
  }


  
  impl AccountContract {
    #[ink(constructor)]
    pub fn new(creds: CredentialData) -> Self {
        let mut instance = Self::default();
        instance.credentials = creds.credentials;
        ownable::Internal::_init_with_owner(&mut instance, Self::env().caller());
        instance
    }

    #[modifiers(only_owner)]
    pub fn update_credentials(&mut self, creds: CredentialData) -> Result<(), ContractError> {
        self.credentials = creds.credentials;
        Ok(())
    }


    pub fn get_credentials(&self) -> Credentials {
        self.credentials.clone()
    }


    #[ink(message, payable)]
    pub fn invoke_transaction(
        &mut self,
        creds: CredentialData,
        tranasction : Transaction
    ) -> Result<(), ContractError> {

        if creds.credentials.iter().any(|c| !self.credentials.contains(c)) {
            return Err(ContractError::Unauthorized("Not valid credentials".into()));
        }

        creds.verified_ink(self.env())?;

        let t = tranasction;

        let result = build_call::<<Self as ::ink::env::ContractEnv>::Env>()
            .call(t.callee)
            .gas_limit(t.gas_limit)
            .transferred_value(self.env().transferred_value())
            .call_flags(CallFlags::default().set_allow_reentry(t.allow_reentry))
            .exec_input(
                ExecutionInput::new(t.selector.into()).push_arg(CallInput(&t.input)),
            )
            .returns::<()>()
            .try_invoke();

        let result = match result {
            Ok(Ok(_)) => Ok(()),
            _ => Err(ContractError::TransactionFailed),
        };

        self.env().emit_event(Execution {
            result: result.clone().map(|_| None),
        });
        
        result
    }


  }
}
