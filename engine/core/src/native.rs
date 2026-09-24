use std::{fs::File, io, path::{Path, PathBuf}};
use zip::ZipArchive;
use crate::{error::EngineError, resolver::{maven_path, MinecraftArtifact}};

fn safe_entry(name: &str) -> Option<PathBuf> {
    let normalized = name.replace('\\\\', "/");
    let p=Path::new(&normalized);
    if p.is_absolute() || p.components().any(|c| matches!(c,std::path::Component::ParentDir)) { None } else { Some(p.to_path_buf()) }
}

pub fn prepare_native_directory(destination: &Path) -> Result<(), EngineError> {\n    if destination.exists() {\n        std::fs::remove_dir_all(destination)\n            .map_err(|e| EngineError::DownloadFailed(format!("clear native directory: {e}")))?;\n    }\n    std::fs::create_dir_all(destination)\n        .map_err(|e| EngineError::DownloadFailed(format!("create native directory: {e}")))?;\n    Ok(())\n}\n\npub fn extract_native_jar(jar: &Path, destination: &Path) -> Result<usize, EngineError> {
    let file=File::open(jar).map_err(|e| EngineError::DownloadFailed(format!("open native JAR: {e}")))?;
    let mut archive=ZipArchive::new(file).map_err(|e| EngineError::DownloadFailed(format!("read native JAR: {e}")))?;
    std::fs::create_dir_all(destination).map_err(|e| EngineError::DownloadFailed(format!("create native directory: {e}")))?;
    let mut count=0;
    for i in 0..archive.len() {
        let mut entry=archive.by_index(i).map_err(|e| EngineError::DownloadFailed(format!("read native entry: {e}")))?;
        let name=entry.name().to_string();
        if entry.is_dir() { continue; }
        let is_native = name.ends_with(".so") || name.ends_with(".dll") || name.ends_with(".dylib") || name.ends_with(".jnilib");
        if !is_native { continue; }
        let rel=match safe_entry(&name) { Some(p)=>p, None=>continue };
        let filename=match rel.file_name().and_then(|n|n.to_str()) { Some(n)=>n, None=>continue };
        if filename.is_empty() { continue; }
        let out=destination.join(rel);
        if out.exists() {
            return Err(EngineError::DownloadFailed(format!("native extraction collision: {}", out.display())));
        }
        let root = std::fs::canonicalize(destination)
            .map_err(|e| EngineError::DownloadFailed(format!("resolve native destination: {e}")))?;
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| EngineError::DownloadFailed(e.to_string()))?;
            let parent_resolved = std::fs::canonicalize(parent)
                .map_err(|e| EngineError::DownloadFailed(format!("resolve native parent: {e}")))?;
            if !parent_resolved.starts_with(&root) {
                return Err(EngineError::DownloadFailed("native extraction escaped destination".into()));
            }
        }
        let mut f=File::create(&out).map_err(|e|EngineError::DownloadFailed(e.to_string()))?;
        io::copy(&mut entry,&mut f).map_err(|e|EngineError::DownloadFailed(e.to_string()))?;
        count+=1;
    }
    Ok(count)
}

pub fn native_paths(artifacts: &[MinecraftArtifact], library_root: &Path) -> Vec<PathBuf> {
    artifacts.iter().filter_map(|a| {
        let rel = a.path.clone().or_else(|| maven_path(&a.id, a.classifier.as_deref(), "jar").ok());
        rel.map(|p| library_root.join(p))
    }).collect()
}
