use solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair};

pub mod cli_args;
pub mod log;
pub mod ncn_portal;
pub mod ncn_portal_handler;

pub struct CliConfig {
    pub rpc_url: String,

    pub commitment: CommitmentConfig,

    pub keypair: Option<Keypair>,
}
