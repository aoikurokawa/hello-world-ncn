use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum NcnPortalError {
    #[error("NcnPortalWhitelistAdminInvalid")]
    NcnPortalWhitelistAdminInvalid,
    #[error("NcnPortalParentInvalid")]
    NcnPortalParentInvalid,
    #[error("NcnPortalWhitelistedInvalid")]
    NcnPortalWhitelistedInvalid,

    #[error("ArithmeticOverflow")]
    ArithmeticOverflow = 3000,
    #[error("ArithmeticUnderflow")]
    ArithmeticUnderflow,
    #[error("DivisionByZero")]
    DivisionByZero,
}

impl From<NcnPortalError> for ProgramError {
    fn from(e: NcnPortalError) -> Self {
        Self::Custom(e as u32)
    }
}

impl From<NcnPortalError> for u64 {
    fn from(e: NcnPortalError) -> Self {
        e as Self
    }
}

impl From<NcnPortalError> for u32 {
    fn from(e: NcnPortalError) -> Self {
        e as Self
    }
}
