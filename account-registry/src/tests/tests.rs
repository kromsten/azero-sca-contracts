mod tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use contract::account_registry::RegistryContract as Contract;
    use smart_account_auth::{AuthError, CredentialData};
    use crate::{contract, error::ContractError};



    fn creds() -> CredentialData {
        CredentialData {
            credentials: vec![],
            primary_index: None,
            with_caller: Some(true)
        }
    }

    fn other_creds() -> CredentialData {
        CredentialData {
            credentials: vec![],
            primary_index: Some(0),
            with_caller: Some(true)}
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

        assert_eq!(contract.create_account(creds.clone()).unwrap_err(), ContractError::VerifiableAuth(AuthError::RecoveryParam));
        assert!(contract.create_account(creds.clone()).is_err());

        assert_eq!(contract.get_account(other_creds()), None);
        assert!(contract.get_account(creds).is_some());

        assert!(false)
    }
}
    