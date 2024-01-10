mod e2e_tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use super::*;

    /// A helper function used for calling contract messages.
    use ink_e2e::build_message;

    /// The End-to-End test `Result` type.
    type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    /// We test that we can upload and instantiate the contract using its default constructor.
    #[ink_e2e::test]
    async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        // Given
        let constructor = FlipperRef::default();

        // When
        let contract_account_id = client
            .instantiate("flipper", &ink_e2e::alice(), constructor, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;

        // Then
        let get = build_message::<FlipperRef>(contract_account_id.clone())
            .call(|flipper| flipper.get());
        let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
        assert!(matches!(get_result.return_value(), false));

        Ok(())
    }

    /// We test that we can read and write a value from the on-chain contract contract.
    #[ink_e2e::test]
    async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        // Given
        let constructor = FlipperRef::new(false);
        let contract_account_id = client
            .instantiate("flipper", &ink_e2e::bob(), constructor, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;

        let get = build_message::<FlipperRef>(contract_account_id.clone())
            .call(|flipper| flipper.get());
        let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
        assert!(matches!(get_result.return_value(), false));

        // When
        let flip = build_message::<FlipperRef>(contract_account_id.clone())
            .call(|flipper| flipper.flip());
        let _flip_result = client
            .call(&ink_e2e::bob(), flip, 0, None)
            .await
            .expect("flip failed");

        // Then
        let get = build_message::<FlipperRef>(contract_account_id.clone())
            .call(|flipper| flipper.get());
        let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
        assert!(matches!(get_result.return_value(), true));

        Ok(())
    }
}