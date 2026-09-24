use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EngineInfo {
    pub engine_version: String,
    pub api_version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstanceSummary {
    pub id: String,
    pub name: String,
    pub minecraft_version: String,
    pub loader: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LaunchRequest {
    pub instance_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LaunchStatus {
    pub session_id: String,
    pub state: LaunchStateDto,
    pub message: String,
    pub exit_code: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LaunchStateDto {
    Preparing,
    Resolving,
    Downloading,
    Starting,
    Running,
    Exited,
    Failed,
    Cancelled,
}

impl From<crate::LaunchState> for LaunchStateDto {
    fn from(value: crate::LaunchState) -> Self {
        match value {
            crate::LaunchState::Preparing => Self::Preparing,
            crate::LaunchState::Resolving => Self::Resolving,
            crate::LaunchState::Downloading => Self::Downloading,
            crate::LaunchState::Starting => Self::Starting,
            crate::LaunchState::Running => Self::Running,
            crate::LaunchState::Exited => Self::Exited,
            crate::LaunchState::Failed => Self::Failed,
            crate::LaunchState::Cancelled => Self::Cancelled,
        }
    }
}

pub const API_VERSION: u32 = 1;
