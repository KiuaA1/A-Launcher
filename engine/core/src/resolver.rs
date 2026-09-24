use serde::{Deserialize, Serialize};
use crate::{error::EngineError, manifest::{ArtifactDownload, Library, Rule}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetOs { Linux, Windows, Osx }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetArch { X86, X86_64, Arm32, Arm64 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetPlatform { pub os: TargetOs, pub arch: TargetArch }

impl TargetPlatform {
    pub fn android_arm64() -> Self { Self { os: TargetOs::Linux, arch: TargetArch::Arm64 } }
    pub fn os_name(&self) -> &'static str {
        match self.os { TargetOs::Linux => "linux", TargetOs::Windows => "windows", TargetOs::Osx => "osx" }
    }
    pub fn arch_name(&self) -> &'static str {
        match self.arch { TargetArch::X86 => "x86", TargetArch::X86_64 => "x86_64", TargetArch::Arm32 => "arm", TargetArch::Arm64 => "aarch64" }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftArtifact {
    pub id: String,
    pub url: String,
    pub sha1: Option<String>,
    pub size: Option<u64>,
    #[serde(default)] pub path: Option<String>,
    #[serde(default)] pub classifier: Option<String>,
    #[serde(default)] pub native: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    pub minecraft_version: String,
    pub client_jar: MinecraftArtifact,
    pub libraries: Vec<MinecraftArtifact>,
    #[serde(default)] pub native_libraries: Vec<MinecraftArtifact>,
}

pub trait VersionResolver {
    fn resolve(&self, version: &str) -> Result<Resolution, EngineError>;
}

fn coordinate_parts(name: &str) -> Result<(&str, &str, &str), EngineError> {
    let mut p = name.split(':');
    let group = p.next().unwrap_or("");
    let artifact = p.next().unwrap_or("");
    let version = p.next().unwrap_or("");
    if group.is_empty() || artifact.is_empty() || version.is_empty() || p.next().is_some() {
        return Err(EngineError::ManifestInvalid(format!("invalid Maven coordinate: {name}")));
    }
    Ok((group, artifact, version))
}

pub fn maven_path(name: &str, classifier: Option<&str>, extension: &str) -> Result<String, EngineError> {
    let (group, artifact, version) = coordinate_parts(name)?;
    let mut filename = format!("{artifact}-{version}");
    if let Some(c) = classifier.filter(|c| !c.is_empty()) { filename.push('-'); filename.push_str(c); }
    filename.push('.');
    filename.push_str(extension);
    Ok(format!("{}/{}/{}/{}", group.replace('.', "/"), artifact, version, filename))
}

fn rule_matches(rule: &Rule, platform: TargetPlatform) -> bool {
    match &rule.os {
        None => true,
        Some(os) => {
            os.name.as_deref().map(|n| n == platform.os_name()).unwrap_or(true)
                && os.arch.as_deref().map(|a| a == platform.arch_name()).unwrap_or(true)
        }
    }
}

pub fn library_allowed(rules: &[Rule], platform: TargetPlatform) -> bool {
    if rules.is_empty() { return true; }
    let mut allowed = false;
    for rule in rules {
        if rule_matches(rule, platform) {
            allowed = rule.action == "allow";
        }
    }
    allowed
}

fn artifact_from_download(name: &str, download: &ArtifactDownload, classifier: Option<&str>, native: bool)
    -> MinecraftArtifact {
    MinecraftArtifact {
        id: name.to_string(), url: download.url.clone(), sha1: Some(download.sha1.clone()),
        size: Some(download.size), path: download.path.clone(), classifier: classifier.map(str::to_string), native,
    }
}

pub fn resolve_libraries(
    libraries: &[Library],
    platform: TargetPlatform,
) -> Result<(Vec<MinecraftArtifact>, Vec<MinecraftArtifact>), EngineError> {
    let mut normal = Vec::new();
    let mut natives = Vec::new();
    for library in libraries {
        if !library_allowed(&library.rules, platform) { continue; }

        if let Some(artifact) = &library.downloads.artifact {
            normal.push(artifact_from_download(&library.name, artifact, None, false));
        }

        if let Some(classifiers) = &library.downloads.classifiers {
            let candidates = [
                format!("natives-{}-{}", platform.os_name(), platform.arch_name()),
                format!("natives-{}", platform.os_name()),
            ];
            for classifier in candidates {
                if let Some(native) = classifiers.get(&classifier) {
                    natives.push(artifact_from_download(&library.name, native, Some(&classifier), true));
                    break;
                }
            }
        }
    }
    Ok((normal, natives))
}
