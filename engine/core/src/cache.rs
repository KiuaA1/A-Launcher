use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use sha1::{Digest, Sha1};

#[derive(Debug, Clone)]
pub struct CacheEntry { pub path: PathBuf, pub sha1: Option<String>, pub size: Option<u64> }

pub fn verify_file(path: &Path, expected_sha1: Option<&str>, expected_size: Option<u64>) -> io::Result<bool> {
    let metadata = std::fs::metadata(path)?;
    if let Some(size) = expected_size { if metadata.len() != size { return Ok(false); } }
    if let Some(expected) = expected_sha1 {
        let mut file = File::open(path)?; let mut hasher = Sha1::new(); let mut buf=[0u8; 1024*64];
        loop { let n=file.read(&mut buf)?; if n==0 { break; } hasher.update(&buf[..n]); }
        let actual = format!("{:x}", hasher.finalize());
        if !actual.eq_ignore_ascii_case(expected) { return Ok(false); }
    }
    Ok(true)
}

#[derive(Debug, Clone)]
pub struct DownloadCache { pub root: PathBuf }
impl DownloadCache {
    pub fn new(root: impl Into<PathBuf>) -> Self { Self { root: root.into() } }
    pub fn path_for(&self, key: &str) -> PathBuf { self.root.join(key) }
    pub fn valid(&self, key: &str, sha1: Option<&str>, size: Option<u64>) -> io::Result<bool> {
        verify_file(&self.path_for(key), sha1, size)
    }
}

#[cfg(test)]
mod tests {
    use super::*; use std::io::Write;
    #[test]
    fn verifies_size_and_sha1() {
        let dir=std::env::temp_dir().join(format!("a-launcher-cache-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap(); let p=dir.join("x"); let mut f=File::create(&p).unwrap(); f.write_all(b"hello").unwrap();
        assert!(verify_file(&p, Some("aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"), Some(5)).unwrap());
        let _=std::fs::remove_dir_all(dir);
    }
}
