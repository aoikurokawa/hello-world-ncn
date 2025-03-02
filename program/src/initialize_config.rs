use jito_jsm_core::loader::{load_signer, load_system_account, load_system_program};
use solana_sdk::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process_initialize_config(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let [config_info, ncn_admin_info, system_program_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    load_system_account(config_info, true)?;
    load_signer(ncn_admin_info, true)?;
    load_system_program(system_program_info)?;

    Ok(())
}
