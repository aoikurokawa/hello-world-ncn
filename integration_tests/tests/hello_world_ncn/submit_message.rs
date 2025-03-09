#[cfg(test)]
mod tests {
    use hello_world_ncn_core::ballot_box::BallotBox;

    use crate::fixtures::test_builder::TestBuilder;

    #[tokio::test]
    async fn test_submit_message_ok() {
        let mut fixture = TestBuilder::new().await;
        let mut restaking_program_client = fixture.restaking_program_client();
        let mut hello_world_ncn_client = fixture.hello_world_ncn_client();

        let mut test_ncn = fixture.create_test_ncn().await.unwrap();
        fixture
            .add_operators_to_test_ncn(&mut test_ncn, 3, None)
            .await
            .unwrap();
        fixture
            .add_vaults_to_test_ncn(&mut test_ncn, 1, None)
            .await
            .unwrap();
        fixture
            .add_delegation_in_test_ncn(&mut test_ncn, 100)
            .await
            .unwrap();

        let message_data = "Hello,";

        let config = jito_restaking_core::config::Config::find_program_address(
            &jito_restaking_program::id(),
        )
        .0;
        let restaking_config = restaking_program_client.get_config(&config).await.unwrap();
        let slot = fixture.get_current_slot().await.unwrap();
        let epoch = slot / restaking_config.epoch_length();

        let min_stake = 100;

        hello_world_ncn_client
            .do_initialize_config(
                &test_ncn.ncn_root.ncn_pubkey,
                &test_ncn.ncn_root.ncn_admin,
                min_stake,
            )
            .await
            .unwrap();
        hello_world_ncn_client
            .do_initialize_ballot_box(
                &test_ncn.ncn_root.ncn_pubkey,
                &test_ncn.ncn_root.ncn_admin,
                epoch,
            )
            .await
            .unwrap();

        hello_world_ncn_client
            .do_request_message(
                &test_ncn.ncn_root.ncn_pubkey,
                &test_ncn.ncn_root.ncn_admin,
                epoch,
                message_data.to_string(),
            )
            .await
            .unwrap();

        fixture
            .warp_slot_incremental(2 * restaking_config.epoch_length())
            .await
            .unwrap();

        let vault = &test_ncn.vaults[0];

        // Operator 1
        let message_data = format!("{message_data} World");

        hello_world_ncn_client
            .do_submit_message(
                &test_ncn.ncn_root.ncn_pubkey,
                &test_ncn.operators[0],
                vault,
                epoch,
                message_data.clone(),
            )
            .await
            .unwrap();

        let ballot_box_pubkey = BallotBox::find_program_address(
            &hello_world_ncn_program::id(),
            &test_ncn.ncn_root.ncn_pubkey,
            epoch,
        )
        .0;
        let ballot_box = hello_world_ncn_client
            .get_ballot_box(&ballot_box_pubkey)
            .await
            .unwrap();
        assert_eq!(ballot_box.operator_votes[0].message_data(), message_data);
        assert!(!ballot_box.is_consensus_reached());

        // Operator 2
        let message_data = format!("{message_data} World");

        hello_world_ncn_client
            .do_submit_message(
                &test_ncn.ncn_root.ncn_pubkey,
                &test_ncn.operators[1],
                vault,
                epoch,
                message_data.clone(),
            )
            .await
            .unwrap();

        let ballot_box_pubkey = BallotBox::find_program_address(
            &hello_world_ncn_program::id(),
            &test_ncn.ncn_root.ncn_pubkey,
            epoch,
        )
        .0;
        let ballot_box = hello_world_ncn_client
            .get_ballot_box(&ballot_box_pubkey)
            .await
            .unwrap();
        assert_eq!(ballot_box.operator_votes[1].message_data(), message_data);
        assert!(ballot_box.is_consensus_reached());
    }
}
