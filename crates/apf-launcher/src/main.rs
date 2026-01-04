
use clap::Parser;
use tracing::info;

#[derive(Parser)]
#[command(name = "apf-run")]
#[command(about = "Launch applications with AppFence protection")]
struct Args {
    #[arg(required = true)]
    command: Vec<String>,

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

    info!("Launching: {:?}", args.command);


    Ok(())
}
