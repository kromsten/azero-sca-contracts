mod tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use sca_common::{AccountData, AuthType};
    use contract::account_registry::Contract;
    use smart_account_auth::{CredentialId, AuthError, Credential, credentials::EvmCredential};
    use crate::contract;


    type Credentials = Vec<Box<dyn Credential>>;

    fn evm_creds() -> Credentials {
        vec![Box::new(EvmCredential { 
            message: Vec::default(), 
            signature: Vec::default(), 
            signer: Vec::default() 
        })]
    }

    fn creds() -> Credentials {
        evm_creds()
    }

    fn other_creds() -> Credentials {
        vec![Box::new(EvmCredential { 
            message: Vec::default(), 
            signature: Vec::default(), 
            signer: vec![1, 2, 3] 
        })]
    }

    fn acc_data() -> AccountData {
        AccountData {
            auth_types: vec![AuthType::Passkey {}],
        }
    }

    /// We test if the default constructor does its job.
    #[ink::test]
    fn get_creds_init() {
        let contract =  Contract::default();
        let creds = creds();
        assert_eq!(contract.get_account(creds), None);
    }

    /// We test a simple use case of our contract.
    #[ink::test]
    fn create_account() {
        let mut contract =  Contract::default();

        let creds = creds();

        assert!(contract.create_account(creds.clone()).is_ok());
        assert!(contract.create_account(creds.clone()).is_err());

        assert_eq!(contract.get_account(other_creds()), None);
        assert_eq!(contract.get_account(creds), Some(acc_data()));

        assert!(false)
    }
}
    