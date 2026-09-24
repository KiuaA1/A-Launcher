use crate::{
    error::EngineError,
    manifest::{parse_manifest_json, parse_version_json, VersionJson},
    resolver::{MinecraftArtifact, Resolution, VersionResolver},
};
use std::path::{Path, PathBuf};

pub const VERSION_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Debug, Clone)]
pub struct MojangResolver { pub manifest_json: String }

impl MojangResolver {
    pub fn from_manifest_json(json: impl Into<String>) -> Result<Self, EngineError> {
        let json = json.into();
        parse_manifest_json(&json)?;
        Ok(Self { manifest_json: json })
    }
    pub fn version_json_from_str(json: &str) -> Result<VersionJson, EngineError> {
        parse_version_json(json)
    }

    pub fn resolve_with_transport<T: crate::download::DownloadTransport>(
        &self,
        version: &str,
        transport: &T,
        metadata_root: &Path,
        platform: crate::resolver::TargetPlatform,
    ) -> Result<Resolution, EngineError> {
        let manifest = parse_manifest_json(&self.manifest_json)?;
        let entry = manifest.versions.iter().find(|v| v.id == version)
            .ok_or_else(|| EngineError::VersionNotFound(version.into()))?;

        let destination = Self::cached_version_path(metadata_root, version);
        if !destination.exists() {
            if let Some(parent) = destination.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| EngineError::DownloadFailed(format!("create metadata cache: {e}")))?;
            }
            transport.fetch_to(&entry.url, &destination)?;
        }

        let parsed = self.resolve_inheritance_chain(version, transport, metadata_root)?;

        if parsed.id != version {
            return Err(EngineError::ManifestInvalid(format!(
                "version metadata id {} does not match requested version {version}",
                parsed.id
            )));
        }

        resolution_from_version_json_value(&parsed, platform)
    }
    pub fn resolve_inheritance_chain<T: crate::download::DownloadTransport>(
    &self,
    version: &str,
    transport: &T,
    metadata_root: &Path,
) -> Result<VersionJson, EngineError> {
    let manifest = parse_manifest_json(&self.manifest_json)?;
    let mut current = version.to_string();
    let mut chain = Vec::new();
    let mut visited = std::collections::HashSet::new();

    loop {
        if !visited.insert(current.clone()) {
            return Err(EngineError::ManifestInvalid(format!("version inheritance cycle detected at {current}")));
        }
        let entry = manifest.versions.iter().find(|v| v.id == current)
            .ok_or_else(|| EngineError::VersionNotFound(current.clone()))?;
        let destination = Self::cached_version_path(metadata_root, &current);
        if !destination.exists() {
            if let Some(parent) = destination.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| EngineError::DownloadFailed(format!("create inheritance cache: {e}")))?;
            }
            transport.fetch_to(&entry.url, &destination)?;
        }
        let json = std::fs::read_to_string(&destination)
            .map_err(|e| EngineError::ManifestInvalid(format!("read inherited metadata: {e}")))?;
        let parsed = parse_version_json(&json)?;
        let parent = parsed.inherits_from.clone();
        chain.push(parsed);
        match parent {
            Some(parent_id) => current = parent_id,
            None => break,
        }
    }

    let mut merged = chain.pop().ok_or_else(|| EngineError::VersionNotFound(version.into()))?;
    while let Some(child) = chain.pop() {
        merged = crate::manifest::merge_version_json(&merged, &child);
    }
    Ok(merged)
}

}

impl VersionResolver for MojangResolver {
    fn resolve(&self, version: &str) -> Result<Resolution, EngineError> {
        let manifest = parse_manifest_json(&self.manifest_json)?;
        let entry = manifest.versions.iter().find(|v| v.id == version)
            .ok_or_else(|| EngineError::VersionNotFound(version.into()))?;
        Err(EngineError::ResolutionRequiresMetadata(entry.url.clone()))
    }
}

pub fn resolve_inheritance(child_json: &str, parent_json: &str) -> Result<VersionJson, EngineError> {
    let child = parse_version_json(child_json)?;
    let parent = parse_version_json(parent_json)?;
    Ok(crate::manifest::merge_version_json(&parent, &child))
}

pub fn resolution_from_version_json(json: &str) -> Result<Resolution, EngineError> {
    let v = parse_version_json(json)?;
    resolution_from_version_json_value(&v, crate::resolver::TargetPlatform::android_arm64())
}
fn resolution_from_version_json_value(
    v: &VersionJson,
    platform: crate::resolver::TargetPlatform,
) -> Result<Resolution, EngineError> {

pub fn resolution_from_version_json_for(
    json: &str,
    platform: crate::resolver::TargetPlatform,
) -> Result<Resolution, EngineError> {
    let client = MinecraftArtifact {
        id: v.id.clone(), url: v.downloads.client.url, sha1: Some(v.downloads.client.sha1),
        size: Some(v.downloads.client.size), path: v.downloads.client.path,
        classifier: None, native: false,
    };
    let (libraries, native_libraries) =
        crate::resolver::resolve_libraries(&v.libraries, platform)?;
    Ok(Resolution {
        minecraft_version: v.id,
        client_jar: client,
        libraries,
        native_libraries,
    })
}

pub struct MojangMetadataClient<T> {
    pub transport: T,
}

impl<T: crate::download::DownloadTransport> MojangMetadataClient<T> {
    pub fn new(transport: T) -> Self { Self { transport } }

    pub fn download_manifest(&self, destination: &Path) -> Result<(), EngineError> {
        self.transport.fetch_to(VERSION_MANIFEST_URL, destination)
    }

    pub fn download_version_json(
        &self, version_url: &str, destination: &Path
    ) -> Result<(), EngineError> {
        self.transport.fetch_to(version_url, destination)
    }

    pub fn cached_manifest_path(root: &Path) -> PathBuf {
        root.join("metadata").join("version_manifest_v2.json")
    }

    pub fn cached_version_path(root: &Path, version: &str) -> PathBuf {
        root.join("metadata").join("versions").join(format!("{version}.json"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resolves_version_json() {
        let j = r#"{"id":"1.21.8","main_class":"net.minecraft.client.main.Main","downloads":{"client":{"sha1":"abc","size":3,"url":"https://x/client.jar"}},"libraries":[]}"#;
        let r = resolution_from_version_json(j).unwrap();
        assert_eq!(r.minecraft_version, "1.21.8");
    }
}
