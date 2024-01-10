mod tests {
    use crate::{contract, types::AccountData};

    /// Imports all the definitions from the outer scope so we can use them here.
    use contract::account_registry::Contract;

    /// We test if the default constructor does its job.
    #[ink::test]
    fn default_works() {
        let contract =  Contract::default();
        assert_eq!(contract.get_account(), None);
    }

    /// We test a simple use case of our contract.
    #[ink::test]
    fn set_account_works() {
        let mut contract =  Contract::default();

        let empty : AccountData = AccountData::default();

        contract.create_account(empty.clone());
        assert_eq!(contract.get_account(), Some(empty));

    }
}
    