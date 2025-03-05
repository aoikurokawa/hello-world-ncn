#[cfg(test)]
mod tests {
    use crate::fixtures::test_builder::TestBuilder;

    #[tokio::test]
    async fn test_initialize_ncn_config_ok() {
        let mut fixture = TestBuilder::new().await;
        let mut hello_world_ncn_client = fixture.hello_world_ncn_client();
        let ncn_root = fixture.setup_ncn().await.unwrap();
        hello_world_ncn_client
            .do_initialize_config(ncn_root.ncn_pubkey, &ncn_root.ncn_admin)
            .await
            .unwrap();
    }
}
