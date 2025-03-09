use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;

use hello_world_ncn_cli::{args::Args, handler::CliHandler};
use log::info;

#[tokio::main]
#[allow(clippy::large_stack_frames)]
async fn main() -> Result<()> {
    dotenv().ok();

    let args: Args = Args::parse();

    info!("\n{}", args);

    let handler = CliHandler::from_args(&args).await?;
    handler.handle(args.command).await?;

    Ok(())
}
