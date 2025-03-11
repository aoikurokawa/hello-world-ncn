use std::str::FromStr;

use anyhow::Context;
use hello_world_ncn_client::{
    accounts::{BallotBox, Message},
    instructions::SubmitMessageBuilder,
};
use jito_restaking_core::{
    ncn_operator_state::NcnOperatorState, ncn_vault_ticket::NcnVaultTicket,
    operator_vault_ticket::OperatorVaultTicket,
};
use jito_vault_core::{
    vault_ncn_ticket::VaultNcnTicket, vault_operator_delegation::VaultOperatorDelegation,
};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};

#[allow(dead_code)]
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

    pub fn get_config_pubkey(&self, ncn: &Pubkey) -> Pubkey {
        let seeds = [b"config".to_vec(), ncn.to_bytes().to_vec()];
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();

        Pubkey::find_program_address(
            &seeds_iter,
            &Pubkey::from_str("ncncd27gXkYMV56EfwntDmYhH5Wzo896yTnrBbEq9xW").unwrap(),
        )
        .0
    }

    pub fn get_message_pubkey(&self, ncn: Pubkey, epoch: u64) -> Pubkey {
        let seeds = [
            b"message".to_vec(),
            ncn.to_bytes().to_vec(),
            epoch.to_be_bytes().to_vec(),
        ];
        // let seeds = [b"message".to_vec(), epoch.to_be_bytes().to_vec()];
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();

        Pubkey::find_program_address(
            &seeds_iter,
            &Pubkey::from_str("ncncd27gXkYMV56EfwntDmYhH5Wzo896yTnrBbEq9xW").unwrap(),
        )
        .0
    }

    pub fn get_ballot_box_pubkey(&self, ncn: &Pubkey, epoch: u64) -> Pubkey {
        let seeds = Vec::from_iter([
            b"ballot_box".to_vec(),
            ncn.to_bytes().to_vec(),
            epoch.to_le_bytes().to_vec(),
        ]);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();

        Pubkey::find_program_address(
            &seeds_iter,
            &Pubkey::from_str("ncncd27gXkYMV56EfwntDmYhH5Wzo896yTnrBbEq9xW").unwrap(),
        )
        .0
    }

    pub async fn get_message(&self, ncn: Pubkey, epoch: u64) -> anyhow::Result<Message> {
        let rpc_client = self.get_rpc_client();

        let message_pubkey = self.get_message_pubkey(ncn, epoch);

        let message_account = rpc_client.get_account(&message_pubkey).await?;
        let message = Message::from_bytes(&message_account.data)?;

        Ok(message)
    }

    pub async fn get_ballot_box(&self, ncn: Pubkey, epoch: u64) -> anyhow::Result<BallotBox> {
        let rpc_client = self.get_rpc_client();

        let ballot_box_pubkey = self.get_ballot_box_pubkey(&ncn, epoch);

        let ballot_box_account = rpc_client.get_account(&ballot_box_pubkey).await?;
        let ballot_box = BallotBox::from_bytes(&ballot_box_account.data)?;

        Ok(ballot_box)
    }

    pub async fn submit_message(
        &self,
        ncn: &Pubkey,
        operator: &Pubkey,
        vault: &Pubkey,
        epoch: u64,
        message_data: String,
    ) -> anyhow::Result<Option<Signature>> {
        let rpc_client = self.get_rpc_client();

        let ballot_box = self.get_ballot_box(*ncn, epoch).await?;
        for vote in ballot_box.operator_votes.iter() {
            if vote.operator.eq(operator) {
                let message =
                    String::from_utf8(vote.message_data[..vote.message_len as usize].to_vec())?;
                log::info!("You have submitted message already!: {}", message);
                return Ok(None);
            }
        }

        let config_pubkey = self.get_config_pubkey(ncn);
        let message_pubkey = self.get_message_pubkey(*ncn, epoch);
        let ballot_box_pubkey = self.get_ballot_box_pubkey(ncn, epoch);

        let restaking_config_info = jito_restaking_core::config::Config::find_program_address(
            &jito_restaking_program::id(),
        )
        .0;
        let vault_ncn_ticket_info =
            VaultNcnTicket::find_program_address(&jito_vault_program::id(), vault, ncn).0;
        let ncn_vault_ticket_info =
            NcnVaultTicket::find_program_address(&jito_restaking_program::id(), ncn, vault).0;
        let ncn_operator_state_info =
            NcnOperatorState::find_program_address(&jito_restaking_program::id(), ncn, operator).0;
        let vault_operator_delegation_info = VaultOperatorDelegation::find_program_address(
            &jito_vault_program::id(),
            vault,
            operator,
        )
        .0;
        let operator_vault_ticket_info = OperatorVaultTicket::find_program_address(
            &jito_restaking_program::id(),
            operator,
            vault,
        )
        .0;

        // Do computation
        let hash_value = solana_program::hash::hash(message_data.as_bytes());

        let mut submit_message_ix_builder = SubmitMessageBuilder::new();
        submit_message_ix_builder
            .config_info(config_pubkey)
            .restaking_config_info(restaking_config_info)
            .ncn_info(*ncn)
            .operator_info(*operator)
            .vault_info(*vault)
            .vault_ncn_ticket_info(vault_ncn_ticket_info)
            .ncn_vault_ticket_info(ncn_vault_ticket_info)
            .ncn_operator_state_info(ncn_operator_state_info)
            .vault_operator_delegation_info(vault_operator_delegation_info)
            .operator_vault_ticket(operator_vault_ticket_info)
            .message_info(message_pubkey)
            .ballot_box_info(ballot_box_pubkey)
            .operator_voter_info(self.payer.pubkey())
            .message(hash_value.to_string());
        let mut submit_message_ix = submit_message_ix_builder.instruction();
        submit_message_ix.program_id = self.program_id;

        let blockhash = rpc_client
            .get_latest_blockhash()
            .await
            .context("Failed to get latest blockhash")?;
        let tx = Transaction::new_signed_with_payer(
            &[submit_message_ix],
            Some(&self.payer.pubkey()),
            &[self.payer],
            blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .context("Failed to send and confirm transaction")?;

        Ok(Some(signature))
    }
}
