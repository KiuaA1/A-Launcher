use serde::{Deserialize, Serialize};
use crate::error::EngineError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionManifest { pub versions: Vec<VersionManifestEntry> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionManifestEntry { pub id: String, pub url: String, #[serde(default)] pub sha1: Option<String>, #[serde(default)] pub compliance_level: Option<u32> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionJson {
    pub id: String,
    pub main_class: String,
    #[serde(default)] pub inherits_from: Option<String>,
    #[serde(default)] pub java_version: Option<JavaVersionRequirement>,
    pub downloads: VersionDownloads,
    #[serde(default)] pub libraries: Vec<Library>,
    #[serde(default)] pub arguments: Option<Arguments>,
    #[serde(default)] pub minecraft_arguments: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaVersionRequirement { pub major_version: u32 }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDownloads { pub client: ArtifactDownload }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactDownload { pub sha1: String, pub size: u64, pub url: String, #[serde(default)] pub path: Option<String> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library { pub name: String, #[serde(default)] pub downloads: LibraryDownloads, #[serde(default)] pub rules: Vec<Rule> }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LibraryDownloads { #[serde(default)] pub artifact: Option<ArtifactDownload> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule { pub action: String, #[serde(default)] pub os: Option<OsRule> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsRule { #[serde(default)] pub name: Option<String>, #[serde(default)] pub arch: Option<String> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arguments { #[serde(default)] pub game: Vec<serde_json::Value>, #[serde(default)] pub jvm: Vec<serde_json::Value> }

pub fn parse_manifest_json(input: &str) -> Result<VersionManifest, EngineError> {
    serde_json::from_str(input).map_err(|e| EngineError::ManifestInvalid(e.to_string()))
}
pub fn parse_version_json(input: &str) -> Result<VersionJson, EngineError> {
    serde_json::from_str(input).map_err(|e| EngineError::ManifestInvalid(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn parses_manifest() { let x=parse_manifest_json(r#"{"versions":[{"id":"1.21.8","url":"https://example.invalid/1.json"}]}"#).unwrap(); assert_eq!(x.versions[0].id,"1.21.8"); }
}
