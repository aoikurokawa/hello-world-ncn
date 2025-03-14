use hello_world_ncn_core::{ballot_box::BallotBox, config::Config as NcnConfig, message::Message};
use jito_bytemuck::AccountDeserialize;
use jito_restaking_core::{
    ncn_operator_state::NcnOperatorState, ncn_vault_ticket::NcnVaultTicket,
    operator_vault_ticket::OperatorVaultTicket,
};
use jito_vault_core::{
    vault_ncn_ticket::VaultNcnTicket, vault_operator_delegation::VaultOperatorDelegation,
};
use solana_program_test::BanksClient;
use solana_sdk::{
    commitment_config::CommitmentLevel, native_token::sol_to_lamports, pubkey::Pubkey,
    signature::Keypair, signer::Signer, system_instruction::transfer, transaction::Transaction,
};

use super::{restaking_client::OperatorRoot, vault_client::VaultRoot, TestResult};

pub struct HelloWorldNcnClient {
    banks_client: BanksClient,
    payer: Keypair,
}

impl HelloWorldNcnClient {
    pub fn new(banks_client: BanksClient, payer: Keypair) -> Self {
        Self {
            banks_client,
            payer,
        }
    }

    pub async fn process_transaction(&mut self, tx: &Transaction) -> TestResult<()> {
        self.banks_client
            .process_transaction_with_preflight_and_commitment(
                tx.clone(),
                CommitmentLevel::Processed,
            )
            .await?;
        Ok(())
    }

    pub async fn airdrop(&mut self, to: &Pubkey, sol: f64) -> TestResult<()> {
        // let blockhash = self.banks_client.get_latest_blockhash().await?;
        let new_blockhash = self.banks_client.get_latest_blockhash().await.unwrap();
        self.banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &[transfer(&self.payer.pubkey(), to, sol_to_lamports(sol))],
                    Some(&self.payer.pubkey()),
                    &[&self.payer],
                    new_blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await?;
        Ok(())
    }

    pub async fn get_ncn_config(&mut self, account: &Pubkey) -> TestResult<NcnConfig> {
        let account = self.banks_client.get_account(*account).await?.unwrap();
        Ok(*NcnConfig::try_from_slice_unchecked(account.data.as_slice()).unwrap())
    }

    pub async fn get_message(&mut self, account: &Pubkey) -> TestResult<Message> {
        let account = self.banks_client.get_account(*account).await?.unwrap();
        Ok(*Message::try_from_slice_unchecked(account.data.as_slice()).unwrap())
    }

    pub async fn get_ballot_box(&mut self, account: &Pubkey) -> TestResult<BallotBox> {
        let account = self.banks_client.get_account(*account).await?.unwrap();
        Ok(*BallotBox::try_from_slice_unchecked(account.data.as_slice()).unwrap())
    }

    pub async fn do_initialize_config(
        &mut self,
        ncn: &Pubkey,
        ncn_admin: &Keypair,
        min_stake: u64,
    ) -> TestResult<()> {
        // Setup Payer
        self.airdrop(&self.payer.pubkey(), 1.0).await?;

        self.initialize_config(ncn, ncn_admin, min_stake).await
    }

    pub async fn initialize_config(
        &mut self,
        ncn: &Pubkey,
        ncn_admin: &Keypair,
        min_stake: u64,
    ) -> TestResult<()> {
        let config = hello_world_ncn_core::config::Config::find_program_address(
            &hello_world_ncn_program::id(),
            ncn,
        )
        .0;

        let ix = hello_world_ncn_sdk::sdk::initialize_config(
            &hello_world_ncn_program::id(),
            &config,
            ncn,
            &ncn_admin.pubkey(),
            min_stake,
        );

        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.process_transaction(&Transaction::new_signed_with_payer(
            &[ix],
            Some(&ncn_admin.pubkey()),
            &[&ncn_admin],
            blockhash,
        ))
        .await
    }

    pub async fn do_initialize_ballot_box(
        &mut self,
        ncn: &Pubkey,
        ncn_admin: &Keypair,
        epoch: u64,
    ) -> TestResult<()> {
        // Setup Payer
        self.airdrop(&self.payer.pubkey(), 1.0).await?;

        self.initialize_ballot_box(ncn, ncn_admin, epoch).await
    }

    pub async fn initialize_ballot_box(
        &mut self,
        ncn: &Pubkey,
        ncn_admin: &Keypair,
        epoch: u64,
    ) -> TestResult<()> {
        let config = hello_world_ncn_core::config::Config::find_program_address(
            &hello_world_ncn_program::id(),
            ncn,
        )
        .0;
        let ballot_box = hello_world_ncn_core::ballot_box::BallotBox::find_program_address(
            &hello_world_ncn_program::id(),
            ncn,
            epoch,
        )
        .0;

        let ix = hello_world_ncn_sdk::sdk::initialize_ballot_box(
            &hello_world_ncn_program::id(),
            &config,
            ncn,
            &ballot_box,
            &ncn_admin.pubkey(),
        );

        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.process_transaction(&Transaction::new_signed_with_payer(
            &[ix],
            Some(&ncn_admin.pubkey()),
            &[&ncn_admin],
            blockhash,
        ))
        .await
    }

