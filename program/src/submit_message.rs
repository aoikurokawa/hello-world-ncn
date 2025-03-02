use jito_jsm_core::loader::{load_signer, load_system_account, load_system_program};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process_submit_message(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    // let [message_info, operator_voter_info, system_program_info] = accounts else {
    //     return Err(ProgramError::NotEnoughAccountKeys);
    // };

    // load_system_account(message_info, true)?;
    // load_signer(ncn_admin_info, true)?;
    // load_system_program(system_program_info)?;

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
