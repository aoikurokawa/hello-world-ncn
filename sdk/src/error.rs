use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HelloWorldNcnError {
    #[error("NcnPortalWhitelistAdminInvalid")]
    NcnPortalWhitelistAdminInvalid,

    #[error("NcnPortalParentInvalid")]
    NcnPortalParentInvalid,

    #[error("NcnPortalWhitelistedInvalid")]
    NcnPortalWhitelistedInvalid,

    #[error("ConsensusAlreadyReached")]
    ConsensusAlreadyReached,

    #[error("OperatorVotesFull")]
    OperatorVotesFull,

    #[error("ArithmeticOverflow")]
    ArithmeticOverflow = 3000,

    #[error("ArithmeticUnderflow")]
    ArithmeticUnderflow,

    #[error("DivisionByZero")]
    DivisionByZero,
}

impl From<HelloWorldNcnError> for ProgramError {
    fn from(e: HelloWorldNcnError) -> Self {
        Self::Custom(e as u32)
    }
}

impl From<HelloWorldNcnError> for u64 {
    fn from(e: HelloWorldNcnError) -> Self {
        e as Self
    }
}

impl From<HelloWorldNcnError> for u32 {
    fn from(e: HelloWorldNcnError) -> Self {
        e as Self
    }
}
