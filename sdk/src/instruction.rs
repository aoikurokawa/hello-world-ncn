use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[derive(Debug, BorshSerialize, BorshDeserialize, ShankInstruction)]
pub enum HelloWorldNcnInstruction {
    /// Initializes global configuration
    #[account(0, writable, name = "config_info")]
    #[account(1, writable, signer, name = "ncn_admin_info")]
    #[account(2, name = "system_program_info")]
    InitializeConfig,

    /// Request Message
    #[account(0, writable, name = "config_info")]
    #[account(1, writable, name = "message_info")]
    #[account(2, writable, signer, name = "ncn_admin_info")]
    #[account(3, name = "system_program_info")]
    RequestMessage { message: String },

    /// Send Message
    #[account(0, writable, name = "config_info")]
    #[account(1, writable, name = "message_info")]
    #[account(2, writable, signer, name = "operator_info")]
    #[account(3, name = "system_program_info")]
    SubmitMessage,
}
