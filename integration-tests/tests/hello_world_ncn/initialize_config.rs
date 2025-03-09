#[cfg(test)]
mod tests {
    use hello_world_ncn_core::config::Config;

    use crate::fixtures::test_builder::TestBuilder;

    #[tokio::test]
    async fn test_initialize_ncn_config_ok() {
        let mut fixture = TestBuilder::new().await;
        let mut hello_world_ncn_client = fixture.hello_world_ncn_client();
        let ncn_root = fixture.setup_ncn().await.unwrap();

        let min_stake = 100;

        hello_world_ncn_client
            .do_initialize_config(&ncn_root.ncn_pubkey, &ncn_root.ncn_admin, min_stake)
            .await
            .unwrap();

        let config_pubkey =
            Config::find_program_address(&hello_world_ncn_program::id(), &ncn_root.ncn_pubkey).0;
        let config = hello_world_ncn_client
            .get_ncn_config(&config_pubkey)
            .await
            .unwrap();
        assert_eq!(config.ncn, ncn_root.ncn_pubkey);
    }
}
