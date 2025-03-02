use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[derive(Debug, BorshSerialize, BorshDeserialize, ShankInstruction)]
pub enum HelloWorldNcnInstruction {
    /// Initializes Hello World NCN configuration
    #[account(0, writable, name = "config_info")]
    #[account(1, writable, signer, name = "ncn_admin_info")]
    #[account(2, name = "system_program_info")]
    InitializeConfig,
}
