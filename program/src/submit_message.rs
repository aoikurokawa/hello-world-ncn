use hello_world_ncn_core::{config::Config, message::Message};
use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::{load_signer, load_system_account, load_system_program};
use jito_restaking_core::operator::Operator;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process_submit_message(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let [config_info, message_info, operator_info, operator_voter_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let mut config_data = config_info.try_borrow_data()?;
    let _config = Config::try_from_slice_unchecked(&mut config_data)?;

    let mut message_data = message_info.try_borrow_mut_data()?;
    let _message = Message::try_from_slice_unchecked_mut(&mut message_data)?;

    let operator_data = operator_info.try_borrow_data()?;
    let operator = Operator::try_from_slice_unchecked(&operator_data)?;

    if operator.voter.ne(&operator_voter_info.key) {
        msg!("Invalid Operator Voter Key");
        return Err(ProgramError::InvalidAccountData);
    }

    // // The Config account shall be at the canonical PDA
    // let (config_pubkey, config_bump, mut config_seeds) = Config::find_program_address(program_id);
    // config_seeds.push(vec![config_bump]);
    // if config_pubkey.ne(config_info.key) {
    //     msg!("Config account is not at the correct PDA");
    //     return Err(ProgramError::InvalidAccountData);
    // }

    // msg!("Initializing Config at address {}", config_info.key);
    // create_account(
    //     ncn_admin_info,
    //     config_info,
    //     system_program_info,
    //     program_id,
    //     &Rent::get()?,
    //     8_u64
    //         .checked_add(std::mem::size_of::<Config>() as u64)
    //         .ok_or(HelloWorldNcnError::ArithmeticOverflow)?,
    //     &config_seeds,
    // )?;

    // let mut whitelist_data = whitelist.try_borrow_mut_data()?;
    // whitelist_data[0] = Whitelist::DISCRIMINATOR;
    // let whitelist = Whitelist::try_from_slice_unchecked_mut(&mut whitelist_data)?;
    // *whitelist = Whitelist::new(*admin.key, MerkleRoot { root });

    Ok(())
}
