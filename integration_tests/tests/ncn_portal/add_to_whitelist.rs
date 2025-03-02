#[cfg(test)]
mod tests {
    use ncn_portal_core::whitelist::Whitelist;
    use solana_sdk::pubkey::Pubkey;

    use crate::fixtures::fixture::TestBuilder;

    #[tokio::test]
    async fn test_admin_set_new_admin_ok() {
        let fixture = TestBuilder::new().await;

        let mut ncn_portal_program_client = fixture.ncn_portal_program_client();

        let root = [0u8; 32];

        let admin = ncn_portal_program_client
            .do_initialize_whitelist(&root)
            .await
            .unwrap();

        let new_admin = Pubkey::new_unique();

        ncn_portal_program_client
            .do_admin_set_new_admin(&admin, &new_admin)
            .await
            .unwrap();

        let whitelist_pubkey = Whitelist::find_program_address(&ncn_portal_program::id()).0;

        let whitelist = ncn_portal_program_client
            .get_whitelist(&whitelist_pubkey)
            .await
            .unwrap();

        assert_eq!(whitelist.admin, new_admin);
    }
}
