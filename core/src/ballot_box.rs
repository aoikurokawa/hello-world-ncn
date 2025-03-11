use bytemuck::{Pod, Zeroable};
use hello_world_ncn_sdk::error::HelloWorldNcnError;
use jito_bytemuck::{types::PodU64, AccountDeserialize, Discriminator};
use shank::ShankAccount;
use solana_program::{account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey};

use crate::operator_vote::OperatorVote;

#[derive(Debug, Clone, Copy, Zeroable, ShankAccount, Pod, AccountDeserialize)]
#[repr(C)]
pub struct BallotBox {
    /// The NCN account this ballot box is for
    ncn: Pubkey,

    /// The epoch this ballot box is for
    epoch: PodU64,

    /// Slot when this ballot box was created
    slot_created: PodU64,

    /// Slot when consensus reached
    slot_consensus_reached: PodU64,

    /// Number of operators that have voted
    operators_voted: PodU64,

    /// Operator votes
    pub operator_votes: [OperatorVote; 3],
}

impl BallotBox {
    pub fn new(ncn: Pubkey, epoch: u64, current_slot: u64) -> Self {
        Self {
            ncn,
            epoch: PodU64::from(epoch),
            slot_created: PodU64::from(current_slot),
            slot_consensus_reached: PodU64::from(u64::MAX),
            operators_voted: PodU64::from(0),
            operator_votes: [OperatorVote::default(); 3],
        }
    }

    pub fn seeds(ncn: &Pubkey, epoch: u64) -> Vec<Vec<u8>> {
        Vec::from_iter([
            b"ballot_box".to_vec(),
            ncn.to_bytes().to_vec(),
            epoch.to_le_bytes().to_vec(),
        ])
    }

    pub fn find_program_address(
        program_id: &Pubkey,
        ncn: &Pubkey,
        epoch: u64,
    ) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(ncn, epoch);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    pub fn load(
        program_id: &Pubkey,
        account: &AccountInfo,
        ncn: &Pubkey,
        epoch: u64,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if account.owner.ne(program_id) {
            msg!("Config account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if account.data_is_empty() {
            msg!("Config account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !account.is_writable {
            msg!("Config account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if account.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("Config account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        if account
            .key
            .ne(&Self::find_program_address(program_id, ncn, epoch).0)
        {
            msg!("Config account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }

    pub fn slot_consensus_reached(&self) -> u64 {
        self.slot_consensus_reached.into()
    }

    pub fn set_slot_consensus_reached(&mut self, slot: u64) {
        self.slot_consensus_reached = PodU64::from(slot);
    }

    pub fn is_consensus_reached(&self) -> bool {
        self.slot_consensus_reached() != u64::MAX
    }

    pub fn operators_voted(&self) -> u64 {
        self.operators_voted.into()
    }

    /// Operator cast vote with message
    pub fn cast_vote(
        &mut self,
        operator: &Pubkey,
        message_data: &str,
        current_slot: u64,
    ) -> Result<(), HelloWorldNcnError> {
        for vote in self.operator_votes.iter_mut() {
            if vote.is_empty() {
                let operator_vote = OperatorVote::new(*operator, current_slot, message_data);
                *vote = operator_vote;

                self.operators_voted = PodU64::from(
                    self.operators_voted()
                        .checked_add(1)
                        .ok_or(HelloWorldNcnError::ArithmeticOverflow)?,
                );

                if self.operators_voted() > 1 {
                    self.set_slot_consensus_reached(current_slot);
                }

                return Ok(());
            }
        }

        Err(HelloWorldNcnError::OperatorVotesFull)
    }
}
