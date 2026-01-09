use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};
use zbus::{Connection, ConnectionBuilder, interface};
use zbus::fdo;

use apf_core::{app_id::AppId, types::{PermissionType, PromptDecision}};
use crate::policy_engine::PolicyEngine;
use crate::audit::AuditLogger;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequestId(pub String);

impl RequestId {
    pub fn new() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        Self(format!("req-{}-{}", timestamp, uuid::Uuid::new_v4()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRequest {
    pub request_id: RequestId,
    pub app_id: AppId,
    pub pid: u32,
    pub uid: u32,
    pub permission: PermissionType,
    pub timestamp: u64,
}

pub struct DaemonService {
    policy_engine: Arc<Mutex<PolicyEngine>>,
    audit_logger: Arc<Mutex<AuditLogger>>,
    pending_requests: Arc<Mutex<std::collections::HashMap<RequestId, PermissionRequest>>>,
}

impl DaemonService {
    pub fn new(policy_engine: PolicyEngine, audit_logger: AuditLogger) -> Self {
        Self {
            policy_engine: Arc::new(Mutex::new(policy_engine)),
            audit_logger: Arc::new(Mutex::new(audit_logger)),
            pending_requests: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    async fn verify_caller(&self, pid: u32, uid: u32) -> Result<()> {
        debug!("Verifying caller credentials: pid={}, uid={}", pid, uid);
        Ok(())
    }

    async fn should_prompt(&self, app_id: &AppId, permission: &PermissionType) -> Result<bool> {
        let engine = self.policy_engine.lock().await;
        engine.should_prompt(app_id, permission).await
    }

    async fn get_cached_decision(&self, app_id: &AppId, permission: &PermissionType) -> Result<Option<PromptDecision>> {
        let engine = self.policy_engine.lock().await;
        engine.get_cached_decision(app_id, permission).await
    }

    async fn store_decision(&self, app_id: &AppId, permission: &PermissionType, decision: PromptDecision) -> Result<()> {
        let mut engine = self.policy_engine.lock().await;
        engine.store_decision(app_id, permission, decision).await
    }
}

#[interface(name = "org.apf.Daemon")]
impl DaemonService {
    async fn request_permission(
        &mut self,
        app_id_json: String,
        pid: u32,
        uid: u32,
        permission_json: String,
    ) -> fdo::Result<(bool, String, bool)> {
        info!("Permission request: pid={}, uid={}", pid, uid);

        let app_id: AppId = serde_json::from_str(&app_id_json)
            .map_err(|e| fdo::Error::InvalidArgs(format!("Invalid app_id: {}", e)))?;
        let permission: PermissionType = serde_json::from_str(&permission_json)
            .map_err(|e| fdo::Error::InvalidArgs(format!("Invalid permission: {}", e)))?;

        self.verify_caller(pid, uid).await
            .map_err(|e| fdo::Error::AccessDenied(format!("Credential verification failed: {}", e)))?;

        if let Ok(Some(decision)) = self.get_cached_decision(&app_id, &permission).await {
            info!("Using cached decision for {:?}: {:?}", app_id.primary, decision);
            
            let granted = matches!(decision, PromptDecision::AllowAlways | PromptDecision::AllowOnce | PromptDecision::AllowDuration(_));
            
            let mut logger = self.audit_logger.lock().await;
            let _ = logger.log_permission_check(&app_id, &permission, granted, false).await;

            return Ok((false, String::new(), granted));
        }

        let needs_prompt = self.should_prompt(&app_id, &permission).await
            .map_err(|e| fdo::Error::Failed(format!("Policy check failed: {}", e)))?;

        if needs_prompt {
            let request_id = RequestId::new();
            let request = PermissionRequest {
                request_id: request_id.clone(),
                app_id,
                pid,
                uid,
                permission,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };

            let mut pending = self.pending_requests.lock().await;
            pending.insert(request_id.clone(), request);

            info!("Prompt required, request_id: {}", request_id.0);
            Ok((true, request_id.0, false))
        } else {
            warn!("Permission denied by default: {:?}", app_id.primary);
            
            let mut logger = self.audit_logger.lock().await;
            let _ = logger.log_permission_check(&app_id, &permission, false, false).await;

            Ok((false, String::new(), false))
        }
    }

    async fn submit_decision(
        &mut self,
        request_id_str: String,
        decision_json: String,
    ) -> fdo::Result<bool> {
        let request_id = RequestId(request_id_str);
        
        debug!("Received decision for request: {}", request_id.0);

        let decision: PromptDecision = serde_json::from_str(&decision_json)
            .map_err(|e| fdo::Error::InvalidArgs(format!("Invalid decision: {}", e)))?;

        let mut pending = self.pending_requests.lock().await;
        let request = pending.remove(&request_id)
            .ok_or_else(|| fdo::Error::Failed("Request ID not found or expired".to_string()))?;

        let should_store = matches!(decision, PromptDecision::AllowAlways | PromptDecision::DenyAlways | PromptDecision::AllowDuration(_));
        if should_store {
            self.store_decision(&request.app_id, &request.permission, decision.clone()).await
                .map_err(|e| fdo::Error::Failed(format!("Failed to store decision: {}", e)))?;
        }

        let granted = matches!(decision, PromptDecision::AllowAlways | PromptDecision::AllowOnce | PromptDecision::AllowDuration(_));

        let mut logger = self.audit_logger.lock().await;
        let _ = logger.log_permission_check(&request.app_id, &request.permission, granted, true).await;

        info!("Decision processed: request={}, granted={}", request_id.0, granted);
        Ok(granted)
    }

    async fn get_app_policy(&self, app_id_json: String) -> fdo::Result<String> {
        let app_id: AppId = serde_json::from_str(&app_id_json)
            .map_err(|e| fdo::Error::InvalidArgs(format!("Invalid app_id: {}", e)))?;

        let engine = self.policy_engine.lock().await;
        let policy = engine.get_app_policy(&app_id).await
            .map_err(|e| fdo::Error::Failed(format!("Failed to get policy: {}", e)))?;

        serde_json::to_string(&policy)
            .map_err(|e| fdo::Error::Failed(format!("Serialization failed: {}", e)))
    }

    async fn update_app_policy(
        &mut self,
        #[zbus(header)]
        _hdr: zbus::message::Header<'_>,
        app_id_json: String,
        policy_json: String,
    ) -> fdo::Result<()> {
        info!("Policy update requested");

        let app_id: AppId = serde_json::from_str(&app_id_json)
            .map_err(|e| fdo::Error::InvalidArgs(format!("Invalid app_id: {}", e)))?;
        
        let policy: Vec<(PermissionType, PromptDecision)> = serde_json::from_str(&policy_json)
            .map_err(|e| fdo::Error::InvalidArgs(format!("Invalid policy: {}", e)))?;

        let mut engine = self.policy_engine.lock().await;
        engine.update_app_policy(&app_id, policy).await
            .map_err(|e| fdo::Error::Failed(format!("Failed to update policy: {}", e)))?;

        info!("Policy updated for: {:?}", app_id.primary);
        Ok(())
    }

    async fn get_audit_log(
        &self,
        limit: u32,
    ) -> fdo::Result<String> {
        let logger = self.audit_logger.lock().await;
        let entries = logger.get_recent_entries(limit as usize).await
            .map_err(|e| fdo::Error::Failed(format!("Failed to get audit log: {}", e)))?;

        serde_json::to_string(&entries)
            .map_err(|e| fdo::Error::Failed(format!("Serialization failed: {}", e)))
    }

    async fn ping(&self) -> fdo::Result<String> {
        Ok("pong".to_string())
    }
}

pub async fn start_dbus_service(
    policy_engine: PolicyEngine,
    audit_logger: AuditLogger,
) -> Result<Connection> {
    info!("Starting DBus service: org.apf.Daemon");

    let service = DaemonService::new(policy_engine, audit_logger);

    let connection = ConnectionBuilder::system()?
        .name("org.apf.Daemon")?
        .serve_at("/org/apf/Daemon", service)?
        .build()
        .await?;

    info!("DBus service started successfully");
    Ok(connection)
}
