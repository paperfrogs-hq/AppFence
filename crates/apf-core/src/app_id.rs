use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AppOrigin {
    System,
    User,
    Flatpak,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AppId {
    pub primary: String,
    pub binary_hash: Option<String>,
    pub origin: AppOrigin,
    pub identity_version: u64, // Incremented on identity change
}

impl AppId {
    pub fn from_flatpak(flatpak_id: impl Into<String>) -> Self {
        Self {
            primary: flatpak_id.into(),
            binary_hash: None,
            origin: AppOrigin::Flatpak,
            identity_version: 1,
        }
    }

    pub fn from_desktop(desktop_id: impl Into<String>, system: bool) -> Self {
        Self {
            primary: desktop_id.into(),
            binary_hash: None,
            origin: if system { AppOrigin::System } else { AppOrigin::User },
            identity_version: 1,
        }
    }

    pub fn from_executable(path: impl AsRef<Path>, include_hash: bool, user_owned: bool) -> std::io::Result<Self> {
        let path = path.as_ref();
        let canonical = path.canonicalize()?;
        let primary = canonical.to_string_lossy().to_string();
        let binary_hash = if include_hash {
            Some(Self::compute_hash(&canonical)?)
        } else {
            None
        };
        Ok(Self {
            primary,
            binary_hash,
            origin: if user_owned { AppOrigin::User } else { AppOrigin::System },
            identity_version: 1,
        })
    }

    pub fn update_identity(&mut self, new_primary: String, new_hash: Option<String>, new_origin: AppOrigin) {
        if self.primary != new_primary || self.binary_hash != new_hash || self.origin != new_origin {
            self.primary = new_primary;
            self.binary_hash = new_hash;
            self.origin = new_origin;
            self.identity_version += 1;
        }
    }

    fn compute_hash(path: &Path) -> std::io::Result<String> {
        let content = std::fs::read(path)?;
        let hash = Sha256::digest(&content);
        Ok(hex::encode(hash))
    }

    pub fn verify_hash(&self, path: &Path) -> std::io::Result<bool> {
        match &self.binary_hash {
            Some(expected) => {
                let actual = Self::compute_hash(path)?;
                Ok(expected == &actual)
            }
            None => Ok(true), // No hash to verify
        }
    }

    pub fn display_name(&self) -> &str {
        &self.primary
    }
}

impl std::fmt::Display for AppId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.primary)
    }
}
