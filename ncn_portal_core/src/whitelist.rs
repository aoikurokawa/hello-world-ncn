use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{AccountDeserialize, Discriminator};
use ncn_portal_sdk::error::NcnPortalError;
use shank::ShankAccount;
use solana_program::{account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey};

use crate::{discriminators::Discriminators, merkle_root::MerkleRoot};

/// The "base" whitelist account upon which all whitelist entry account addresses are derived
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct Whitelist {
    // The account that created this whitelist
    pub admin: Pubkey,

    pub merkle_root: MerkleRoot,
}

impl Discriminator for Whitelist {
    const DISCRIMINATOR: u8 = Discriminators::Whitelist as u8;
}

impl Whitelist {
    pub fn new(admin: Pubkey, merkle_root: MerkleRoot) -> Self {
        Self { admin, merkle_root }
    }

    pub fn check_admin(&self, admin_info: &Pubkey) -> Result<(), NcnPortalError> {
        if self.admin.eq(admin_info) {
            Ok(())
        } else {
            Err(NcnPortalError::NcnPortalWhitelistAdminInvalid)
        }
    }

    pub fn update_merkle_root(&mut self, merkle_root: MerkleRoot) {
        self.merkle_root = merkle_root;
    }

    /// This function deals with verification of Merkle trees (hash trees).
    /// Direct port of https://github.com/OpenZeppelin/openzeppelin-contracts/blob/v3.4.0/contracts/cryptography/MerkleProof.sol
    /// Returns true if a `leaf` can be proved to be a part of a Merkle tree
    /// defined by `root`. For this, a `proof` must be provided, containing
    /// sibling hashes on the branch from the leaf to the root of the tree. Each
    /// pair of leaves and each pair of pre-images are assumed to be sorted.
    pub fn verify(&self, proof: Vec<[u8; 32]>, leaf: [u8; 32]) -> bool {
        let mut computed_hash = leaf;
        for proof_element in proof.into_iter() {
            if computed_hash <= proof_element {
                // Hash(current computed hash + current element of the proof)
                computed_hash =
                    solana_program::hash::hashv(&[&[1u8], &computed_hash, &proof_element])
                        .to_bytes();
            } else {
                // Hash(current element of the proof + current computed hash)
                computed_hash =
                    solana_program::hash::hashv(&[&[1u8], &proof_element, &computed_hash])
                        .to_bytes();
            }
        }
        // Check if the computed hash (root) is equal to the provided root
        computed_hash == self.merkle_root.root
    }

    pub fn seeds() -> Vec<Vec<u8>> {
        vec![b"whitelist".to_vec()]
    }

    pub fn find_program_address(program_id: &Pubkey) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds();
        let (address, bump) = Pubkey::find_program_address(
            &seeds.iter().map(|s| s.as_slice()).collect::<Vec<_>>(),
            program_id,
        );
        (address, bump, seeds)
    }

    pub fn load(
        program_id: &Pubkey,
        whitelist_account: &AccountInfo,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if whitelist_account.owner.ne(program_id) {
            msg!("Whitelist account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if whitelist_account.data_is_empty() {
            msg!("Whitelist account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !whitelist_account.is_writable {
            msg!("Whitelist account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if whitelist_account.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("Whitelist account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        if whitelist_account
            .key
            .ne(&Self::find_program_address(program_id).0)
        {
            msg!("Whitelist account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}
