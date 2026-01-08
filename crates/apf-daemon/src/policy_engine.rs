
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use apf_core::{app_id::AppId, types::{PermissionType, PromptDecision}};
use crate::database::Database;

pub struct PolicyEngine {
    db: Arc<Mutex<Database>>,
}

impl PolicyEngine {
    pub fn new(db: Database) -> Self {
        Self { 
            db: Arc::new(Mutex::new(db))
        }
    }

    pub async fn should_prompt(&self, app_id: &AppId, permission: &PermissionType) -> Result<bool> {
        let db = self.db.lock().await;
        let decision = db.get_policy(app_id, permission)?;
        
        if decision.is_some() {
            return Ok(false); // No prompt needed
        }

        Ok(Self::is_sensitive(permission))
    }

    fn is_sensitive(permission: &PermissionType) -> bool {
        use apf_core::types::{NetworkLevel, DeviceType};
        
        matches!(permission,
            PermissionType::Network(NetworkLevel::Internet) |
            PermissionType::Network(NetworkLevel::Lan) |
            PermissionType::Device(DeviceType::Camera) |
            PermissionType::Device(DeviceType::Microphone) |
            PermissionType::Device(DeviceType::Screen) |
            PermissionType::Clipboard |
            PermissionType::Autostart
        )
    }

    pub async fn get_cached_decision(&self, app_id: &AppId, permission: &PermissionType) -> Result<Option<PromptDecision>> {
        let db = self.db.lock().await;
        db.get_policy(app_id, permission)
    }

    pub async fn store_decision(&mut self, app_id: &AppId, permission: &PermissionType, decision: PromptDecision) -> Result<()> {
        let mut db = self.db.lock().await;
        db.store_policy(app_id, permission, &decision)
    }

    pub async fn get_app_policy(&self, app_id: &AppId) -> Result<Vec<(PermissionType, PromptDecision)>> {
        let db = self.db.lock().await;
        db.get_app_policies(app_id)
    }

    pub async fn update_app_policy(&mut self, app_id: &AppId, policies: Vec<(PermissionType, PromptDecision)>) -> Result<()> {
        let mut db = self.db.lock().await;
        for (permission, decision) in policies {
            db.store_policy(app_id, &permission, &decision)?;
        }
        Ok(())
    }
}
