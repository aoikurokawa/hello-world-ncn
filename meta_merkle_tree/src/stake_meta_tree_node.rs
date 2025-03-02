// use serde::{Deserialize, Serialize};
// use solana_program::hash::{Hash, Hasher};
// use solana_sdk::pubkey::Pubkey;
//
// use crate::{error::MerkleRootGeneratorError, pubkey_string_conversion, stake_meta::StakeMeta};
//
// #[derive(Clone, Eq, Debug, Hash, PartialEq, Deserialize, Serialize)]
// pub struct StakeMetaTreeNode {
//     /// The stake account entitled to redeem.
//     #[serde(with = "pubkey_string_conversion")]
//     pub claimant: Pubkey,
//
//     // Pubkey of the ClaimStatus PDA account, this account should be closed to reclaim rent.
//     // #[serde(with = "pubkey_string_conversion")]
//     // pub claim_status_pubkey: Pubkey,
//
//     // Bump of the ClaimStatus PDA account
//     // pub claim_status_bump: u8,
//
//     // #[serde(with = "pubkey_string_conversion")]
//     // pub staker_pubkey: Pubkey,
//
//     // #[serde(with = "pubkey_string_conversion")]
//     // pub withdrawer_pubkey: Pubkey,
//
//     // The amount this account is entitled to.
//     // pub amount: u64,
//     /// The proof associated with this TreeNode
//     pub proof: Option<Vec<[u8; 32]>>,
// }
//
// impl StakeMetaTreeNode {
//     pub(crate) fn vec_from_stake_meta(
//         stake_meta: &StakeMeta,
//     ) -> Result<Option<Vec<Self>>, MerkleRootGeneratorError> {
//         if let Some(tip_distribution_meta) = stake_meta.maybe_tip_distribution_meta.as_ref() {
//             // let validator_amount = (tip_distribution_meta.total_tips as u128)
//             //     .checked_mul(tip_distribution_meta.validator_fee_bps as u128)
//             //     .unwrap()
//             //     .checked_div(10_000)
//             //     .unwrap() as u64;
//             // let (claim_status_pubkey, claim_status_bump) = Pubkey::find_program_address(
//             //     &[
//             //         CLAIM_STATUS_SEED,
//             //         &stake_meta.validator_vote_account.to_bytes(),
//             //         &tip_distribution_meta.tip_distribution_pubkey.to_bytes(),
//             //     ],
//             //     &jito_tip_distribution::ID,
//             // );
//             let tree_nodes = vec![Self {
//                 claimant: stake_meta.validator_vote_account,
//                 // claim_status_pubkey,
//                 // claim_status_bump,
//                 // staker_pubkey: Pubkey::default(),
//                 // withdrawer_pubkey: Pubkey::default(),
//                 // amount: validator_amount,
//                 proof: None,
//             }];
//
//             // let remaining_total_rewards = tip_distribution_meta
//             //     .total_tips
//             //     .checked_sub(validator_amount)
//             //     .unwrap() as u128;
//
//             // let total_delegated = stake_meta.total_delegated as u128;
//             // tree_nodes.extend(
//             //     stake_meta
//             //         .delegations
//             //         .iter()
//             //         .map(|delegation| {
//             //             let amount_delegated = delegation.lamports_delegated as u128;
//             //             let reward_amount = (amount_delegated.checked_mul(remaining_total_rewards))
//             //                 .unwrap()
//             //                 .checked_div(total_delegated)
//             //                 .unwrap();
//             //             let (claim_status_pubkey, claim_status_bump) = Pubkey::find_program_address(
//             //                 &[
//             //                     CLAIM_STATUS_SEED,
//             //                     &delegation.stake_account_pubkey.to_bytes(),
//             //                     &tip_distribution_meta.tip_distribution_pubkey.to_bytes(),
//             //                 ],
//             //                 &jito_tip_distribution::ID,
//             //             );
//             //             Ok(Self {
//             //                 claimant: delegation.stake_account_pubkey,
//             //                 claim_status_pubkey,
//             //                 claim_status_bump,
//             //                 staker_pubkey: delegation.staker_pubkey,
//             //                 withdrawer_pubkey: delegation.withdrawer_pubkey,
//             //                 amount: reward_amount as u64,
//             //                 proof: None,
//             //             })
//             //         })
//             //         .collect::<Result<Vec<Self>, MerkleRootGeneratorError>>()?,
//             // );
//
//             Ok(Some(tree_nodes))
//         } else {
//             Ok(None)
//         }
//     }
//
//     pub(crate) fn hash(&self) -> Hash {
//         let mut hasher = Hasher::default();
//         hasher.hash(self.claimant.as_ref());
//         // hasher.hash(self.amount.to_le_bytes().as_ref());
//         hasher.result()
//     }
// }
