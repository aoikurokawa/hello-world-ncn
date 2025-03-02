use hello_world_ncn_core::{ballot_box::BallotBox, config::Config, message::Message};
use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use jito_restaking_core::{ncn::Ncn, operator::Operator};
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey, sysvar::Sysvar,
};

pub fn process_submit_message(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    message: String,
) -> ProgramResult {
    let [config_info, ncn_info, operator_info, message_info, ballot_box_info, operator_voter_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let mut config_data = config_info.try_borrow_data()?;
    let _config = Config::try_from_slice_unchecked(&mut config_data)?;

    let ncn_data = ncn_info.try_borrow_data()?;
    let _ncn = Ncn::try_from_slice_unchecked(&ncn_data)?;

    let operator_data = operator_info.try_borrow_data()?;
    let operator = Operator::try_from_slice_unchecked(&operator_data)?;

    if operator.voter.ne(&operator_voter_info.key) {
        msg!("Invalid Operator Voter Key");
        return Err(ProgramError::InvalidAccountData);
    }

    let mut message_data = message_info.try_borrow_mut_data()?;
    let message_acc = Message::try_from_slice_unchecked_mut(&mut message_data)?;

    let mut ballot_box_data = ballot_box_info.try_borrow_mut_data()?;
    let ballot_box = BallotBox::try_from_slice_unchecked_mut(&mut ballot_box_data)?;

    load_signer(operator_voter_info, false)?;

    if !message.starts_with(&message_acc.keyword()) {
        return Err(ProgramError::InvalidAccountData);
    }

    let clock = Clock::get()?;
    let current_slot = clock.slot;
    let current_epoch = clock.slot;

    ballot_box.cast_vote(operator_info.key, &message, current_slot)?;

    if ballot_box.is_consensus_reached() {
        msg!("Consensus reached for epoch {} with ballot", current_epoch,);
    }

    Ok(())
}
