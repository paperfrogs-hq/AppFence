use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApfError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Permission denied")]
    PermissionDenied,

    #[error("Invalid application ID: {0}")]
    InvalidAppId(String),

    #[error("Policy not found for app: {0}")]
    PolicyNotFound(String),

    #[error("DBus error: {0}")]
    DBus(String),

    #[error("Enforcement failed: {0}")]
    EnforcementFailed(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, ApfError>;
