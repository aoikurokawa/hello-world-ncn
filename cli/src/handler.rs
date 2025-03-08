use std::str::FromStr;

use anyhow::anyhow;
use hello_world_ncn_client::instructions::{InitializeBallotBoxBuilder, InitializeConfigBuilder};
use hello_world_ncn_core::{ballot_box::BallotBox, config::Config};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};

use crate::args::{Args, ProgramCommand};

pub struct CliHandler {
    pub rpc_url: String,

    pub commitment: CommitmentConfig,

    keypair: Option<Keypair>,

    pub hello_world_ncn_program_id: Pubkey,

    pub restaking_program_id: Pubkey,

    ncn: Option<Pubkey>,

    rpc_client: RpcClient,
}

impl CliHandler {
    pub async fn from_args(args: &Args) -> anyhow::Result<Self> {
        let rpc_url = args.rpc_url.clone();
        CommitmentConfig::confirmed();
        let commitment = CommitmentConfig::from_str(&args.commitment)?;

        let keypair = args
            .keypair_path
            .as_ref()
            .map(|k| read_keypair_file(k).unwrap());

        let restaking_program_id = Pubkey::from_str(&args.restaking_program_id)?;

        let hello_world_ncn_program_id = Pubkey::from_str(&args.hello_world_ncn_program_id)?;

        let ncn = args
            .ncn
            .clone()
            .map(|id| Pubkey::from_str(&id))
            .transpose()?;

        let rpc_client = RpcClient::new_with_commitment(rpc_url.clone(), commitment);

        let handler = Self {
            rpc_url,
            commitment,
            keypair,
            restaking_program_id,
            hello_world_ncn_program_id,
            ncn,
            rpc_client,
        };

        Ok(handler)
    }

    pub const fn rpc_client(&self) -> &RpcClient {
        &self.rpc_client
    }

    pub fn keypair(&self) -> anyhow::Result<&Keypair> {
        self.keypair.as_ref().ok_or_else(|| anyhow!("No keypair"))
    }

    pub fn ncn(&self) -> anyhow::Result<&Pubkey> {
        self.ncn.as_ref().ok_or_else(|| anyhow!("No NCN address"))
    }

    pub async fn handle(&self, action: ProgramCommand) -> anyhow::Result<()> {
        match action {
            ProgramCommand::InitializeConfig => {
                let ncn_pubkey = self.ncn()?;
                let ncn_admin = self.keypair()?;
                let ncn_admin_pubkey = ncn_admin.pubkey();

                let config_info =
                    Config::find_program_address(&self.hello_world_ncn_program_id, ncn_pubkey).0;

                let mut initialize_config_ix = InitializeConfigBuilder::new()
                    .config_info(config_info)
                    .ncn_info(*ncn_pubkey)
                    .ncn_admin_info(ncn_admin_pubkey)
                    .min_stake(100)
                    .instruction();
                initialize_config_ix.program_id = self.hello_world_ncn_program_id;

                let recent_blockhash = self.rpc_client.get_latest_blockhash().await?;
                let tx = Transaction::new_signed_with_payer(
                    &[initialize_config_ix],
                    Some(&ncn_admin_pubkey),
                    &[ncn_admin],
                    recent_blockhash,
                );

                let result = self
                    .rpc_client
                    .send_and_confirm_transaction(&tx)
                    .await
                    .unwrap();

                println!("Result: {}", result);
            }
            ProgramCommand::InitializeBallotBox => {
                let ncn_pubkey = self.ncn()?;
                let ncn_admin = self.keypair()?;
                let ncn_admin_pubkey = ncn_admin.pubkey();

                let epoch_info = self.rpc_client.get_epoch_info().await?;
                let epoch = epoch_info.epoch;

                let config_info =
                    Config::find_program_address(&self.hello_world_ncn_program_id, ncn_pubkey).0;
                let ballot_box_info = BallotBox::find_program_address(
                    &self.hello_world_ncn_program_id,
                    ncn_pubkey,
                    epoch,
                )
                .0;

                let mut initialize_config_ix = InitializeBallotBoxBuilder::new()
                    .config_info(config_info)
                    .ncn_info(*ncn_pubkey)
                    .ballot_box_info(ballot_box_info)
                    .ncn_admin_info(ncn_admin_pubkey)
                    .instruction();
                initialize_config_ix.program_id = self.hello_world_ncn_program_id;

                let recent_blockhash = self.rpc_client.get_latest_blockhash().await?;
                let tx = Transaction::new_signed_with_payer(
                    &[initialize_config_ix],
                    Some(&ncn_admin_pubkey),
                    &[ncn_admin],
                    recent_blockhash,
                );

                let result = self
                    .rpc_client
                    .send_and_confirm_transaction(&tx)
                    .await
                    .unwrap();

                println!("Result: {}", result);
            }
            ProgramCommand::RequestMessage => {}
        }

        Ok(())
    }
}
