use solana_program_test::BanksClient;
use solana_sdk::{
    commitment_config::CommitmentLevel, native_token::sol_to_lamports, pubkey::Pubkey,
    signature::Keypair, signer::Signer, system_instruction::transfer, transaction::Transaction,
};

use super::{restaking_client::NcnRoot, TestResult};

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

    pub async fn airdrop_lamports(&mut self, to: &Pubkey, lamports: u64) -> TestResult<()> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &[transfer(&self.payer.pubkey(), to, lamports)],
                    Some(&self.payer.pubkey()),
                    &[&self.payer],
                    blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await?;
        Ok(())
    }

    pub async fn setup_hello_world_ncn(&mut self, ncn_root: &NcnRoot) -> TestResult<()> {
        Ok(())
    }

    pub async fn do_initialize_config(
        &mut self,
        ncn: Pubkey,
        ncn_admin: &Keypair,
    ) -> TestResult<()> {
        // Setup Payer
        self.airdrop(&self.payer.pubkey(), 1.0).await?;

        // Setup account payer
        // let (account_payer, _, _) =
        //     AccountPayer::find_program_address(&jito_tip_router_program::id(), &ncn);
        // self.airdrop(&account_payer, 100.0).await?;

        let ncn_admin_pubkey = ncn_admin.pubkey();
        self.initialize_config(ncn, ncn_admin).await
    }

    pub async fn initialize_config(&mut self, ncn: Pubkey, ncn_admin: &Keypair) -> TestResult<()> {
        let config = hello_world_ncn_core::config::Config::find_program_address(
            &hello_world_ncn_program::id(),
        )
        .0;

        // let (account_payer, _, _) =
        //     AccountPayer::find_program_address(&jito_tip_router_program::id(), &ncn);

        let ix = hello_world_ncn_sdk::sdk::initialize_config(
            &hello_world_ncn_program::id(),
            &config,
            &ncn_admin.pubkey(),
        );

        // let ix = InitializeConfigBuilder::new()
        //     .config(config)
        //     .ncn(ncn)
        //     .ncn_admin(ncn_admin.pubkey())
        //     .account_payer(account_payer)
        //     .fee_wallet(*fee_wallet)
        //     .tie_breaker_admin(*tie_breaker_admin)
        //     .dao_fee_bps(dao_fee_bps)
        //     .default_ncn_fee_bps(default_ncn_fee_bps)
        //     .block_engine_fee_bps(block_engine_fee_bps)
        //     .epochs_before_stall(epochs_before_stall)
        //     .epochs_after_consensus_before_close(epochs_after_consensus_before_close)
        //     .valid_slots_after_consensus(valid_slots_after_consensus)
        //     .instruction();

        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.process_transaction(&Transaction::new_signed_with_payer(
            &[ix],
            Some(&ncn_admin.pubkey()),
            &[&ncn_admin],
            blockhash,
        ))
        .await
    }
}
