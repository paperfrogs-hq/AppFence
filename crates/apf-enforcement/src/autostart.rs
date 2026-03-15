use anyhow::Result;
use tracing::{info, warn};

pub struct AutostartBackend {
    pub allowed: bool,
}

impl AutostartBackend {
    pub fn new(allowed: bool) -> Self {
        Self { allowed }
    }

    pub fn enforce_autostart_policy(&self, command: &[String]) -> Result<()> {
        info!("Enforcing autostart policy: allowed={}", self.allowed);
        warn!("Autostart enforcement is not fully implemented yet");
        // Launch command as-is for now
        Ok(())
    }
}
