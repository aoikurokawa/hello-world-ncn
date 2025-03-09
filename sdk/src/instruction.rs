use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[derive(Debug, BorshSerialize, BorshDeserialize, ShankInstruction)]
pub enum HelloWorldNcnInstruction {
    /// Initializes NCN global configuration
    #[account(0, writable, name = "config_info")]
    #[account(1, name = "ncn_info")]
    #[account(2, writable, signer, name = "ncn_admin_info")]
    #[account(3, name = "system_program")]
    InitializeConfig { min_stake: u64 },

    /// Initializes NCN Ballot Box
    #[account(0, name = "config_info")]
    #[account(1, name = "ncn_info")]
    #[account(2, writable, name = "ballot_box_info")]
    #[account(3, writable, signer, name = "ncn_admin_info")]
    #[account(4, name = "system_program")]
    InitializeBallotBox,

    /// Request Message
    #[account(0, name = "config_info")]
    #[account(1, name = "ncn_info")]
    #[account(2, writable, name = "message_info")]
    #[account(3, writable, signer, name = "ncn_admin_info")]
    #[account(4, name = "system_program")]
    RequestMessage { keyword: String },

    /// Send Message
    #[account(0, name = "config_info")]
    #[account(1, name = "restaking_config_info")]
    #[account(2, name = "ncn_info")]
    #[account(3, name = "operator_info")]
    #[account(4, name = "vault_info")]
    #[account(5, name = "vault_ncn_ticket_info")]
    #[account(6, name = "ncn_vault_ticket_info")]
    #[account(7, name = "ncn_operator_state_info")]
    #[account(8, name = "vault_operator_delegation_info")]
    #[account(9, name = "message_info")]
    #[account(10, writable, name = "ballot_box_info")]
    #[account(11, writable, signer, name = "operator_voter_info")]
    SubmitMessage { message: String },
}
