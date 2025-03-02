use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use ncn_portal_core::{merkle_root::MerkleRoot, whitelist::Whitelist};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process_admin_update_merkle_root(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    root: [u8; 32],
) -> ProgramResult {
    let [whitelist_info, admin_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Whitelist::load(program_id, whitelist_info, true)?;
    let mut whitelist_data = whitelist_info.data.borrow_mut();
    let whitelist = Whitelist::try_from_slice_unchecked_mut(&mut whitelist_data)?;

    whitelist.check_admin(admin_info.key)?;

    load_signer(admin_info, true)?;

    msg!("Updating whitelist merkle root");

    whitelist.update_merkle_root(MerkleRoot { root });

    Ok(())
}
