use std::str::FromStr;

use hello_world_ncn_client::{accounts::Message, instructions::SubmitMessageBuilder};
use jito_restaking_core::{
    ncn_operator_state::NcnOperatorState, ncn_vault_ticket::NcnVaultTicket,
    operator_vault_ticket::OperatorVaultTicket,
};
use jito_vault_core::{
    vault_ncn_ticket::VaultNcnTicket, vault_operator_delegation::VaultOperatorDelegation,
};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair, signer::Signer,
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

        let message_account = rpc_client.get_account(&message_pubkey).await.unwrap();
        let message = Message::from_bytes(&message_account.data)?;

        Ok(message)
    }

    pub async fn submit_message(
        &self,
        ncn: &Pubkey,
        operator: &Pubkey,
        vault: &Pubkey,
        epoch: u64,
        message_data: String,
    ) -> anyhow::Result<()> {
        let rpc_client = self.get_rpc_client();

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
