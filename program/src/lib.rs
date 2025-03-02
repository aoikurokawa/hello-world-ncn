use borsh::BorshDeserialize;
use hello_world_ncn_sdk::instruction::HelloWorldNcnInstruction;
use initialize_config::process_initialize_config;
use solana_program::declare_id;
use solana_sdk::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

mod initialize_config;

declare_id!(env!("HELLO_WORLD_NCN_PROGRAM_ID"));

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction = HelloWorldNcnInstruction::try_from_slice(instruction_data)?;

    match instruction {
        HelloWorldNcnInstruction::InitializeConfig => {
            msg!("Instruction: InitializeConfig");
            process_initialize_config(program_id, accounts)?;
        }
    }

    Ok(())
}
