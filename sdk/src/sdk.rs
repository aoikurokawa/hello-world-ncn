use borsh::BorshSerialize;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use crate::instruction::HelloWorldNcnInstruction;

pub fn initialize_config(
    program_id: &Pubkey,
    config: &Pubkey,
    ncn: &Pubkey,
    ncn_admin: &Pubkey,
    min_stake: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*config, false),
        AccountMeta::new_readonly(*ncn, false),
        AccountMeta::new(*ncn_admin, true),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: HelloWorldNcnInstruction::InitializeConfig { min_stake }
            .try_to_vec()
            .unwrap(),
    }
}

pub fn initialize_ballot_box(
    program_id: &Pubkey,
    config: &Pubkey,
    ncn: &Pubkey,
    ballot_box: &Pubkey,
    ncn_admin: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(*config, false),
        AccountMeta::new_readonly(*ncn, false),
        AccountMeta::new(*ballot_box, false),
        AccountMeta::new(*ncn_admin, true),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: HelloWorldNcnInstruction::InitializeBallotBox
            .try_to_vec()
            .unwrap(),
    }
}

pub fn request_message(
    program_id: &Pubkey,
    config: &Pubkey,
    ncn: &Pubkey,
    message: &Pubkey,
    ncn_admin: &Pubkey,
    _message_data: String,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(*config, false),
        AccountMeta::new_readonly(*ncn, false),
        AccountMeta::new(*message, false),
        AccountMeta::new(*ncn_admin, true),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: HelloWorldNcnInstruction::RequestMessage
            .try_to_vec()
            .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn submit_message(
    program_id: &Pubkey,
    config_info: &Pubkey,
    ncn_info: &Pubkey,
    operator_info: &Pubkey,
    message_info: &Pubkey,
    ballot_box_info: &Pubkey,
    operator_voter_info: &Pubkey,
    message_data: String,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(*config_info, false),
        AccountMeta::new_readonly(*ncn_info, false),
        AccountMeta::new_readonly(*operator_info, false),
        AccountMeta::new_readonly(*message_info, false),
        AccountMeta::new(*ballot_box_info, false),
        AccountMeta::new_readonly(*operator_voter_info, true),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: HelloWorldNcnInstruction::SubmitMessage {
            message: message_data,
        }
        .try_to_vec()
        .unwrap(),
    }
}
