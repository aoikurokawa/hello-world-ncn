#[cfg(test)]
mod tests {
    use crate::fixtures::test_builder::TestBuilder;

    #[tokio::test]
    async fn test_initialize_ballot_box_ok() {
        let mut fixture = TestBuilder::new().await;
        let mut hello_world_ncn_client = fixture.hello_world_ncn_client();
        let mut restaking_program_client = fixture.restaking_program_client();
        let ncn_root = fixture.setup_ncn().await.unwrap();

        let config = jito_restaking_core::config::Config::find_program_address(
            &jito_restaking_program::id(),
        )
        .0;
        let restaking_config = restaking_program_client.get_config(&config).await.unwrap();
        let slot = fixture.get_current_slot().await.unwrap();
        let epoch = slot / restaking_config.epoch_length();
        let min_stake = 100;

        hello_world_ncn_client
            .do_initialize_config(&ncn_root.ncn_pubkey, &ncn_root.ncn_admin, min_stake)
            .await
            .unwrap();

        hello_world_ncn_client
            .do_initialize_ballot_box(&ncn_root.ncn_pubkey, &ncn_root.ncn_admin, epoch)
            .await
            .unwrap();
    }
}
