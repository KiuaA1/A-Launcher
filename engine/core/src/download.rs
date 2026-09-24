use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::error::EngineError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRequest { pub url: String, pub destination: PathBuf, pub expected_sha1: Option<String>, pub expected_size: Option<u64> }

pub trait DownloadEngine { fn enqueue(&self, request: DownloadRequest) -> Result<(), EngineError>; }
