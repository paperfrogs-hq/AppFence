
use std::process::Command;
use anyhow::{Result, Context};
use tracing::{info, error};

pub struct SandboxBackend;

impl SandboxBackend {
    pub fn new() -> Self {
        Self
    }

    pub fn enforce_sandbox_policy(&self, command: &[String]) -> Result<()> {
        let args = vec!["--unshare-all", "--die-with-parent"];
        info!("Launching sandbox: {:?}", args);
        let mut sb_cmd = Command::new("bubblewrap");
        for arg in &args {
            sb_cmd.arg(arg);
        }
        for c in command {
            sb_cmd.arg(c);
        }
        let status = sb_cmd.status().context("Failed to launch sandbox")?;
        if !status.success() {
            error!("Sandbox exited with failure: {:?}", status);
            return Err(anyhow::anyhow!("Sandbox failed"));
        }
        info!("Sandbox exited successfully");
        Ok(())
    }
}
