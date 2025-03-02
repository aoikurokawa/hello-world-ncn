// use serde::{Deserialize, Serialize};
//
// use crate::{
//     error::MerkleRootGeneratorError, generated_merkle_tree::GeneratedMerkleTree,
//     merkle_tree::MerkleTree, stake_meta_collection::StakeMetaCollection,
//     stake_meta_tree_node::StakeMetaTreeNode, utils::get_proof,
// };
//
// #[derive(Clone, Deserialize, Serialize, Debug)]
// pub struct GeneratedMerkleTreeCollection {
//     pub generated_merkle_trees: Vec<GeneratedMerkleTree>,
//     // pub bank_hash: String,
//     // pub epoch: Epoch,
//     // pub slot: Slot,
// }
//
// impl GeneratedMerkleTreeCollection {
//     pub fn new_from_stake_meta_collection(
//         stake_meta_coll: StakeMetaCollection,
//     ) -> Result<Self, MerkleRootGeneratorError> {
//         let generated_merkle_trees = stake_meta_coll
//             .stake_metas
//             .into_iter()
//             // .filter(|stake_meta| stake_meta.maybe_tip_distribution_meta.is_some())
//             .filter_map(|stake_meta| {
//                 let mut tree_nodes = match StakeMetaTreeNode::vec_from_stake_meta(&stake_meta) {
//                     Err(e) => return Some(Err(e)),
//                     Ok(maybe_tree_nodes) => maybe_tree_nodes,
//                 }?;
//
//                 // if let Some(rpc_client) = &maybe_rpc_client {
//                 //     if let Some(tda) = stake_meta.maybe_tip_distribution_meta.as_ref() {
//                 // emit_inconsistent_tree_node_amount_dp(
//                 //     &tree_nodes[..],
//                 //     &tda.tip_distribution_pubkey,
//                 //     rpc_client,
//                 // );
//                 //     }
//                 // }
//
//                 let hashed_nodes: Vec<[u8; 32]> =
//                     tree_nodes.iter().map(|n| n.hash().to_bytes()).collect();
//
//                 // let tip_distribution_meta = stake_meta.maybe_tip_distribution_meta.unwrap();
//
//                 let merkle_tree = MerkleTree::new(&hashed_nodes[..], true);
//                 let max_num_nodes = tree_nodes.len() as u64;
//
//                 for (i, tree_node) in tree_nodes.iter_mut().enumerate() {
//                     tree_node.proof = Some(get_proof(&merkle_tree, i));
//                 }
//
//                 Some(Ok(GeneratedMerkleTree {
//                     max_num_nodes,
//                     // tip_distribution_account: tip_distribution_meta.tip_distribution_pubkey,
//                     merkle_root_upload_authority: tip_distribution_meta
//                         .merkle_root_upload_authority,
//                     merkle_root: *merkle_tree.get_root().unwrap(),
//                     tree_nodes,
//                     // max_total_claim: tip_distribution_meta.total_tips,
//                 }))
//             })
//             .collect::<Result<Vec<GeneratedMerkleTree>, MerkleRootGeneratorError>>()?;
//
//         Ok(Self {
//             generated_merkle_trees,
//             // bank_hash: stake_meta_coll.bank_hash,
//             // epoch: stake_meta_coll.epoch,
//             // slot: stake_meta_coll.slot,
//         })
//     }
// }
