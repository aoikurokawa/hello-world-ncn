use solana_program::program_error::ProgramError;
use solana_program_test::BanksClientError;
use solana_sdk::{instruction::InstructionError, transaction::TransactionError};
use thiserror::Error;

pub mod hello_world_ncn_client;
pub mod restaking_client;
pub mod test_builder;
pub mod vault_client;

pub type TestResult<T> = Result<T, TestError>;

#[derive(Error, Debug)]
pub enum TestError {
    #[error(transparent)]
    BanksClientError(#[from] BanksClientError),

    #[error(transparent)]
    ProgramError(#[from] ProgramError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
    // #[error(transparent)]
    // AnchorError(#[from] anchor_lang::error::Error),
}

impl TestError {
    pub fn to_transaction_error(&self) -> Option<TransactionError> {
        match self {
            Self::BanksClientError(e) => match e {
                BanksClientError::TransactionError(e) => Some(e.clone()),
                BanksClientError::SimulationError { err, .. } => Some(err.clone()),
                _ => None,
            },
            Self::ProgramError(_) => None,
            _ => None,
        }
    }
}

#[allow(dead_code)]
#[inline(always)]
#[track_caller]
pub fn assert_ix_error<T>(test_error: Result<T, TestError>, ix_error: InstructionError) {
    assert!(test_error.is_err());
    assert_eq!(
        test_error.err().unwrap().to_transaction_error().unwrap(),
        TransactionError::InstructionError(0, ix_error)
    );
}
