use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

pub struct ApfPaths {
    pub data_dir: PathBuf,
    pub db_path: PathBuf,
    pub config_dir: PathBuf,
    pub log_dir: PathBuf,
}

impl Default for ApfPaths {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("/var/lib/apf"),
            db_path: PathBuf::from("/var/lib/apf/apf.db"),
            config_dir: PathBuf::from("/etc/appfence"),
            log_dir: PathBuf::from("/var/log/apf"),
        }
    }
}

impl ApfPaths {
    pub fn initialize(&self) -> Result<()> {
        info!("Initializing APF directory structure");

        self.create_secure_directory(&self.data_dir, 0o700)?;
        self.create_secure_directory(&self.config_dir, 0o755)?;
        self.create_secure_directory(&self.log_dir, 0o700)?;

        if let Some(parent) = self.db_path.parent() {
            self.create_secure_directory(parent, 0o700)?;
        }

        info!("APF directory structure initialized successfully");
        Ok(())
    }

    fn create_secure_directory(&self, path: &Path, mode: u32) -> Result<()> {
        if path.exists() {
            debug!("Directory already exists: {}", path.display());
            
            let metadata = fs::metadata(path)
                .context(format!("Failed to get metadata for {}", path.display()))?;
            
            let current_mode = metadata.permissions().mode() & 0o777;
            if current_mode != mode {
                warn!(
                    "Fixing permissions for {}: {:o} -> {:o}",
                    path.display(),
                    current_mode,
                    mode
                );
                let permissions = fs::Permissions::from_mode(mode);
                fs::set_permissions(path, permissions)
                    .context(format!("Failed to set permissions for {}", path.display()))?;
            }
        } else {
            info!("Creating directory: {}", path.display());
            fs::create_dir_all(path)
                .context(format!("Failed to create directory {}", path.display()))?;
            
            let permissions = fs::Permissions::from_mode(mode);
            fs::set_permissions(path, permissions)
                .context(format!("Failed to set permissions for {}", path.display()))?;
        }

        Ok(())
    }

    pub fn verify_root_privileges() -> Result<()> {
        let uid = nix::unistd::getuid();
        
        if !uid.is_root() {
            anyhow::bail!(
                "APF daemon must run as root (current UID: {})",
                uid.as_raw()
            );
        }

        debug!("Root privilege verification passed");
        Ok(())
    }

    pub fn secure_database_file(&self) -> Result<()> {
        if self.db_path.exists() {
            info!("Securing database file: {}", self.db_path.display());
            
            let permissions = fs::Permissions::from_mode(0o600);
            fs::set_permissions(&self.db_path, permissions)
                .context("Failed to set database file permissions")?;
            
            debug!("Database file permissions set to 0600");
        }

        Ok(())
    }

}