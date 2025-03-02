use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::ncn_portal::NcnPortalCommands;

#[derive(Parser)]
#[command(author, version, about = "A CLI for managing ncn portal operations", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<ProgramCommand>,

    #[arg(long, global = true, help = "Path to the configuration file")]
    pub config_file: Option<PathBuf>,

    #[arg(long, global = true, help = "RPC URL to use")]
    pub rpc_url: Option<String>,

    #[arg(long, global = true, help = "Commitment level")]
    pub commitment: Option<String>,

    #[arg(long, global = true, help = "Restaking program ID")]
    pub ncn_portal_program_id: Option<String>,

    #[arg(long, global = true, help = "Keypair")]
    pub keypair: Option<String>,

    #[arg(long, global = true, help = "Verbose mode")]
    pub verbose: bool,

    #[arg(long, global = true, hide = true)]
    pub markdown_help: bool,
}

#[derive(Subcommand)]
pub enum ProgramCommand {
    /// Restaking program commands
    NcnPortal {
        #[command(subcommand)]
        action: NcnPortalCommands,
    },
}
