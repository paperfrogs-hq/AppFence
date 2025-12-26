use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Permission categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PermissionType {
    Network(NetworkLevel),
    Filesystem(FilesystemAccess),
    Device(DeviceType),
    Clipboard,
    BackgroundExecution,
    Autostart,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NetworkLevel {
    None,
    Lan,
    Internet,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FilesystemAccess {
    pub path: PathBuf,
    pub mode: AccessMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AccessMode {
    ReadOnly,
    ReadWrite,
    Deny,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DeviceType {
    Microphone,
    Camera,
    Screen,
    Usb,
}

/// Enforcement strength levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnforcementStrength {
    Strong,   // Sandbox + namespace + cgroup
    Medium,   // OS-level restrictions
    Weak,     // Audit and warning only
}

/// Prompt decision
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PromptDecision {
    AllowOnce,
    AllowAlways,
    DenyOnce,
    DenyAlways,
    AllowDuration(std::time::Duration),
}

/// Process information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub uid: u32,
    pub gid: u32,
    pub executable: PathBuf,
    pub cmdline: Vec<String>,
}
