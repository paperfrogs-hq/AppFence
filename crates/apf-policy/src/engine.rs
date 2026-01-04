
use anyhow::Result;
use apf_core::{app_id::AppId, types::{PermissionType, PromptDecision}};
use crate::storage::PolicyStorage;

pub struct PolicyEngine {
    storage: PolicyStorage,
}

impl PolicyEngine {
    pub fn new(storage: PolicyStorage) -> Self {
        Self { storage }
    }

    pub async fn should_prompt(&self, app_id: &AppId, permission: &PermissionType) -> Result<bool> {
        let decision = self.storage.get_decision(app_id, permission).await?;
        
        if decision.is_some() {
            return Ok(false); // Don't prompt if we have a decision
        }

        let requires_prompt = Self::is_sensitive_permission(permission);
        
        Ok(requires_prompt)
    }

    fn is_sensitive_permission(permission: &PermissionType) -> bool {
        use apf_core::types::{NetworkLevel, DeviceType};
        
        match permission {
            PermissionType::Network(NetworkLevel::Internet) => true,
            PermissionType::Network(NetworkLevel::Lan) => true,
            PermissionType::Device(DeviceType::Camera) => true,
            PermissionType::Device(DeviceType::Microphone) => true,
            PermissionType::Device(DeviceType::Screen) => true,
            PermissionType::Filesystem(fs) => {
                let path_str = fs.path.to_string_lossy();
                path_str.contains("/home") || 
                path_str.contains("/Documents") || 
                path_str.contains("/Downloads")
            }
            PermissionType::Clipboard => true,
            PermissionType::Autostart => true,
            _ => false,
        }
    }

    pub async fn get_cached_decision(&self, app_id: &AppId, permission: &PermissionType) -> Result<Option<PromptDecision>> {
        self.storage.get_decision(app_id, permission).await
    }

    pub async fn store_decision(&mut self, app_id: &AppId, permission: &PermissionType, decision: PromptDecision) -> Result<()> {
        self.storage.store_decision(app_id, permission, decision).await
    }

    pub async fn get_app_policy(&self, app_id: &AppId) -> Result<Vec<(PermissionType, PromptDecision)>> {
        self.storage.get_app_policies(app_id).await
    }

    pub async fn update_app_policy(&mut self, app_id: &AppId, policies: Vec<(PermissionType, PromptDecision)>) -> Result<()> {
        for (permission, decision) in policies {
            self.storage.store_decision(app_id, &permission, decision).await?;
        }
        
        Ok(())
    }

    pub async fn delete_policy(&mut self, app_id: &AppId, permission: &PermissionType) -> Result<()> {
        self.storage.delete_policy(app_id, permission).await
    }

    pub async fn delete_app_policy(&mut self, app_id: &AppId) -> Result<()> {
        self.storage.delete_app_policies(app_id).await
    }

    pub async fn cleanup_expired(&mut self) -> Result<usize> {
        self.storage.cleanup_expired().await
    }

    pub async fn evaluate_permission(&self, app_id: &AppId, permission: &PermissionType) -> Result<Option<bool>> {
        if let Some(decision) = self.storage.get_decision(app_id, permission).await? {
            let granted = matches!(
                decision,
                PromptDecision::AllowAlways | PromptDecision::AllowOnce | PromptDecision::AllowDuration(_)
            );
            Ok(Some(granted))
        } else {
            Ok(None) // No decision found
        }
    }
}
