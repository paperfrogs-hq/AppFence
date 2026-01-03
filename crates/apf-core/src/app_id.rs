use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AppId {
    pub primary: String,
    pub binary_hash: Option<String>,
}

impl AppId {
    pub fn from_flatpak(flatpak_id: impl Into<String>) -> Self {
        Self {
            primary: flatpak_id.into(),
            binary_hash: None,
        }
    }

    pub fn from_desktop(desktop_id: impl Into<String>) -> Self {
        Self {
            primary: desktop_id.into(),
            binary_hash: None,
        }
    }

    pub fn from_executable(path: impl AsRef<Path>, include_hash: bool) -> std::io::Result<Self> {
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
        })
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
