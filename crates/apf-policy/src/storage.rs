
use anyhow::Result;
use apf_core::{app_id::AppId, types::{PermissionType, PromptDecision}};
use std::path::Path;

pub struct PolicyStorage;

impl PolicyStorage {
    pub fn new(_db_path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self)
    }

    pub async fn get_decision(&self, _app_id: &AppId, _permission: &PermissionType) -> Result<Option<PromptDecision>> {
        Ok(None)
    }

    pub async fn store_decision(&mut self, _app_id: &AppId, _permission: &PermissionType, _decision: PromptDecision) -> Result<()> {
        Ok(())
    }

    pub async fn get_app_policies(&self, _app_id: &AppId) -> Result<Vec<(PermissionType, PromptDecision)>> {
        Ok(Vec::new())
    }

    pub async fn delete_policy(&mut self, _app_id: &AppId, _permission: &PermissionType) -> Result<()> {
        Ok(())
    }

    pub async fn delete_app_policies(&mut self, _app_id: &AppId) -> Result<()> {
        Ok(())
    }

    pub async fn cleanup_expired(&mut self) -> Result<usize> {
        Ok(0)
    }
}
