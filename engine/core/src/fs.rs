use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct StorageLayout {
    pub root: PathBuf,
    pub instances: PathBuf,
    pub runtimes: PathBuf,
    pub libraries: PathBuf,
    pub assets: PathBuf,
    pub downloads: PathBuf,
    pub natives: PathBuf,
    pub logs: PathBuf,
    pub cache: PathBuf,
}

impl StorageLayout {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        Self {
            instances: root.join("instances"), runtimes: root.join("runtimes"),
            libraries: root.join("libraries"), assets: root.join("assets"),
            downloads: root.join("downloads"), natives: root.join("natives"),
            logs: root.join("logs"), cache: root.join("cache"), root,
        }
    }

    pub fn instance_dir(&self, id: &str) -> PathBuf { self.instances.join(id) }
    pub fn instance_game_dir(&self, id: &str) -> PathBuf { self.instance_dir(id).join("game") }
    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        for path in [&self.root, &self.instances, &self.runtimes, &self.libraries, &self.assets,
            &self.downloads, &self.natives, &self.logs, &self.cache] { std::fs::create_dir_all(path)?; }
        Ok(())
    }

    pub fn is_within_root(&self, path: &Path) -> bool {
        path.strip_prefix(&self.root).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_is_deterministic() {
        let l = StorageLayout::new("/data/a-launcher");
        assert_eq!(l.instance_game_dir("survival"), PathBuf::from("/data/a-launcher/instances/survival/game"));
        assert!(l.is_within_root(&l.logs));
    }
}
