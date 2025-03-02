use anyhow::{anyhow, Result};
use jito_bytemuck::AccountDeserialize;
use log::{debug, info};
use meta_merkle_tree::{meta_merkle_tree::MetaMerkleTree, tree_node::TreeNode};
use ncn_portal_core::whitelist::Whitelist;
use ncn_portal_sdk::sdk::{admin_set_new_admin, admin_update_merkle_tree, initialize_whitelist};
use serde::Deserialize;
use solana_program::pubkey::Pubkey;
use solana_rpc_client::{nonblocking::rpc_client::RpcClient, rpc_client::SerializableTransaction};
use solana_sdk::{signature::Signer, transaction::Transaction};

use crate::{
    ncn_portal::{NcnPortalCommands, WhitelistActions},
    CliConfig,
};

#[derive(Debug, Deserialize)]
struct NcnPortalResponse {
    status: bool,
    data: Option<Vec<Pubkey>>,
    message: String,
}

pub struct NcnPortalCliHandler {
    cli_config: CliConfig,
    ncn_portal_program_id: Pubkey,
}

impl NcnPortalCliHandler {
    pub const fn new(cli_config: CliConfig, ncn_portal_program_id: Pubkey) -> Self {
        Self {
            cli_config,
            ncn_portal_program_id,
        }
    }

    fn get_rpc_client(&self) -> RpcClient {
        RpcClient::new_with_commitment(self.cli_config.rpc_url.clone(), self.cli_config.commitment)
    }

    pub async fn handle(&self, action: NcnPortalCommands) -> Result<()> {
        match action {
            NcnPortalCommands::Whitelist {
                action: WhitelistActions::Initialize,
            } => self.initialize_whitelist().await,
            NcnPortalCommands::Whitelist {
                action: WhitelistActions::Get,
            } => self.get_whitelist().await,
            NcnPortalCommands::Whitelist {
                action: WhitelistActions::AdminUpdateMerkleRoot { url },
            } => self.admin_update_merkle_root(url).await,
            NcnPortalCommands::Whitelist {
                action: WhitelistActions::AdminSetNewAdmin { new_admin },
            } => self.admin_set_new_admin(new_admin).await,
        }
    }

    async fn initialize_whitelist(&self) -> Result<()> {
        let keypair = self
            .cli_config
            .keypair
            .as_ref()
            .ok_or_else(|| anyhow!("No keypair"))?;
        let rpc_client = self.get_rpc_client();

        let whitelist_address = Whitelist::find_program_address(&self.ncn_portal_program_id).0;

        let ix = initialize_whitelist(
            &self.ncn_portal_program_id,
            &whitelist_address,
            &keypair.pubkey(),
            [0u8; 32],
        );
        let blockhash = rpc_client.get_latest_blockhash().await?;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&keypair.pubkey()),
            &[keypair],
            blockhash,
        );
        info!(
            "Initializing whitelist transaction: {:?}",
            tx.get_signature()
        );
        rpc_client.send_and_confirm_transaction(&tx).await?;
        info!("Transaction confirmed: {:?}", tx.get_signature());
        Ok(())
    }

    async fn get_whitelist(&self) -> Result<()> {
        let rpc_client = self.get_rpc_client();

        let whitelist_address = Whitelist::find_program_address(&self.ncn_portal_program_id).0;
        debug!(
            "Reading the whitelist account at address: {}",
            whitelist_address
        );

        let account = rpc_client.get_account(&whitelist_address).await?;
        let whitelist = Whitelist::try_from_slice_unchecked(&account.data)?;
        info!(
            "Whitelist at address {}: {:?}",
            whitelist_address, whitelist
        );
        Ok(())
    }

    async fn admin_set_new_admin(&self, new_admin: Pubkey) -> Result<()> {
        let keypair = self
            .cli_config
            .keypair
            .as_ref()
            .ok_or_else(|| anyhow!("No keypair"))?;
        let rpc_client = self.get_rpc_client();

        let whitelist_address = Whitelist::find_program_address(&self.ncn_portal_program_id).0;

        let ix = admin_set_new_admin(
            &self.ncn_portal_program_id,
            &whitelist_address,
            &keypair.pubkey(),
            &new_admin,
        );

        let blockhash = rpc_client.get_latest_blockhash().await?;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&keypair.pubkey()),
            &[keypair],
            blockhash,
        );
        info!("Set new admin: {:?}", tx.get_signature());
        let result = rpc_client.send_and_confirm_transaction(&tx).await?;
        info!("Transaction confirmed: {:?}", result);
        let statuses = rpc_client
            .get_signature_statuses(&[*tx.get_signature()])
            .await?;

        let tx_status = statuses
            .value
            .first()
            .unwrap()
            .as_ref()
            .ok_or_else(|| anyhow!("No signature status"))?;
        info!("Transaction status: {:?}", tx_status);

        Ok(())
    }

    async fn admin_update_merkle_root(&self, url: String) -> Result<()> {
        let keypair = self
            .cli_config
            .keypair
            .as_ref()
            .ok_or_else(|| anyhow!("No keypair"))?;
        let rpc_client = self.get_rpc_client();

        let whitelist_address = Whitelist::find_program_address(&self.ncn_portal_program_id).0;

        let res = reqwest::get(&url)
            .await?
            .json::<NcnPortalResponse>()
            .await?;

        if res.status {
            if let Some(addresses) = res.data {
                let mut tree_nodes = Vec::new();
                for address in addresses.iter() {
                    let tree_node = TreeNode::new(address, 0);
                    tree_nodes.push(tree_node);
                }

                let meta_merkle_tree = MetaMerkleTree::new(tree_nodes).unwrap();

                let root = meta_merkle_tree.merkle_root;

                let ix = admin_update_merkle_tree(
                    &self.ncn_portal_program_id,
                    &whitelist_address,
                    &keypair.pubkey(),
                    root,
                );

                let blockhash = rpc_client.get_latest_blockhash().await?;
                let tx = Transaction::new_signed_with_payer(
                    &[ix],
                    Some(&keypair.pubkey()),
                    &[keypair],
                    blockhash,
                );
                info!("Updating Whitelist transaction: {:?}", tx.get_signature());
                let result = rpc_client.send_and_confirm_transaction(&tx).await?;
                info!("Transaction confirmed: {:?}", result);
                let statuses = rpc_client
                    .get_signature_statuses(&[*tx.get_signature()])
                    .await?;

                let tx_status = statuses
                    .value
                    .first()
                    .unwrap()
                    .as_ref()
                    .ok_or_else(|| anyhow!("No signature status"))?;
                info!("Transaction status: {:?}", tx_status);
            }
        } else {
            info!("Failed to fetch whitelist addresses: {:?}", res.message);
        }

        Ok(())
    }
}
