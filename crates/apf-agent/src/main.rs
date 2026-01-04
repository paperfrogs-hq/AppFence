
use clap::Parser;
use tracing::info;

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

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(if args.verbose {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting AppFence Session Agent");


    info!("Agent initialized successfully");

    tokio::signal::ctrl_c().await?;
    info!("Shutting down");

    Ok(())
}
