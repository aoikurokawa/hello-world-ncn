use hello_world_ncn_core::{ballot_box::BallotBox, config::Config as NcnConfig, message::Message};
use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use jito_restaking_core::{
    config::Config as RestakingConfig, ncn::Ncn, ncn_vault_ticket::NcnVaultTicket,
    operator::Operator,
};
use jito_vault_core::{
    vault::Vault, vault_ncn_ticket::VaultNcnTicket,
    vault_operator_delegation::VaultOperatorDelegation,
};
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey, sysvar::Sysvar,
};

pub fn process_submit_message(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    message: String,
) -> ProgramResult {
    let [config_info, restaking_config_info, ncn_info, operator_info, vault_info, vault_ncn_ticket_info, ncn_vault_ticket_info, vault_operator_delegation_info, message_info, ballot_box_info, operator_voter_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let clock = Clock::get()?;
    let current_slot = clock.slot;

    let config_data = config_info.try_borrow_data()?;
    let ncn_config = NcnConfig::try_from_slice_unchecked(&config_data)?;

    let restaking_config_data = restaking_config_info.try_borrow_data()?;
    let restaking_config = RestakingConfig::try_from_slice_unchecked(&restaking_config_data)?;

    let ncn_data = ncn_info.try_borrow_data()?;
    let _ncn = Ncn::try_from_slice_unchecked(&ncn_data)?;

    let operator_data = operator_info.try_borrow_data()?;
    let operator = Operator::try_from_slice_unchecked(&operator_data)?;

    if operator.voter.ne(operator_voter_info.key) {
        msg!("Invalid Operator Voter Key");
        return Err(ProgramError::InvalidAccountData);
    }

    let vault_data = vault_info.try_borrow_data()?;
    let _vault = Vault::try_from_slice_unchecked(&vault_data)?;

    let vault_ncn_ticket_data = vault_ncn_ticket_info.try_borrow_data()?;
    let vault_ncn_ticket = VaultNcnTicket::try_from_slice_unchecked(&vault_ncn_ticket_data)?;

    if !vault_ncn_ticket
        .state
        .is_active(current_slot, restaking_config.epoch_length())
    {
        return Err(ProgramError::InvalidAccountData);
    }

    let ncn_vault_ticket_data = ncn_vault_ticket_info.try_borrow_data()?;
    let ncn_vault_ticket = NcnVaultTicket::try_from_slice_unchecked(&ncn_vault_ticket_data)?;

    if !ncn_vault_ticket
        .state
        .is_active(current_slot, restaking_config.epoch_length())
    {
        return Err(ProgramError::InvalidAccountData);
    }

    let vault_operator_delegation_data = vault_operator_delegation_info.try_borrow_data()?;
    let vault_operator_delegation =
        VaultOperatorDelegation::try_from_slice_unchecked(&vault_operator_delegation_data)?;

    if vault_operator_delegation.delegation_state.staked_amount() < ncn_config.min_stake() {
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
