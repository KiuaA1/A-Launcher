use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("invalid launch plan: {0}")]
    InvalidLaunchPlan(String),
    #[error("runtime unavailable: {0}")]
    RuntimeUnavailable(String),
    #[error("download failed: {0}")]
    DownloadFailed(String),
    #[error("manifest is invalid: {0}")]
    ManifestInvalid(String),
    #[error("version not found: {0}")]
    VersionNotFound(String),
    #[error("resolution requires version metadata: {0}")]
    ResolutionRequiresMetadata(String),
}
