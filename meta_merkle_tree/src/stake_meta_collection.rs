// use serde::{Deserialize, Serialize};
// use solana_sdk::pubkey::Pubkey;
//
// use crate::{pubkey_string_conversion, stake_meta::StakeMeta};
//
// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct StakeMetaCollection {
//     /// List of [StakeMeta].
//     pub stake_metas: Vec<StakeMeta>,
//
//     // base58 encoded tip-distribution program id.
//     // #[serde(with = "pubkey_string_conversion")]
//     // pub tip_distribution_program_id: Pubkey,
//
//     // Base58 encoded bank hash this object was generated at.
//     // pub bank_hash: String,
//
//     // Epoch for which this object was generated for.
//     // pub epoch: Epoch,
//
//     // Slot at which this object was generated.
//     // pub slot: Slot,
// }
