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
    operator_votes: [OperatorVote; 5],
}

impl BallotBox {
    pub fn new(ncn: Pubkey, epoch: u64, current_slot: u64) -> Self {
        Self {
            ncn,
            epoch: PodU64::from(epoch),
            slot_created: PodU64::from(current_slot),
            slot_consensus_reached: PodU64::from(u64::MAX),
            operators_voted: PodU64::from(0),
            operator_votes: [OperatorVote::default(); 5],
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

    pub fn is_consensus_reached(&self) -> bool {
        self.slot_consensus_reached() != u64::MAX
    }

    /// Determines if an operator can still cast their vote.
    /// Returns true when:
    /// Consensus is not reached OR the voting window is still valid, assuming set_tie_breaker was not invoked
    pub fn is_voting_valid(&self, current_slot: u64) -> Result<bool, HelloWorldNcnError> {
        // if self.tie_breaker_set() {
        //     return Ok(false);
        // }

        // if self.is_consensus_reached() {
        //     let vote_window_valid = current_slot
        //         <= self
        //             .slot_consensus_reached()
        //             .checked_add(valid_slots_after_consensus)
        //             .ok_or(HelloWorldNcnError::ArithmeticOverflow)?;

        //     return Ok(vote_window_valid);
        // }

        Ok(true)
    }

    // fn increment_or_create_ballot_tally(&mut self) -> Result<usize, TipRouterError> {
    //     let result = self
    //         .ballot_tallies
    //         .iter()
    //         .enumerate()
    //         .find(|(_, t)| t.is_valid() && t.ballot.eq(ballot));

    //     if let Some((tally_index, _)) = result {
    //         self.ballot_tallies[tally_index].increment_tally(stake_weights)?;
    //         return Ok(tally_index);
    //     }

    //     for (tally_index, tally) in self.ballot_tallies.iter_mut().enumerate() {
    //         if !tally.is_valid() {
    //             *tally = BallotTally::new(tally_index as u16, ballot, stake_weights);

    //             self.unique_ballots = PodU64::from(
    //                 self.unique_ballots()
    //                     .checked_add(1)
    //                     .ok_or(TipRouterError::ArithmeticOverflow)?,
    //             );

    //             return Ok(tally_index);
    //         }
    //     }

    //     Err(TipRouterError::BallotTallyFull)
    // }

    /// Operator cast vote with message
    pub fn cast_vote(
        &mut self,
        operator: &Pubkey,
        message_data: &str,
        current_slot: u64,
    ) -> Result<(), HelloWorldNcnError> {
        // if !self.is_voting_valid(current_slot)? {
        //     return Err(TipRouterError::VotingNotValid);
        // }

        // if !ballot.is_valid() {
        //     return Err(TipRouterError::BadBallot);
        // }

        // let ballot_index = self.increment_or_create_ballot_tally(ballot, stake_weights)?;

        // let unique_ballots = self.unique_ballots();
        let consensus_reached = self.is_consensus_reached();

        for vote in self.operator_votes.iter_mut() {
            if vote.operator().eq(operator) {
                if consensus_reached {
                    return Err(HelloWorldNcnError::ConsensusAlreadyReached);
                }

                // If the operator has already voted, we need to decrement their vote from the previous ballot
                let prev_ballot_index = vote.vote_index();
                // if let Some(prev_tally) = self.ballot_tallies.get_mut(prev_ballot_index as usize) {
                // prev_tally.decrement_tally(vote.stake_weights())?;

                // If no more operators voting for the previous ballot, wipe and decrement the unique ballots
                // if prev_tally.tally() == 0 {
                //     *prev_tally = BallotTally::default();
                //     self.unique_ballots = PodU64::from(
                //         unique_ballots
                //             .checked_sub(1)
                //             .ok_or(TipRouterError::ArithmeticOverflow)?,
                //     );
                // }
                // }

                let operator_vote = OperatorVote::new(*operator, current_slot, message_data);
                *vote = operator_vote;
                return Ok(());
            }

            // if vote.is_empty() {
            //     let operator_vote =
            //         OperatorVote::new(ballot_index, operator, current_slot, stake_weights);
            //     *vote = operator_vote;

            //     self.operators_voted = PodU64::from(
            //         self.operators_voted()
            //             .checked_add(1)
            //             .ok_or(TipRouterError::ArithmeticOverflow)?,
            //     );
            //     return Ok(());
            // }
        }

        Err(HelloWorldNcnError::OperatorVotesFull)
    }
}
