use std::{fs::File, io, path::{Path, PathBuf}};
use zip::ZipArchive;
use crate::{error::EngineError, resolver::MinecraftArtifact};

fn safe_entry(name: &str) -> Option<PathBuf> {
    let p=Path::new(name);
    if p.is_absolute() || p.components().any(|c| matches!(c,std::path::Component::ParentDir)) { None } else { Some(p.to_path_buf()) }
}

pub fn extract_native_jar(jar: &Path, destination: &Path) -> Result<usize, EngineError> {
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
        if out.exists() { return Err(EngineError::DownloadFailed(format!("native extraction collision: {}", out.display()))); }
        if let Some(parent)=out.parent(){std::fs::create_dir_all(parent).map_err(|e|EngineError::DownloadFailed(e.to_string()))?;}
        let mut f=File::create(&out).map_err(|e|EngineError::DownloadFailed(e.to_string()))?;
        io::copy(&mut entry,&mut f).map_err(|e|EngineError::DownloadFailed(e.to_string()))?;
        count+=1;
    }
    Ok(count)
}

pub fn native_paths(artifacts: &[MinecraftArtifact], library_root: &Path) -> Vec<PathBuf> {
    artifacts.iter().filter_map(|a| a.path.as_ref().map(|p| library_root.join(p))).collect()
}
