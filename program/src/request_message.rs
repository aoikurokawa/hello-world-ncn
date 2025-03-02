use hello_world_ncn_core::{ballot_box::BallotBox, config::Config, message::Message};
use hello_world_ncn_sdk::error::HelloWorldNcnError;
use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::{
    create_account,
    loader::{load_signer, load_system_account, load_system_program},
};
use jito_restaking_core::ncn::Ncn;
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
};

/// Request Message
///
/// Initialize two accounts
/// - Message
/// - Ballot Box
pub fn process_request_message(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    message: String,
) -> ProgramResult {
    let [config_info, ncn_info, message_info, ballot_box_info, ncn_admin_info, system_program_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let mut config_data = config_info.try_borrow_data()?;
    let _config = Config::try_from_slice_unchecked(&mut config_data)?;

    let ncn_data = ncn_info.try_borrow_data()?;
    let ncn = Ncn::try_from_slice_unchecked(&ncn_data)?;

    if ncn.ncn_program_admin.ne(ncn_admin_info.key) {
        msg!("Incorrect NCN Program Admin Key");
        return Err(ProgramError::InvalidAccountData);
    }

    load_system_account(message_info, true)?;
    load_signer(ncn_admin_info, true)?;
    load_system_program(system_program_info)?;

    let epoch = Clock::get()?.epoch;

    // The Message account shall be at the canonical PDA
    let (message_pubkey, message_bump, mut message_seeds) =
        Message::find_program_address(program_id, epoch);
    message_seeds.push(vec![message_bump]);
    if message_pubkey.ne(message_info.key) {
        msg!("Message account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    // The BallotBox account shall be at the canonical PDA
    let (ballot_box_pubkey, ballot_box_bump, mut ballot_box_seeds) =
        BallotBox::find_program_address(program_id, ncn_info.key, epoch);
    ballot_box_seeds.push(vec![ballot_box_bump]);
    if ballot_box_pubkey.ne(ballot_box_info.key) {
        msg!("BallotBox account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    msg!("Initialize Message at address {}", message_info.key);
    create_account(
        ncn_admin_info,
        message_info,
        system_program_info,
        program_id,
        &Rent::get()?,
        8_u64
            .checked_add(std::mem::size_of::<Message>() as u64)
            .ok_or(HelloWorldNcnError::ArithmeticOverflow)?,
        &message_seeds,
    )?;

    let mut message_data = message_info.try_borrow_mut_data()?;
    message_data[0] = Message::DISCRIMINATOR;
    let message_acc = Message::try_from_slice_unchecked_mut(&mut message_data)?;
    *message_acc = Message::new(epoch, &message);

    msg!("Initialize BallotBox at address {}", ballot_box_info.key);
    create_account(
        ncn_admin_info,
        message_info,
        system_program_info,
        program_id,
        &Rent::get()?,
        8_u64
            .checked_add(std::mem::size_of::<Message>() as u64)
            .ok_or(HelloWorldNcnError::ArithmeticOverflow)?,
        &message_seeds,
    )?;

    let mut message_data = message_info.try_borrow_mut_data()?;
    message_data[0] = Message::DISCRIMINATOR;
    let message_acc = Message::try_from_slice_unchecked_mut(&mut message_data)?;
    *message_acc = Message::new(epoch, &message);

    Ok(())
}
