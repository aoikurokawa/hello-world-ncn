use bytemuck::{Pod, Zeroable};
use jito_bytemuck::types::{PodU16, PodU64};
use shank::ShankType;
use solana_program::pubkey::Pubkey;

#[derive(Debug, Clone, Copy, Zeroable, ShankType, Pod)]
#[repr(C)]
pub struct OperatorVote {
    /// The operator that cast the vote
    operator: Pubkey,

    /// The slot the operator voted
    slot_voted: PodU64,

    /// The index of the ballot in the ballot_tallies
    vote_index: PodU16,

    /// The message operator submitted
    message_data: [u8; 64],
}

impl Default for OperatorVote {
    fn default() -> Self {
        Self {
            operator: Pubkey::default(),
            slot_voted: PodU64::from(0),
            vote_index: PodU16::from(0),
            message_data: [0; 64],
        }
    }
}

impl OperatorVote {
    pub fn new(operator: Pubkey, current_slot: u64, message_data: &str) -> Self {
        let mut message_slice = [0u8; 64];
        message_slice.copy_from_slice(message_data.as_bytes());

        Self {
            operator,
            slot_voted: PodU64::from(current_slot),
            vote_index: PodU16::from(0),
            message_data: message_slice,
        }
    }

    pub fn operator(&self) -> Pubkey {
        self.operator
    }

    pub fn slot_voted(&self) -> u64 {
        self.slot_voted.into()
    }

    pub fn vote_index(&self) -> u16 {
        self.vote_index.into()
    }

    pub fn message_data(&self) -> String {
        String::from_utf8(self.message_data.to_vec()).unwrap()
    }
}
