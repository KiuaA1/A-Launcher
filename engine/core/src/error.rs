use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("invalid launch plan: {0}")]
    InvalidLaunchPlan(String),
    #[error("runtime unavailable: {0}")]
    RuntimeUnavailable(String),
    #[error("download failed: {0}")]
    DownloadFailed(String),
}
