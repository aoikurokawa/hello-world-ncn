#[cfg(test)]
mod tests {
    use ncn_portal_core::whitelist::Whitelist;
    use solana_sdk::signature::Signer;

    use crate::fixtures::fixture::TestBuilder;

    #[tokio::test]
    async fn test_initialize_whitelist_ok() {
        let fixture = TestBuilder::new().await;

        let mut ncn_portal_program_client = fixture.ncn_portal_program_client();

        let whitelist_pubkey = Whitelist::find_program_address(&ncn_portal_program::id()).0;

        let root = [0u8; 32];

        let admin = ncn_portal_program_client
            .do_initialize_whitelist(&root)
            .await
            .unwrap();

        let whitelist = ncn_portal_program_client
            .get_whitelist(&whitelist_pubkey)
            .await
            .unwrap();

        assert_eq!(whitelist.admin, admin.pubkey());
    }
}
