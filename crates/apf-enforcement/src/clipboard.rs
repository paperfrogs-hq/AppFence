use anyhow::Result;
use tracing::{info, warn};

pub struct ClipboardBackend {
    pub allowed: bool,
}

impl ClipboardBackend {
    pub fn new(allowed: bool) -> Self {
        Self { allowed }
    }

    pub fn enforce_clipboard_policy(&self, command: &[String]) -> Result<()> {
        info!("Enforcing clipboard policy: allowed={}", self.allowed);
        warn!("Clipboard enforcement is not fully implemented yet");
        // Launch command as-is for now
        Ok(())
    }
}
