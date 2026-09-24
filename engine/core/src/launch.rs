use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error::EngineError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchPlan {
    pub minecraft_version: String,
    pub java_executable: String,
    pub game_directory: String,
    pub classpath: Vec<String>,
    pub jvm_args: Vec<String>,
    pub game_args: Vec<String>,
    #[serde(default)]
    pub environment: HashMap<String, String>,
}

impl LaunchPlan {
    pub fn validate(&self) -> Result<(), EngineError> {
        if self.minecraft_version.trim().is_empty() { return Err(EngineError::InvalidLaunchPlan("minecraft version is empty".into())); }
        if self.java_executable.trim().is_empty() { return Err(EngineError::InvalidLaunchPlan("java executable is empty".into())); }
        if self.game_directory.trim().is_empty() { return Err(EngineError::InvalidLaunchPlan("game directory is empty".into())); }
        if self.classpath.is_empty() { return Err(EngineError::InvalidLaunchPlan("classpath is empty".into())); }
        Ok(())
    }
}
