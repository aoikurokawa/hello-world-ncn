use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use ncn_portal_core::whitelist::Whitelist;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process_admin_set_new_admin(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let [whitelist_info, admin_info, new_admin_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Whitelist::load(program_id, whitelist_info, false)?;
    let mut whitelist_data = whitelist_info.data.borrow_mut();
    let whitelist = Whitelist::try_from_slice_unchecked_mut(&mut whitelist_data)?;

    load_signer(admin_info, true)?;

    whitelist.check_admin(admin_info.key)?;

    whitelist.admin = *new_admin_info.key;
    msg!("Admin set to {}", new_admin_info.key);

    Ok(())
}
