use solana_program::program_error::ProgramError;
use solana_program_test::BanksClientError;
use thiserror::Error;

pub mod fixture;
pub mod ncn_portal_client;

pub type TestResult<T> = Result<T, TestError>;

#[derive(Error, Debug)]
pub enum TestError {
    #[error(transparent)]
    BanksClientError(#[from] BanksClientError),
    #[error(transparent)]
    ProgramError(#[from] ProgramError),
}
