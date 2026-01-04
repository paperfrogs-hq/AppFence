
use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use std::path::{Path, PathBuf};
use tracing::{debug, info};

use apf_core::{app_id::AppId, types::{PermissionType, PromptDecision}};

pub struct Database {
    conn: Connection,
    #[allow(dead_code)] // Used by path() getter method
    path: PathBuf,
}

impl Database {
    pub fn new(db_path: impl AsRef<Path>) -> Result<Self> {
        let path = db_path.as_ref().to_path_buf();
        
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create database directory")?;
        }

        info!("Opening database at: {}", path.display());
        let conn = Connection::open(&path)
            .context("Failed to open database")?;

        conn.execute("PRAGMA foreign_keys = ON", [])
            .context("Failed to enable foreign keys")?;

        let mut db = Self { conn, path };
        db.initialize_schema()?;
        
        Ok(db)
    }

    fn initialize_schema(&mut self) -> Result<()> {
        info!("Initializing database schema");

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS applications (
                app_id TEXT PRIMARY KEY NOT NULL,
                binary_hash TEXT,
                first_seen INTEGER NOT NULL,
                last_seen INTEGER NOT NULL
            )",
            [],
        ).context("Failed to create applications table")?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS policies (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                app_id TEXT NOT NULL,
                permission_type TEXT NOT NULL,
                decision TEXT NOT NULL,
                expires_at INTEGER,
                created_at INTEGER NOT NULL,
                FOREIGN KEY (app_id) REFERENCES applications(app_id) ON DELETE CASCADE,
                UNIQUE(app_id, permission_type)
            )",
            [],
        ).context("Failed to create policies table")?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS audit_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                app_id TEXT NOT NULL,
                pid INTEGER NOT NULL,
                uid INTEGER NOT NULL,
                permission_type TEXT NOT NULL,
                decision TEXT NOT NULL,
                granted INTEGER NOT NULL,
                was_prompted INTEGER NOT NULL,
                FOREIGN KEY (app_id) REFERENCES applications(app_id)
            )",
            [],
        ).context("Failed to create audit_log table")?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_policies_app_id ON policies(app_id)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_audit_timestamp ON audit_log(timestamp DESC)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_audit_app_id ON audit_log(app_id)",
            [],
        )?;

        info!("Database schema initialized successfully");
        Ok(())
    }

    pub fn register_application(&mut self, app_id: &AppId) -> Result<()> {
        let now = current_timestamp();
        
        self.conn.execute(
            "INSERT INTO applications (app_id, binary_hash, first_seen, last_seen)
             VALUES (?1, ?2, ?3, ?3)
             ON CONFLICT(app_id) DO UPDATE SET
                binary_hash = ?2,
                last_seen = ?3",
            params![
                &app_id.primary,
                &app_id.binary_hash,
                now,
            ],
        ).context("Failed to register application")?;

        debug!("Registered application: {}", app_id.primary);
        Ok(())
    }

    pub fn store_policy(&mut self, app_id: &AppId, permission: &PermissionType, decision: &PromptDecision) -> Result<()> {
        self.register_application(app_id)?;

        let permission_json = serde_json::to_string(permission)?;
        let decision_json = serde_json::to_string(decision)?;
        let now = current_timestamp();
        
        let expires_at = match decision {
            PromptDecision::AllowDuration(duration) => {
                Some(now + duration.as_secs() as i64)
            }
            _ => None,
        };

        self.conn.execute(
            "INSERT INTO policies (app_id, permission_type, decision, expires_at, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(app_id, permission_type) DO UPDATE SET
                decision = ?3,
                expires_at = ?4,
                created_at = ?5",
            params![
                &app_id.primary,
                permission_json,
                decision_json,
                expires_at,
                now,
            ],
        ).context("Failed to store policy")?;

        debug!("Stored policy for {} - {:?}", app_id.primary, permission);
        Ok(())
    }

    pub fn get_policy(&self, app_id: &AppId, permission: &PermissionType) -> Result<Option<PromptDecision>> {
        let permission_json = serde_json::to_string(permission)?;
        let now = current_timestamp();

        let mut stmt = self.conn.prepare(
            "SELECT decision, expires_at FROM policies
             WHERE app_id = ?1 AND permission_type = ?2"
        )?;

        let result = stmt.query_row(
            params![&app_id.primary, permission_json],
            |row| {
                let decision_json: String = row.get(0)?;
                let expires_at: Option<i64> = row.get(1)?;
                Ok((decision_json, expires_at))
            },
        );

        match result {
            Ok((decision_json, expires_at)) => {
                if let Some(expiry) = expires_at {
                    if now > expiry {
                        debug!("Policy expired for {}", app_id.primary);
                        return Ok(None);
                    }
                }

                let decision: PromptDecision = serde_json::from_str(&decision_json)?;
                debug!("Found cached policy for {}", app_id.primary);
                Ok(Some(decision))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn get_app_policies(&self, app_id: &AppId) -> Result<Vec<(PermissionType, PromptDecision)>> {
        let mut stmt = self.conn.prepare(
            "SELECT permission_type, decision, expires_at FROM policies
             WHERE app_id = ?1"
        )?;

        let now = current_timestamp();
        let rows = stmt.query_map(params![&app_id.primary], |row| {
            let permission_json: String = row.get(0)?;
            let decision_json: String = row.get(1)?;
            let expires_at: Option<i64> = row.get(2)?;
            Ok((permission_json, decision_json, expires_at))
        })?;

        let mut policies = Vec::new();
        for row in rows {
            let (perm_json, dec_json, expires_at) = row?;
            
            if let Some(expiry) = expires_at {
                if now > expiry {
                    continue;
                }
            }

            let permission: PermissionType = serde_json::from_str(&perm_json)?;
            let decision: PromptDecision = serde_json::from_str(&dec_json)?;
            policies.push((permission, decision));
        }

        Ok(policies)
    }


    pub fn log_audit(&mut self, 
        app_id: &AppId, 
        pid: u32,
        uid: u32,
        permission: &PermissionType, 
        decision: Option<&PromptDecision>,
        granted: bool,
        was_prompted: bool
    ) -> Result<()> {
        self.register_application(app_id)?;

        let permission_json = serde_json::to_string(permission)?;
        let decision_json = decision.map(|d| serde_json::to_string(d).ok()).flatten()
            .unwrap_or_else(|| "null".to_string());
        let now = current_timestamp();

        self.conn.execute(
            "INSERT INTO audit_log 
             (timestamp, app_id, pid, uid, permission_type, decision, granted, was_prompted)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                now,
                &app_id.primary,
                pid,
                uid,
                permission_json,
                decision_json,
                granted as i32,
                was_prompted as i32,
            ],
        )?;

        debug!("Logged audit entry for {}", app_id.primary);
        Ok(())
    }

    pub fn get_audit_entries(&self, limit: usize) -> Result<Vec<AuditEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT timestamp, app_id, pid, uid, permission_type, decision, granted, was_prompted
             FROM audit_log
             ORDER BY timestamp DESC
             LIMIT ?1"
        )?;

        let rows = stmt.query_map(params![limit], |row| {
            Ok(AuditEntry {
                timestamp: row.get(0)?,
                app_id: row.get(1)?,
                pid: row.get(2)?,
                uid: row.get(3)?,
                permission_json: row.get(4)?,
                decision_json: row.get(5)?,
                granted: row.get::<_, i32>(6)? != 0,
                was_prompted: row.get::<_, i32>(7)? != 0,
            })
        })?;

        let mut entries = Vec::new();
        for row in rows {
            entries.push(row?);
        }

        Ok(entries)
    }

    pub fn cleanup_expired_policies(&mut self) -> Result<usize> {
        let now = current_timestamp();
        let count = self.conn.execute(
            "DELETE FROM policies WHERE expires_at IS NOT NULL AND expires_at < ?1",
            params![now],
        )?;

        if count > 0 {
            info!("Cleaned up {} expired policies", count);
        }

        Ok(count)
    }

}

#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub timestamp: i64,
    pub app_id: String,
    pub pid: u32,
    pub uid: u32,
    pub permission_json: String,
    pub decision_json: String,
    pub granted: bool,
    pub was_prompted: bool,
}

fn current_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

