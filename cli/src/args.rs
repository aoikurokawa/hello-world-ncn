use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum ProgramCommand {
    /// Admin
    InitializeConfig,

    /// Admin
    InitializeBallotBox,

    /// Admin
    RequestMessage { keyword: String },
}

#[derive(Parser)]
#[command(author, version, about = "A CLI for creating and managing the MEV Tip Distribution NCN", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: ProgramCommand,

    #[arg(
        long,
        global = true,
        env = "RPC_URL",
        default_value = "https://api.devnet.solana.com",
        help = "RPC URL to use"
    )]
    pub rpc_url: String,

    #[arg(
        long,
        global = true,
        env = "COMMITMENT",
        default_value = "confirmed",
        help = "Commitment level"
    )]
    pub commitment: String,

    #[arg(
        long,
        global = true,
        env = "HELLO_WORLD_NCN_PROGRAM_ID",
        default_value_t = hello_world_ncn_program::id().to_string(),
        help = "Tip router program ID"
    )]
    pub hello_world_ncn_program_id: String,

    #[arg(
        long,
        global = true,
        env = "RESTAKING_PROGRAM_ID",
        default_value_t = jito_restaking_program::id().to_string(),
        help = "Restaking program ID"
    )]
    pub restaking_program_id: String,

    #[arg(long, global = true, env = "NCN", help = "NCN Account Address")]
    pub ncn: Option<String>,

    #[arg(long, global = true, env = "KEYPAIR_PATH", help = "keypair path")]
    pub keypair_path: Option<String>,
}

#[rustfmt::skip]
impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\nHello World NCN CLI Configuration")?;
        writeln!(f, "═══════════════════════════════════════")?;

        // Network Configuration
        writeln!(f, "\n📡 Network Settings:")?;
        writeln!(f, "  • RPC URL:     {}", self.rpc_url)?;
        writeln!(f, "  • Commitment:  {}", self.commitment)?;

        // Program IDs
        writeln!(f, "\n🔑 Program IDs:")?;
        writeln!(f, "  • Hello World NCN:   {}", self.hello_world_ncn_program_id)?;
        writeln!(f, "  • Restaking:         {}", self.restaking_program_id)?;

        // Solana Settings
        writeln!(f, "\n◎  Solana Settings:")?;
        writeln!(f, "  • Keypair Path:  {}", self.keypair_path.as_deref().unwrap_or("Not Set"))?;
        writeln!(f, "  • NCN:  {}", self.ncn.as_deref().unwrap_or("Not Set"))?;

        writeln!(f, "\n")?;

        Ok(())
    }
}
