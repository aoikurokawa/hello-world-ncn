// Mostly copied from modules in jito-solana/tip-distributor/src
// To be replaced by tip distributor code in this repo

use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

use crate::pubkey_string_conversion;

#[derive(Clone, Eq, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub struct GeneratedMerkleTree {
    /// User account (wallet pubkey)
    #[serde(with = "pubkey_string_conversion")]
    pub user_account: Pubkey,
    // #[serde(with = "pubkey_string_conversion")]
    // pub merkle_root_upload_authority: Pubkey,

    // pub merkle_root: Hash,

    // pub tree_nodes: Vec<StakeMetaTreeNode>,

    // pub max_total_claim: u64,
    pub max_num_nodes: u64,
}
