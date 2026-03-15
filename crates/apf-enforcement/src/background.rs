use anyhow::Result;
use tracing::{info, warn};

pub struct BackgroundBackend {
    pub allowed: bool,
}

impl BackgroundBackend {
    pub fn new(allowed: bool) -> Self {
        Self { allowed }
    }

    pub fn enforce_background_policy(&self, command: &[String]) -> Result<()> {
        info!("Enforcing background execution policy: allowed={}", self.allowed);
        warn!("Background execution enforcement is not fully implemented yet");
        // Launch command as-is for now
        Ok(())
    }
}
