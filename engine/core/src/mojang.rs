use crate::{error::EngineError, manifest::{parse_manifest_json, parse_version_json, VersionJson}, resolver::{MinecraftArtifact, Resolution, VersionResolver}};

pub const VERSION_MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Debug, Clone)]
pub struct MojangResolver { pub manifest_json: String }
impl MojangResolver {
    pub fn from_manifest_json(json: impl Into<String>) -> Result<Self, EngineError> {
        let json=json.into(); parse_manifest_json(&json)?; Ok(Self{manifest_json:json})
    }
    pub fn version_json_from_str(json: &str) -> Result<VersionJson, EngineError> { parse_version_json(json) }
}
impl VersionResolver for MojangResolver {
    fn resolve(&self, version: &str) -> Result<Resolution, EngineError> {
        let manifest=parse_manifest_json(&self.manifest_json)?;
        let entry=manifest.versions.iter().find(|v| v.id==version).ok_or_else(|| EngineError::VersionNotFound(version.into()))?;
        Err(EngineError::ResolutionRequiresMetadata(entry.url.clone()))
    }
}

pub fn resolution_from_version_json(json: &str) -> Result<Resolution, EngineError> {
    resolution_from_version_json_for(json, crate::resolver::TargetPlatform::android_arm64())
}

pub fn resolution_from_version_json_for(
    json: &str,
    platform: crate::resolver::TargetPlatform,
) -> Result<Resolution, EngineError> {
    let v = parse_version_json(json)?;
    let client = MinecraftArtifact {
        id: v.id.clone(), url: v.downloads.client.url, sha1: Some(v.downloads.client.sha1),
        size: Some(v.downloads.client.size), path: v.downloads.client.path,
        classifier: None, native: false,
    };
    let (libraries, native_libraries) = crate::resolver::resolve_libraries(&v.libraries, platform)?;
    Ok(Resolution { minecraft_version: v.id, client_jar: client, libraries, native_libraries })
}

#[cfg(test)]
mod tests { use super::*; #[test] fn resolves_version_json() { let j=r#"{"id":"1.21.8","main_class":"net.minecraft.client.main.Main","downloads":{"client":{"sha1":"abc","size":3,"url":"https://x/client.jar"}},"libraries":[]}"#; let r=resolution_from_version_json(j).unwrap(); assert_eq!(r.minecraft_version,"1.21.8"); assert_eq!(r.client_jar.size,Some(3)); } }
