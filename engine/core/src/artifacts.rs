use std::path::{Path, PathBuf};
use crate::{download::{prepare_download, DownloadRequest, DownloadStatus, DownloadTransport}, error::EngineError, fs::StorageLayout, resolver::{maven_path, MinecraftArtifact, Resolution}};

#[derive(Debug, Clone, Copy, Default)]
pub struct ArtifactDownloadSummary { pub cached: usize, pub downloaded: usize }

impl ArtifactDownloadSummary {
    fn add(&mut self, status: DownloadStatus) {
        match status { DownloadStatus::Cached => self.cached += 1, DownloadStatus::Downloaded => self.downloaded += 1 }
    }
    pub fn total(&self) -> usize { self.cached + self.downloaded }
}

fn safe_relative_path(path: &str) -> Option<PathBuf> {
    let p = Path::new(path);
    if p.is_absolute() || p.components().any(|c| matches!(c, std::path::Component::ParentDir)) { return None; }
    Some(p.to_path_buf())
}

fn destination_for(layout: &StorageLayout, artifact: &MinecraftArtifact) -> Result<PathBuf, EngineError> {
    let derived = if artifact.path.is_none() {
        Some(maven_path(&artifact.id, artifact.classifier.as_deref(), "jar")
            .map_err(|e| EngineError::DownloadFailed(format!("derive Maven path for {}: {e}", artifact.id)))?)
    } else { None };
    let rel = artifact.path.as_deref().or(derived.as_deref())
        .and_then(safe_relative_path)
        .ok_or_else(|| EngineError::DownloadFailed(format!("artifact {} has no safe repository path", artifact.id)))?;
    Ok(layout.libraries.join(rel))
}

fn download_artifact<T: DownloadTransport>(
    transport: &T, layout: &StorageLayout, artifact: &MinecraftArtifact
) -> Result<DownloadStatus, EngineError> {
    let destination = destination_for(layout, artifact)?;
    prepare_download(transport, &DownloadRequest {
        url: artifact.url.clone(), destination,
        expected_sha1: artifact.sha1.clone(), expected_size: artifact.size,
    })
}

pub fn download_resolution<T: DownloadTransport>(
    transport: &T, layout: &StorageLayout, resolution: &Resolution
) -> Result<ArtifactDownloadSummary, EngineError> {
    layout.ensure_dirs().map_err(|e| EngineError::DownloadFailed(format!("prepare storage: {e}")))?;
    let mut summary = ArtifactDownloadSummary::default();

    let client_destination = layout.instance_game_dir(&resolution.minecraft_version).join("client.jar");
    summary.add(prepare_download(transport, &DownloadRequest {
        url: resolution.client_jar.url.clone(), destination: client_destination,
        expected_sha1: resolution.client_jar.sha1.clone(), expected_size: resolution.client_jar.size,
    })?);

    for artifact in resolution.libraries.iter().chain(resolution.native_libraries.iter()) {
        summary.add(download_artifact(transport, layout, artifact)?);
    }
    Ok(summary)
}
