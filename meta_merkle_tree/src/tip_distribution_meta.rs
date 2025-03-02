use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

use crate::pubkey_string_conversion;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct TipDistributionMeta {
    #[serde(with = "pubkey_string_conversion")]
    pub merkle_root_upload_authority: Pubkey,

    #[serde(with = "pubkey_string_conversion")]
    pub tip_distribution_pubkey: Pubkey,

    /// The validator's total tips in the [TipDistributionAccount].
    pub total_tips: u64,

    /// The validator's cut of tips from [TipDistributionAccount], calculated from the on-chain
    /// commission fee bps.
    pub validator_fee_bps: u16,
}
