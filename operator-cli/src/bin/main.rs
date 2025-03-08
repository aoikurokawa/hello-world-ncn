use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::{anyhow, Context};
use clap::Parser;
use dotenv::dotenv;
use log::{error, info};
use operator_cli::handler::Handler;
use serde::{Deserialize, Serialize};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file, signer::Signer};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct OperatorConfig {
    name: String,
    keypair_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    rpc_url: String,
    program_id: String,
    ncn_pubkey: String,
    crank_interval: u64,
    metrics_interval: u64,
    priority_fees: u64,
    operators: Vec<OperatorConfig>,
}

#[derive(Parser)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long)]
    config_path: PathBuf,

    /// Override RPC URL from config
    #[arg(short, long)]
    rpc_url: Option<String>,
}

// impl std::fmt::Display for Args {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "Operator Configuration:\n\
//             -------------------------------\n\
//             RPC URL: {}\n\
//             Keypair Path: {:?}\n\
//             Program ID: {}\n\
//             Crank Interval: {} seconds\n\
//             Metrics Interval: {} seconds\n\
//             Priority Fees: {} microlamports\n\
//             -------------------------------",
//             self.rpc_url,
//             self.keypair_path,
//             self.program_id,
//             self.crank_interval,
//             self.metrics_interval,
//             self.priority_fees,
//         )
//     }
// }

fn load_config<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    let config_str = std::fs::read_to_string(path).context("Failed to read configuration file")?;

    let config: Config =
        serde_json::from_str(&config_str).context("Failed to parse configuration file")?;

    Ok(config)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Args::parse();

    let mut config = load_config(&args.config_path)?;

    if let Some(rpc_url) = args.rpc_url {
        config.rpc_url = rpc_url;
    }

    let program_id = config
        .program_id
        .parse::<Pubkey>()
        .context("Invalid program ID in config")?;

    let ncn_pubkey = config
        .ncn_pubkey
        .parse::<Pubkey>()
        .context("Invalid NCN pubkey in config")?;

    if config.operators.is_empty() {
        return Err(anyhow!("No operators defined in configuration file"));
    }

    let mut handles = vec![];

    for operator_config in config.operators {
        let rpc_url = config.rpc_url.clone();
        let program_id_clone = program_id;
        let ncn_pubkey_clone = ncn_pubkey;
        let crank_interval = config.crank_interval;
        let priority_fees = config.priority_fees;
        let operator_name = operator_config.name.clone();

        // Spawn a new task for this operator
        let handle = tokio::spawn(async move {
            if let Err(e) = run_operator(
                operator_config,
                rpc_url,
                program_id_clone,
                ncn_pubkey_clone,
                crank_interval,
                priority_fees,
            )
            .await
            {
                log::error!("Operator '{}' failed: {}", operator_name, e);
            }
        });

        handles.push(handle);
    }

    // Wait for all operators to complete (which should be never in this case)
    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}

async fn run_operator(
    operator_config: OperatorConfig,
    rpc_url: String,
    program_id: Pubkey,
    ncn_pubkey: Pubkey,
    crank_interval: u64,
    priority_fees: u64,
) -> anyhow::Result<()> {
    info!(
        "Starting operator '{}' with keypair at: {}",
        operator_config.name, operator_config.keypair_path
    );

    // Read the operator's keypair
    let operator_keypair = read_keypair_file(&operator_config.keypair_path).map_err(|e| {
        anyhow!(
            "Failed to read keypair file for {}: {}",
            operator_config.name,
            e
        )
    })?;

    let operator_pubkey = operator_keypair.pubkey();
    info!(
        "Operator '{}' pubkey: {}",
        operator_config.name, operator_pubkey
    );

    let rpc_client = RpcClient::new_with_timeout(rpc_url.clone(), Duration::from_secs(60));
    let handler = Handler::new(&rpc_url, &operator_keypair, program_id, priority_fees);

    loop {
        let slot = match rpc_client.get_slot().await {
            Ok(slot) => slot,
            Err(e) => {
                log::error!(
                    "Operator '{}': Failed to get slot: {}",
                    operator_config.name,
                    e
                );
                tokio::time::sleep(Duration::from_secs(10)).await;
                continue;
            }
        };

        let epoch = match rpc_client.get_epoch_info().await {
            Ok(info) => info.epoch,
            Err(e) => {
                log::error!(
                    "Operator '{}': Failed to get epoch info: {}",
                    operator_config.name,
                    e
                );
                tokio::time::sleep(Duration::from_secs(10)).await;
                continue;
            }
        };

        info!(
            "Operator '{}': Checking for message. Slot: {}, Current Epoch: {}",
            operator_config.name, slot, epoch
        );

        match handler.get_message(epoch).await {
            Ok(msg) => {
                let keyword = String::from_utf8_lossy(&msg.keyword[..msg.keyword_len as usize]);
                info!(
                    "Operator '{}': Got Message: {}",
                    operator_config.name, keyword
                );

                let message = format!("{}, World from Operator {}", keyword, operator_config.name);
                match handler
                    .submit_message(&ncn_pubkey, &operator_pubkey, epoch, message)
                    .await
                {
                    Ok(_) => info!(
                        "Operator '{}': Successfully submitted message",
                        operator_config.name
                    ),
                    Err(e) => error!(
                        "Operator '{}': Failed to submit message: {}",
                        operator_config.name, e
                    ),
                }
            }
            Err(e) => {
                // Only log as error if it's not the expected "no message" case
                if !e.to_string().contains("No message found") {
                    error!(
                        "Operator '{}': Error getting message: {}",
                        operator_config.name, e
                    );
                }
            }
        }

        info!(
            "Operator '{}': Sleeping for {} seconds",
            operator_config.name, crank_interval
        );
        tokio::time::sleep(Duration::from_secs(crank_interval)).await;
    }
}