    pub async fn do_request_message(
        &mut self,
        ncn: &Pubkey,
        ncn_admin: &Keypair,
        epoch: u64,
        keyword: String,
    ) -> TestResult<()> {
        // Setup Payer
        self.airdrop(&self.payer.pubkey(), 1.0).await?;

        self.request_message(ncn, ncn_admin, epoch, keyword).await
    }

    pub async fn request_message(
        &mut self,
        ncn: &Pubkey,
        ncn_admin: &Keypair,
        epoch: u64,
        keyword: String,
    ) -> TestResult<()> {
        let ncn_config = hello_world_ncn_core::config::Config::find_program_address(
            &hello_world_ncn_program::id(),
            ncn,
        )
        .0;
        let message = hello_world_ncn_core::message::Message::find_program_address(
            &hello_world_ncn_program::id(),
            *ncn,
            epoch,
        )
        .0;

        let ix = hello_world_ncn_sdk::sdk::request_message(
            &hello_world_ncn_program::id(),
            &ncn_config,
            &ncn,
            &message,
            &ncn_admin.pubkey(),
            keyword,
        );

        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.process_transaction(&Transaction::new_signed_with_payer(
            &[ix],
            Some(&ncn_admin.pubkey()),
            &[&ncn_admin],
            blockhash,
        ))
        .await
    }

    pub async fn do_submit_message(
        &mut self,
        ncn: &Pubkey,
        operator_root: &OperatorRoot,
        vault_root: &VaultRoot,
        epoch: u64,
        message_data: String,
    ) -> TestResult<()> {
        // Setup Payer
        self.airdrop(&self.payer.pubkey(), 1.0).await?;

        self.submit_message(ncn, operator_root, vault_root, epoch, message_data)
            .await
    }

    pub async fn submit_message(
        &mut self,
        ncn: &Pubkey,
        operator_root: &OperatorRoot,
        vault_root: &VaultRoot,
        epoch: u64,
        message_data: String,
    ) -> TestResult<()> {
        let ncn_config = hello_world_ncn_core::config::Config::find_program_address(
            &hello_world_ncn_program::id(),
            ncn,
        )
        .0;
        let restaking_config_info = jito_restaking_core::config::Config::find_program_address(
            &jito_restaking_program::id(),
        )
        .0;
        let vault_ncn_ticket_info = VaultNcnTicket::find_program_address(
            &jito_vault_program::id(),
            &vault_root.vault_pubkey,
            ncn,
        )
        .0;
        let ncn_vault_ticket_info = NcnVaultTicket::find_program_address(
            &jito_restaking_program::id(),
            ncn,
            &vault_root.vault_pubkey,
        )
        .0;
        let ncn_operator_state_info = NcnOperatorState::find_program_address(
            &jito_restaking_program::id(),
            ncn,
            &operator_root.operator_pubkey,
        )
        .0;
        let vault_operator_delegation_info = VaultOperatorDelegation::find_program_address(
            &jito_vault_program::id(),
            &vault_root.vault_pubkey,
            &operator_root.operator_pubkey,
        )
        .0;
        let operator_vault_ticket_info = OperatorVaultTicket::find_program_address(
            &jito_restaking_program::id(),
            &operator_root.operator_pubkey,
            &vault_root.vault_pubkey,
        )
        .0;
        let message = hello_world_ncn_core::message::Message::find_program_address(
            &hello_world_ncn_program::id(),
            *ncn,
            epoch,
        )
        .0;
        let ballot_box = hello_world_ncn_core::ballot_box::BallotBox::find_program_address(
            &hello_world_ncn_program::id(),
            ncn,
            epoch,
        )
        .0;

        let ix = hello_world_ncn_sdk::sdk::submit_message(
            &hello_world_ncn_program::id(),
            &ncn_config,
            &restaking_config_info,
            &ncn,
            &operator_root.operator_pubkey,
            &vault_root.vault_pubkey,
            &vault_ncn_ticket_info,
            &ncn_vault_ticket_info,
            &ncn_operator_state_info,
            &vault_operator_delegation_info,
            &operator_vault_ticket_info,
            &message,
            &ballot_box,
            &operator_root.operator_admin.pubkey(),
            message_data,
        );

        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.process_transaction(&Transaction::new_signed_with_payer(
            &[ix],
            Some(&operator_root.operator_admin.pubkey()),
            &[&operator_root.operator_admin],
            blockhash,
        ))
        .await
    }
}
