use hello_world_ncn_core::message::Message;
use hello_world_ncn_sdk::error::HelloWorldNcnError;
use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::{
    create_account,
    loader::{load_signer, load_system_account, load_system_program},
};
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
};

pub fn process_request_message(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    message: String,
) -> ProgramResult {
    let [message_info, ncn_admin_info, system_program_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    load_system_account(message_info, true)?;
    load_signer(ncn_admin_info, true)?;
    load_system_program(system_program_info)?;

    let epoch = Clock::get()?.epoch;

    // The Config account shall be at the canonical PDA
    let (message_pubkey, message_bump, mut message_seeds) =
        Message::find_program_address(program_id, epoch);
    message_seeds.push(vec![message_bump]);
    if message_pubkey.ne(message_info.key) {
        msg!("Message account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    msg!("Request Message at address {}", message_info.key);
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
