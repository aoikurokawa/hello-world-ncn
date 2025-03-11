use hello_world_ncn_core::{
    ballot_box::BallotBox, config::Config as NcnConfig, message::Message, MAX_OPERATORS,
};
use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use jito_restaking_core::{
    config::Config as RestakingConfig, ncn::Ncn, ncn_operator_state::NcnOperatorState,
    ncn_vault_ticket::NcnVaultTicket, operator::Operator,
    operator_vault_ticket::OperatorVaultTicket,
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
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    message: String,
) -> ProgramResult {
    let [config_info, restaking_config_info, ncn_info, operator_info, vault_info, vault_ncn_ticket_info, ncn_vault_ticket_info, ncn_operator_state_info, vault_operator_delegation_info, operator_vault_ticket_info, message_info, ballot_box_info, operator_voter_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let clock = Clock::get()?;
    let current_slot = clock.slot;

    NcnConfig::load(program_id, config_info, ncn_info.key, false)?;
    let config_data = config_info.try_borrow_data()?;
    let ncn_config = NcnConfig::try_from_slice_unchecked(&config_data)?;

    RestakingConfig::load(&jito_restaking_program::id(), restaking_config_info, false)?;
    let restaking_config_data = restaking_config_info.try_borrow_data()?;
    let restaking_config = RestakingConfig::try_from_slice_unchecked(&restaking_config_data)?;

    Ncn::load(&jito_restaking_program::id(), ncn_info, false)?;

    Operator::load(&jito_restaking_program::id(), operator_info, false)?;
    let operator_data = operator_info.try_borrow_data()?;
    let operator = Operator::try_from_slice_unchecked(&operator_data)?;

    if operator.voter.ne(operator_voter_info.key) {
        msg!("Invalid Operator Voter Key");
        return Err(ProgramError::InvalidAccountData);
    }

    Vault::load(&jito_vault_program::id(), vault_info, false)?;

    VaultNcnTicket::load(
        &jito_vault_program::id(),
        vault_ncn_ticket_info,
        vault_info,
        ncn_info,
        false,
    )?;
    let vault_ncn_ticket_data = vault_ncn_ticket_info.try_borrow_data()?;
    let vault_ncn_ticket = VaultNcnTicket::try_from_slice_unchecked(&vault_ncn_ticket_data)?;

    if !vault_ncn_ticket
        .state
        .is_active(current_slot, restaking_config.epoch_length())?
    {
        msg!("VaultNcnTicket is not active");
        return Err(ProgramError::InvalidAccountData);
    }

    NcnVaultTicket::load(
        &jito_restaking_program::id(),
        ncn_vault_ticket_info,
        ncn_info,
        vault_info,
        false,
    )?;
    let ncn_vault_ticket_data = ncn_vault_ticket_info.try_borrow_data()?;
    let ncn_vault_ticket = NcnVaultTicket::try_from_slice_unchecked(&ncn_vault_ticket_data)?;

    if !ncn_vault_ticket
        .state
        .is_active(current_slot, restaking_config.epoch_length())?
    {
        msg!("NcnVaultTicket is not active");
        return Err(ProgramError::InvalidAccountData);
    }

    NcnOperatorState::load(
        &jito_restaking_program::id(),
        ncn_operator_state_info,
        ncn_info,
        operator_info,
        false,
    )?;
    let ncn_operator_state_data = ncn_operator_state_info.try_borrow_data()?;
    let ncn_operator_state = NcnOperatorState::try_from_slice_unchecked(&ncn_operator_state_data)?;

    if !ncn_operator_state
        .ncn_opt_in_state
        .is_active(current_slot, restaking_config.epoch_length())?
        || !ncn_operator_state
            .operator_opt_in_state
            .is_active(current_slot, restaking_config.epoch_length())?
    {
        msg!("NcnOperatorState is not active");
        return Err(ProgramError::InvalidAccountData);
    }

    VaultOperatorDelegation::load(
        &jito_vault_program::id(),
        vault_operator_delegation_info,
        vault_info,
        operator_info,
        false,
    )?;
    let vault_operator_delegation_data = vault_operator_delegation_info.try_borrow_data()?;
    let vault_operator_delegation =
        VaultOperatorDelegation::try_from_slice_unchecked(&vault_operator_delegation_data)?;

    let total_security = vault_operator_delegation
        .delegation_state
        .total_security()?;
    if total_security < ncn_config.min_stake() {
        msg!(
            "VaultOperatorDelegation is not correct, expected: {}, actual: {}",
            ncn_config.min_stake(),
            total_security
        );
        return Err(ProgramError::InvalidAccountData);
    }

    OperatorVaultTicket::load(
        &jito_restaking_program::id(),
        operator_vault_ticket_info,
        operator_info,
        vault_info,
        false,
    )?;

    let mut message_data = message_info.try_borrow_mut_data()?;
    let _message_acc = Message::try_from_slice_unchecked_mut(&mut message_data)?;

    let mut ballot_box_data = ballot_box_info.try_borrow_mut_data()?;
    let ballot_box = BallotBox::try_from_slice_unchecked_mut(&mut ballot_box_data)?;

    load_signer(operator_voter_info, false)?;

    let clock = Clock::get()?;
    let current_slot = clock.slot;
    let current_epoch = clock.slot;

    msg!("Operator {} voted for {}", operator_info.key, message);

    ballot_box.cast_vote(operator_info.key, &message, current_slot)?;

    if ballot_box.operators_voted() == MAX_OPERATORS {
        ballot_box.check_consensus_reached(current_slot);

        if ballot_box.is_consensus_reached() {
            msg!("Consensus reached for epoch {} with ballot", current_epoch,);
        }
    }

    Ok(())
}
