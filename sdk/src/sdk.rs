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
        data: HelloWorldNcnInstruction::InitializeConfig
            .try_to_vec()
            .unwrap(),
    }
}

// pub fn request_message(
//     program_id: &Pubkey,
//     config: &Pubkey,
//     ncn: &Pubkey,
//     message: &Pubkey,
//     ballot_box: &Pubkey,
//     ncn_admin: &Pubkey,
//     _message_data: String,
// ) -> Instruction {
//     let accounts = vec![
//         AccountMeta::new_readonly(*config, false),
//         AccountMeta::new_readonly(*ncn, false),
//         AccountMeta::new(*message, false),
//         AccountMeta::new(*ballot_box, false),
//         AccountMeta::new(*ncn_admin, true),
//         AccountMeta::new_readonly(system_program::id(), false),
//     ];
//     Instruction {
//         program_id: *program_id,
//         accounts,
//         data: HelloWorldNcnInstruction::RequestMessage
//             .try_to_vec()
//             .unwrap(),
//     }
// }
//
// pub fn admin_set_new_admin(
//     program_id: &Pubkey,
//     whitelist: &Pubkey,
//     admin: &Pubkey,
//     new_admin: &Pubkey,
// ) -> Instruction {
//     let accounts = vec![
//         AccountMeta::new(*whitelist, false),
//         AccountMeta::new(*admin, true),
//         AccountMeta::new_readonly(*new_admin, false),
//     ];
//     Instruction {
//         program_id: *program_id,
//         accounts,
//         data: NcnPortalInstruction::AdminSetNewAdmin.try_to_vec().unwrap(),
//     }
// }
//
// pub fn check_whitelisted(
//     program_id: &Pubkey,
//     whitelist: &Pubkey,
//     whitelisted: &Pubkey,
//     proof: Vec<[u8; 32]>,
// ) -> Instruction {
//     let accounts = vec![
//         AccountMeta::new_readonly(*whitelist, false),
//         AccountMeta::new_readonly(*whitelisted, true),
//     ];
//     Instruction {
//         program_id: *program_id,
//         accounts,
//         data: NcnPortalInstruction::CheckWhitelisted { proof }
//             .try_to_vec()
//             .unwrap(),
//     }
// }
