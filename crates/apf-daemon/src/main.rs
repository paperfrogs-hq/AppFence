mod audit;
mod database;
mod dbus_service;
mod permissions;
mod polkit;
mod policy_engine;

use clap::Parser;
use tracing::{error, info};

use crate::audit::AuditLogger;
use crate::database::Database;
use crate::permissions::ApfPaths;
use crate::policy_engine::PolicyEngine;

#[derive(Parser)]
#[command(name = "apfd")]
#[command(about = "AppFence System Daemon")]
struct Args {
    #[arg(short, long, default_value = "/etc/appfence")]
    config_dir: String,

    #[arg(short, long)]
    verbose: bool,

    #[arg(long)]
    no_dbus: bool,
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
        .with_target(true)
        .with_thread_ids(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting AppFence System Daemon v{}", env!("CARGO_PKG_VERSION"));
    info!("Config directory: {}", args.config_dir);

    if let Err(e) = ApfPaths::verify_root_privileges() {
        error!("Privilege check failed: {}", e);
        error!("APF daemon must be run as root");
        return Err(e);
    }

    let paths = ApfPaths::default();
    paths.initialize()?;
    info!("Directory structure initialized");

    let mut db = Database::new(&paths.db_path)?;
    info!("Database initialized at: {}", paths.db_path.display());

    paths.secure_database_file()?;

    let expired_count = db.cleanup_expired_policies()?;
    if expired_count > 0 {
        info!("Cleaned up {} expired policies", expired_count);
    }

    let policy_engine = PolicyEngine::new(db);
    info!("Policy engine initialized");

    let audit_db = Database::new(&paths.db_path)?;
    let audit_logger = AuditLogger::new(audit_db);
    info!("Audit logger initialized");

    if !args.no_dbus {
        let _connection = dbus_service::start_dbus_service(policy_engine, audit_logger).await?;
        info!("DBus service started: org.apf.Daemon");

        info!("Daemon running, waiting for signals...");
        tokio::signal::ctrl_c().await?;
        
        info!("Received shutdown signal");
    } else {
        info!("Running in no-DBus mode");
        tokio::signal::ctrl_c().await?;
    }

    info!("Shutting down gracefully");
    Ok(())
}
