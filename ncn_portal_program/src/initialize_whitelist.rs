use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::{
    create_account,
    loader::{load_signer, load_system_account, load_system_program},
};
use ncn_portal_core::{merkle_root::MerkleRoot, whitelist::Whitelist};
use ncn_portal_sdk::error::NcnPortalError;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
};

pub fn process_initialize_whitelist(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    root: [u8; 32],
) -> ProgramResult {
    let [whitelist, admin, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    load_system_account(whitelist, true)?;
    load_signer(admin, true)?;
    load_system_program(system_program)?;

    // The whitelist account shall be at the canonical PDA
    let (whitelist_pubkey, whitelist_bump, mut whitelist_seeds) =
        Whitelist::find_program_address(program_id);
    whitelist_seeds.push(vec![whitelist_bump]);
    if whitelist_pubkey.ne(whitelist.key) {
        msg!("Whitelist account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    msg!("Initializing whitelist at address {}", whitelist.key);
    create_account(
        admin,
        whitelist,
        system_program,
        program_id,
        &Rent::get()?,
        8_u64
            .checked_add(std::mem::size_of::<Whitelist>() as u64)
            .ok_or(NcnPortalError::ArithmeticOverflow)?,
        &whitelist_seeds,
    )?;

    let mut whitelist_data = whitelist.try_borrow_mut_data()?;
    whitelist_data[0] = Whitelist::DISCRIMINATOR;
    let whitelist = Whitelist::try_from_slice_unchecked_mut(&mut whitelist_data)?;
    *whitelist = Whitelist::new(*admin.key, MerkleRoot { root });

    Ok(())
}
