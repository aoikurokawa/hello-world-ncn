mod initialize_config;
mod request_message;
mod submit_message;

use borsh::BorshDeserialize;
use const_str_to_pubkey::str_to_pubkey;
use hello_world_ncn_sdk::instruction::HelloWorldNcnInstruction;
use initialize_config::process_initialize_config;
use request_message::process_request_message;
use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

declare_id!(str_to_pubkey(env!("HELLO_WORLD_NCN_PROGRAM_ID")));

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
            msg!("Instruction: InitializeWhitelist");
            process_initialize_config(program_id, accounts)
        }

        HelloWorldNcnInstruction::RequestMessage { message } => {
            msg!("Instruction: RequestMessage");
            process_request_message(program_id, accounts, message)
        }

        HelloWorldNcnInstruction::SubmitMessage => {
            msg!("Instruction: InitializeWhitelist");
            process_initialize_config(program_id, accounts)
        }
    }
}
