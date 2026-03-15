
use clap::Parser;
use tracing::{info, error};
use std::process::Command;
// use std::os::unix::process::CommandExt;
use std::fs;
use anyhow::Context;

#[derive(Parser)]
#[command(name = "apf-run")]
#[command(about = "Launch applications with AppFence protection")]
struct Args {
    #[arg(required = true)]
    command: Vec<String>,

    #[arg(short, long)]
    verbose: bool,
    #[arg(long)]
    app_id: Option<String>,
    #[arg(long)]
    cgroup: Option<String>,
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
    // Register app before execution (stub: replace with DBus call)
    if let Some(app_id) = &args.app_id {
        info!("Registering app: {}", app_id);
        // TODO: DBus call to daemon for registration
    }

    // Setup cgroup v2 (stub: replace with actual cgroup logic)
    if let Some(cgroup_name) = &args.cgroup {
        let cgroup_path = format!("/sys/fs/cgroup/{}", cgroup_name);
        info!("Setting up cgroup: {}", cgroup_path);
        if let Err(e) = fs::create_dir_all(&cgroup_path) {
            error!("Failed to create cgroup: {}", e);
            // Failure rollback: exit
            return Err(anyhow::anyhow!("Failed to setup cgroup"));
        }
    }

    // Launch the command and track process tree
    let mut cmd = Command::new(&args.command[0]);
    for arg in &args.command[1..] {
        cmd.arg(arg);
    }
    // Optionally set cgroup (stub)
    // TODO: attach process to cgroup

    let mut child = cmd.spawn().context("Failed to launch application")?;
    info!("Launched PID: {}", child.id());

    // Wait for process and handle failures
    let status = child.wait().context("Failed to wait for child process")?;
    if !status.success() {
        error!("Application exited with failure: {:?}", status);
        // Failure rollback logic (stub)
        // TODO: unregister app, cleanup cgroup
        return Err(anyhow::anyhow!("Application failed"));
    }

    info!("Application exited successfully");
    Ok(())
}
