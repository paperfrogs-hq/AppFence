//! AppFence Session Agent
//!
//! User-space service responsible for:
//! - Displaying permission prompts
//! - Desktop integration
//! - User notifications

use clap::Parser;
use tracing::{info, error};

#[derive(Parser)]
#[command(name = "apf-agent")]
#[command(about = "AppFence Session Agent")]
struct Args {
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Initialize logging
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(if args.verbose {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting AppFence Session Agent");

    // TODO: Initialize agent components
    // - DBus client connection
    // - Notification system
    // - Prompt UI

    info!("Agent initialized successfully");

    // Keep running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down");

    Ok(())
}
