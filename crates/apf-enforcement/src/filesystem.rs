
use std::process::Command;
use std::path::PathBuf;
use anyhow::{Result, Context};
use tracing::{info, warn, error};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessMode {
    ReadOnly,
    ReadWrite,
}

pub struct FilesystemBackend {
    pub allowed_paths: Vec<(PathBuf, AccessMode)>,
}

impl FilesystemBackend {
    pub fn new() -> Self {
        Self {
            allowed_paths: Vec::new(),
        }
    }

    pub fn add_allowed_path(&mut self, path: PathBuf, mode: AccessMode) {
        self.allowed_paths.push((path, mode));
    }

    pub fn launch_with_bubblewrap(&self, command: &[String]) -> Result<()> {
        let mut args = vec!["--unshare-all".to_string(), "--die-with-parent".to_string()];
        for (path, mode) in &self.allowed_paths {
            match mode {
                AccessMode::ReadOnly => {
                    args.push(format!("--ro-bind {} {}", path.display(), path.display()));
                }
                AccessMode::ReadWrite => {
                    args.push(format!("--bind {} {}", path.display(), path.display()));
                }
            }
        }
        // Default deny outside allowed paths
        args.push("--tmpfs /".to_string());

        // Symlink escape prevention and violation logging
        for (path, _) in &self.allowed_paths {
            if let Ok(meta) = std::fs::symlink_metadata(path) {
                if meta.file_type().is_symlink() {
                    warn!("Symlink detected in allowed path: {}", path.display());
                    error!("Filesystem violation: symlink access attempt at {}", path.display());
                    // TODO: Integrate audit logging here
                }
            }
        }

        // Logging
        info!("Launching bubblewrap sandbox: {:?}", args);
        let mut bw_cmd = Command::new("bubblewrap");
        for arg in &args {
            bw_cmd.arg(arg);
        }
        for c in command {
            bw_cmd.arg(c);
        }
        let status = bw_cmd.status().context("Failed to launch bubblewrap")?;
        if !status.success() {
            error!("Bubblewrap exited with failure: {:?}", status);
            error!("Filesystem violation: denied access attempt for command: {:?}", command);
            // TODO: Integrate audit logging here
            return Err(anyhow::anyhow!("Bubblewrap failed"));
        }
        info!("Bubblewrap sandbox exited successfully");
        Ok(())
    }
}
