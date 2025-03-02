use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::load_signer;
use ncn_portal_core::whitelist::Whitelist;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process_check_whitelisted(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    proof: Vec<[u8; 32]>,
) -> ProgramResult {
    let [whitelist_info, whitelisted_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Whitelist::load(program_id, whitelist_info, false)?;
    let whitelist_data = whitelist_info.data.borrow();
    let whitelist = Whitelist::try_from_slice_unchecked(&whitelist_data)?;

    load_signer(whitelisted_info, false)?;

    whitelist.verify(proof, whitelisted_info.key.to_bytes());

    Ok(())
}
