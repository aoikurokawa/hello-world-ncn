use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{types::PodU64, Discriminator};
use shank::ShankAccount;
use solana_program::{account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey};

#[derive(Debug, Clone, Copy, Zeroable, ShankAccount, Pod)]
#[repr(C)]
pub struct Message {
    /// Operator's Pubkey
    pub operator_pubkey: Pubkey,

    /// Epoch
    pub epoch: PodU64,
}

impl Message {
    /// Initiallize a new Message
    pub fn new(operator_pubkey: Pubkey, epoch: u64) -> Self {
        Self {
            operator_pubkey,
            epoch: PodU64::from(epoch),
        }
    }

    /// Seeds of Message Account
    pub fn seeds(operator: Pubkey, epoch: u64) -> Vec<Vec<u8>> {
        vec![
            b"message".to_vec(),
            operator.to_bytes().to_vec(),
            epoch.to_be_bytes().to_vec(),
        ]
    }

    /// Find the program address of Message Account
    pub fn find_program_address(
        program_id: &Pubkey,
        operator: Pubkey,
        epoch: u64,
    ) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(operator, epoch);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    /// Load Message Account
    pub fn load(
        program_id: &Pubkey,
        account: &AccountInfo,
        expect_writable: bool,
        operator: Pubkey,
        epoch: u64,
    ) -> Result<(), ProgramError> {
        if account.owner.ne(program_id) {
            msg!("Config account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if account.data_is_empty() {
            msg!("Config account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !account.is_writable {
            msg!("Config account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if account.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("Config account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        if account
            .key
            .ne(&Self::find_program_address(program_id, operator, epoch).0)
        {
            msg!("Config account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}
