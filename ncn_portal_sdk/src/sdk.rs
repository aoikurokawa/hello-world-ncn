use borsh::BorshSerialize;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use crate::instruction::NcnPortalInstruction;

pub fn initialize_whitelist(
    program_id: &Pubkey,
    whitelist: &Pubkey,
    admin: &Pubkey,
    root: [u8; 32],
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*whitelist, false),
        AccountMeta::new(*admin, true),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: NcnPortalInstruction::InitializeWhitelist { root }
            .try_to_vec()
            .unwrap(),
    }
}

pub fn admin_update_merkle_tree(
    program_id: &Pubkey,
    whitelist: &Pubkey,
    admin: &Pubkey,
    root: [u8; 32],
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*whitelist, false),
        AccountMeta::new_readonly(*admin, true),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: NcnPortalInstruction::AdminUpdateMerkleRoot { root }
            .try_to_vec()
            .unwrap(),
    }
}

pub fn admin_set_new_admin(
    program_id: &Pubkey,
    whitelist: &Pubkey,
    admin: &Pubkey,
    new_admin: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*whitelist, false),
        AccountMeta::new(*admin, true),
        AccountMeta::new_readonly(*new_admin, false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: NcnPortalInstruction::AdminSetNewAdmin.try_to_vec().unwrap(),
    }
}

pub fn check_whitelisted(
    program_id: &Pubkey,
    whitelist: &Pubkey,
    whitelisted: &Pubkey,
    proof: Vec<[u8; 32]>,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(*whitelist, false),
        AccountMeta::new_readonly(*whitelisted, true),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: NcnPortalInstruction::CheckWhitelisted { proof }
            .try_to_vec()
            .unwrap(),
    }
}
