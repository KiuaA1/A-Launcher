use std::path::{Path, PathBuf};
use crate::{error::EngineError, resolver::{maven_path, MinecraftArtifact}};

#[derive(Debug, Clone)]
pub struct Classpath {
    pub entries: Vec<PathBuf>,
}

impl Classpath {
    pub fn as_separator_string(&self) -> String {
        self.as_separator_string_for(std::path::MAIN_SEPARATOR)
    }
    pub fn as_separator_string_for(&self, separator: char) -> String {
        self.entries.iter().map(|p| p.to_string_lossy().into_owned()).collect::<Vec<_>>().join(&separator.to_string())
    }
    pub fn as_classpath_string(&self, os_name: &str) -> String {
        let separator = if os_name == "windows" { ';' } else { ':' };
        self.as_separator_string_for(separator)
    }
}

pub fn build_classpath(
    libraries: &[MinecraftArtifact],
    client_jar: &MinecraftArtifact,
    library_root: &Path,
    client_path: &Path,
) -> Result<Classpath, EngineError> {
    let mut entries = Vec::with_capacity(libraries.len() + 1);
    for lib in libraries {
        let derived;
        let rel = match lib.path.as_deref() {
            Some(path) => path,
            None => {
                derived = maven_path(&lib.id, lib.classifier.as_deref(), "jar")
                    .map_err(|e| EngineError::InvalidLaunchPlan(format!("derive path for {}: {e}", lib.id)))?;
                &derived
            }
        };
        let path = Path::new(rel);
        if path.is_absolute() || path.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
            return Err(EngineError::InvalidLaunchPlan(format!("unsafe library path: {rel}")));
        }
        let full = library_root.join(path);
        if !full.is_file() { return Err(EngineError::InvalidLaunchPlan(format!("library missing: {}", full.display()))); }
        entries.push(full);
    }
    if !client_path.is_file() { return Err(EngineError::InvalidLaunchPlan(format!("client JAR missing: {}", client_path.display()))); }
    let _ = client_jar;
    entries.push(client_path.to_path_buf());
    Ok(Classpath { entries })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_missing_library() {
        let a=MinecraftArtifact{id:"x".into(),url:"u".into(),sha1:None,size:None,path:Some("../x".into()),classifier:None,native:false};
        let c=MinecraftArtifact{id:"c".into(),url:"u".into(),sha1:None,size:None,path:None,classifier:None,native:false};
        assert!(build_classpath(&[a],&c,Path::new("/libs"),Path::new("/client")).is_err());
    }
}
