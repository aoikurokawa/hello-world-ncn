use std::str::FromStr;

use hello_world_ncn_client::{accounts::Message, instructions::SubmitMessageBuilder};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair, signer::Signer,
    transaction::Transaction,
};

pub struct Handler<'a> {
    rpc_url: String,
    payer: &'a Keypair,
    program_id: Pubkey,
    priority_fees: u64,
}

impl<'a> Handler<'a> {
    pub fn new(rpc_url: &str, payer: &'a Keypair, program_id: Pubkey, priority_fees: u64) -> Self {
        Self {
            rpc_url: rpc_url.to_string(),
            payer,
            program_id,
            priority_fees,
        }
    }

    fn get_rpc_client(&self) -> RpcClient {
        RpcClient::new_with_commitment(self.rpc_url.clone(), CommitmentConfig::confirmed())
    }

    pub fn get_config_pubkey(&self, ncn: &Pubkey, epoch: u64) -> Pubkey {
        let seeds = vec![b"config".to_vec(), ncn.to_bytes().to_vec()];
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let config_pubkey = Pubkey::find_program_address(
            &seeds_iter,
            &Pubkey::from_str("DXWJEC5JBUeNurpo7wPDUHGhDWnjkTzUiV3gp2D9y8zr").unwrap(),
        )
        .0;

        config_pubkey
    }

    pub fn get_message_pubkey(&self, epoch: u64) -> Pubkey {
        let seeds = vec![b"message".to_vec(), epoch.to_be_bytes().to_vec()];
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let message_pubkey = Pubkey::find_program_address(
            &seeds_iter,
            &Pubkey::from_str("DXWJEC5JBUeNurpo7wPDUHGhDWnjkTzUiV3gp2D9y8zr").unwrap(),
        )
        .0;

        message_pubkey
    }

    pub fn get_ballot_box_pubkey(&self, ncn: &Pubkey, epoch: u64) -> Pubkey {
        let seeds = Vec::from_iter([
            b"ballot_box".to_vec(),
            ncn.to_bytes().to_vec(),
            epoch.to_le_bytes().to_vec(),
        ]);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let ballot_box_pubkey = Pubkey::find_program_address(
            &seeds_iter,
            &Pubkey::from_str("DXWJEC5JBUeNurpo7wPDUHGhDWnjkTzUiV3gp2D9y8zr").unwrap(),
        )
        .0;

        ballot_box_pubkey
    }

    pub async fn get_message(&self, epoch: u64) -> anyhow::Result<Message> {
        let rpc_client = self.get_rpc_client();

        // let epoch = rpc_client.get_epoch_info().await?;
        let message_pubkey = self.get_message_pubkey(epoch);

        let message_account = rpc_client.get_account(&message_pubkey).await.unwrap();
        let message = Message::from_bytes(&message_account.data)?;

        Ok(message)
    }

    pub async fn submit_message(
        &self,
        ncn: &Pubkey,
        operator: &Pubkey,
        epoch: u64,
        message_data: String,
    ) -> anyhow::Result<()> {
        let rpc_client = self.get_rpc_client();

        let config_pubkey = self.get_config_pubkey(ncn, epoch);
        let message_pubkey = self.get_message_pubkey(epoch);
        let ballot_box_pubkey = self.get_ballot_box_pubkey(ncn, epoch);

        let mut submit_message_ix_builder = SubmitMessageBuilder::new();
        submit_message_ix_builder
            .config_info(config_pubkey)
            .ncn_info(*ncn)
            .operator_info(*operator)
            .message_info(message_pubkey)
            .ballot_box_info(ballot_box_pubkey)
            .operator_voter_info(self.payer.pubkey())
            .message(message_data);
        let mut submit_message_ix = submit_message_ix_builder.instruction();
        submit_message_ix.program_id = self.program_id;

        let blockhash = rpc_client.get_latest_blockhash().await.unwrap();
        let tx = Transaction::new_signed_with_payer(
            &[submit_message_ix],
            Some(&self.payer.pubkey()),
            &[self.payer],
            blockhash,
        );
        rpc_client.send_and_confirm_transaction(&tx).await.unwrap();

        Ok(())
    }
}
