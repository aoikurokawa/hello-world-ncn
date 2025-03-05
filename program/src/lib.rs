mod initialize_ballot_box;
mod initialize_config;
mod request_message;
mod submit_message;

use borsh::BorshDeserialize;
use const_str_to_pubkey::str_to_pubkey;
use hello_world_ncn_sdk::instruction::HelloWorldNcnInstruction;
use initialize_ballot_box::process_initialize_ballot_box;
use initialize_config::process_initialize_config;
use request_message::process_request_message;
use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};
use submit_message::process_submit_message;

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
            msg!("Instruction: InitializeConfig");
            process_initialize_config(program_id, accounts)
        }

        HelloWorldNcnInstruction::InitializeBallotBox => {
            msg!("Instruction: InitializeBallotBox");
            process_initialize_ballot_box(program_id, accounts)
        }

        HelloWorldNcnInstruction::RequestMessage => {
            msg!("Instruction: RequestMessage");
            process_request_message(program_id, accounts)
        } // HelloWorldNcnInstruction::SubmitMessage { message } => {
          //     msg!("Instruction: InitializeWhitelist");
          //     process_submit_message(program_id, accounts, message)
          // }
    }
}
