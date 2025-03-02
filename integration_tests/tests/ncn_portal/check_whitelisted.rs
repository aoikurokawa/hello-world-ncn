#[cfg(test)]
mod tests {
    use meta_merkle_tree::{meta_merkle_tree::MetaMerkleTree, tree_node::TreeNode};
    use solana_sdk::{pubkey::Pubkey, signature::Keypair};

    use crate::fixtures::fixture::TestBuilder;

    #[tokio::test]
    async fn test_check_whitelisted_ok() {
        let fixture = TestBuilder::new().await;

        let mut ncn_portal_program_client = fixture.ncn_portal_program_client();

        let alice = Pubkey::new_unique();
        let bob = Pubkey::new_unique();

        let tree_nodes = vec![TreeNode::new(&alice, 0), TreeNode::new(&bob, 0)];
        let merkle_info = MetaMerkleTree::new(tree_nodes).unwrap();

        let _admin = ncn_portal_program_client
            .do_initialize_whitelist(&merkle_info.merkle_root)
            .await
            .unwrap();

        let whitelisted = Keypair::new();

        let proof = &merkle_info.tree_nodes[0].proof.clone().unwrap().clone();

        ncn_portal_program_client
            .do_check_whitelisted(&whitelisted, proof.clone())
            .await
            .unwrap();
    }
}
