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
pub struct LibraryDownloads { #[serde(default)] pub artifact: Option<ArtifactDownload>, #[serde(default)] pub classifiers: Option<std::collections::HashMap<String, ArtifactDownload>> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule { pub action: String, #[serde(default)] pub os: Option<OsRule>, #[serde(default)] pub features: std::collections::HashMap<String,bool> }
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

pub fn merge_version_json(parent: &VersionJson, child: &VersionJson) -> VersionJson {
    let mut merged = parent.clone();
    merged.id = child.id.clone();
    merged.inherits_from = child.inherits_from.clone();

    if !child.main_class.trim().is_empty() {
        merged.main_class = child.main_class.clone();
    }
    if child.java_version.is_some() {
        merged.java_version = child.java_version.clone();
    }

    // A child version is expected to provide its own client artifact.
    // Libraries are inherited additively, preserving parent-before-child order.
    merged.downloads = child.downloads.clone();

    let mut libraries = parent.libraries.clone();
    libraries.extend(child.libraries.iter().cloned());
    merged.libraries = libraries;

    // Argument arrays are inherited additively. Child entries run after
    // parent entries, matching the layered nature of inherited profiles.
    match (&parent.arguments, &child.arguments) {
        (Some(p), Some(c)) => {
            let mut arguments = p.clone();
            arguments.jvm.extend(c.jvm.iter().cloned());
            arguments.game.extend(c.game.iter().cloned());
            merged.arguments = Some(arguments);
        }
        (None, Some(c)) => merged.arguments = Some(c.clone()),
        (Some(p), None) => merged.arguments = Some(p.clone()),
        (None, None) => merged.arguments = None,
    }

    // Legacy versions expose one flat argument string, so a child value
    // replaces the inherited value when present.
    if child.minecraft_arguments.is_some() {
        merged.minecraft_arguments = child.minecraft_arguments.clone();
    } else {
        merged.minecraft_arguments = parent.minecraft_arguments.clone();
    }

    merged
}

#[cfg(test)]
mod merge_tests {
    use super::*;
    use serde_json::json;

    fn version(id: &str, args: Option<Arguments>, libraries: Vec<Library>) -> VersionJson {
        VersionJson {
            id: id.into(),
            main_class: "Main".into(),
            inherits_from: None,
            java_version: None,
            downloads: VersionDownloads {
                client: ArtifactDownload {
                    sha1: "sha".into(), size: 1, url: "https://example.invalid/client.jar".into(), path: None,
                },
            },
            libraries,
            arguments: args,
            minecraft_arguments: None,
        }
    }

    fn library(name: &str) -> Library {
        Library {
            name: name.into(),
            downloads: LibraryDownloads::default(),
            rules: Vec::new(),
        }
    }

    #[test]
    fn merges_libraries_and_argument_arrays() {
        let parent = version(
            "parent",
            Some(Arguments { game: vec![json!("--parent")], jvm: vec![json!("-Dparent=true")] }),
            vec![library("parent-lib")],
        );
        let child = version(
            "child",
            Some(Arguments { game: vec![json!("--child")], jvm: vec![json!("-Dchild=true")] }),
            vec![library("child-lib")],
        );

        let merged = merge_version_json(&parent, &child);
        assert_eq!(merged.libraries.len(), 2);
        assert_eq!(merged.arguments.as_ref().unwrap().game.len(), 2);
        assert_eq!(merged.arguments.as_ref().unwrap().jvm.len(), 2);
    }
}
