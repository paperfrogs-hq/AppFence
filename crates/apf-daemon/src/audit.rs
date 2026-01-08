
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};

use apf_core::{app_id::AppId, types::PermissionType};
use crate::database::Database;

pub struct AuditLogger {
    db: Arc<Mutex<Database>>,
}

impl AuditLogger {
    pub fn new(db: Database) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
        }
    }

    pub async fn log_permission_check(
        &mut self,
        app_id: &AppId,
        permission: &PermissionType,
        granted: bool,
        was_prompted: bool,
    ) -> Result<()> {
        let pid = std::process::id();
        let uid = nix::unistd::getuid().as_raw();

        let mut db = self.db.lock().await;
        db.log_audit(
            app_id,
            pid,
            uid,
            permission,
            None,
            granted,
            was_prompted,
        )?;

        if granted {
            info!(
                app_id = %app_id.primary,
                permission = ?permission,
                prompted = was_prompted,
                "Permission granted"
            );
        } else {
            warn!(
                app_id = %app_id.primary,
                permission = ?permission,
                prompted = was_prompted,
                "Permission denied"
            );
        }

        Ok(())
    }


    pub async fn get_recent_entries(&self, limit: usize) -> Result<Vec<AuditEntryView>> {
        let db = self.db.lock().await;
        let entries = db.get_audit_entries(limit)?;

        let mut views = Vec::new();
        for entry in entries {
            views.push(AuditEntryView {
                timestamp: entry.timestamp,
                app_id: entry.app_id,
                pid: entry.pid,
                uid: entry.uid,
                permission: entry.permission_json,
                decision: entry.decision_json,
                granted: entry.granted,
                was_prompted: entry.was_prompted,
            });
        }

        Ok(views)
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntryView {
    pub timestamp: i64,
    pub app_id: String,
    pub pid: u32,
    pub uid: u32,
    pub permission: String,
    pub decision: String,
    pub granted: bool,
    pub was_prompted: bool,
}

