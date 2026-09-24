use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{
    error::EngineError,
    manifest::VersionJson,
    resolver::{MinecraftArtifact, Resolution},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedLaunchArtifacts {
    pub client_jar: MinecraftArtifact,
    pub libraries: Vec<MinecraftArtifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchPreparation {
    pub minecraft_version: String,
    pub main_class: String,
    pub java_major_version: Option<u32>,
    pub artifacts: ResolvedLaunchArtifacts,
    pub game_directory: PathBuf,
}

impl LaunchPreparation {
    pub fn from_metadata(
        version: &VersionJson,
        resolution: Resolution,
        game_directory: impl Into<PathBuf>,
    ) -> Result<Self, EngineError> {
        if version.id != resolution.minecraft_version {
            return Err(EngineError::InvalidLaunchPlan(
                "version metadata and resolution disagree".into(),
            ));
        }
        if version.main_class.trim().is_empty() {
            return Err(EngineError::InvalidLaunchPlan(
                "main class is empty".into(),
            ));
        }
        Ok(Self {
            minecraft_version: version.id.clone(),
            main_class: version.main_class.clone(),
            java_major_version: version.java_version.as_ref().map(|v| v.major_version),
            artifacts: ResolvedLaunchArtifacts {
                client_jar: resolution.client_jar,
                libraries: resolution.libraries,
            },
            game_directory: game_directory.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mojang::resolution_from_version_json;

    #[test]
    fn builds_preparation_contract() {
        let j = r#"{"id":"1.21.8","main_class":"net.minecraft.client.main.Main","downloads":{"client":{"sha1":"abc","size":3,"url":"https://x/client.jar"}},"libraries":[]}"#;
        let v = crate::manifest::parse_version_json(j).unwrap();
        let r = resolution_from_version_json(j).unwrap();
        let p = LaunchPreparation::from_metadata(&v, r, "/game").unwrap();
        assert_eq!(p.main_class, "net.minecraft.client.main.Main");
    }
}
