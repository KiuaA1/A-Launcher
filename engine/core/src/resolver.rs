use serde::{Deserialize, Serialize};
use crate::error::EngineError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftArtifact { pub id: String, pub url: String, pub sha1: Option<String>, pub size: Option<u64> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution { pub minecraft_version: String, pub client_jar: MinecraftArtifact, pub libraries: Vec<MinecraftArtifact> }

pub trait VersionResolver { fn resolve(&self, version: &str) -> Result<Resolution, EngineError>; }
