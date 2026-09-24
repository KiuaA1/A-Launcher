use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::EngineError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaRuntime { pub id: String, pub version: String, pub executable: PathBuf }

impl JavaRuntime {
    pub fn supports_major(&self, required: u32) -> bool { self.version.split('.').next().and_then(|v| v.parse::<u32>().ok()).map(|v| v == required).unwrap_or(false) }

    pub fn validate(&self) -> Result<(), EngineError> {
        if self.id.trim().is_empty() || self.version.trim().is_empty() { return Err(EngineError::RuntimeUnavailable("runtime id/version is empty".into())); }
        if self.executable.as_os_str().is_empty() { return Err(EngineError::RuntimeUnavailable("runtime executable is empty".into())); }
        Ok(())
    }
}
