use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::error::EngineError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRequest {
    pub url: String,
    pub destination: PathBuf,
    pub expected_sha1: Option<String>,
    pub expected_size: Option<u64>,
}

pub trait DownloadEngine {
    fn enqueue(&self, request: DownloadRequest) -> Result<(), EngineError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadStatus {
    Cached,
    Downloaded,
}

pub trait DownloadTransport {
    fn fetch_to(&self, url: &str, destination: &std::path::Path) -> Result<(), EngineError>;
}

pub fn prepare_download(
    transport: &impl DownloadTransport,
    request: &DownloadRequest,
) -> Result<DownloadStatus, EngineError> {
    if request.url.trim().is_empty() {
        return Err(EngineError::DownloadFailed("download URL is empty".into()));
    }
    if let Some(parent) = request.destination.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| EngineError::DownloadFailed(format!("create destination: {e}")))?;
    }

    if request.destination.exists()
        && crate::cache::verify_file(
            &request.destination,
            request.expected_sha1.as_deref(),
            request.expected_size,
        )
        .map_err(|e| EngineError::DownloadFailed(format!("verify cache: {e}")))?
    {
        return Ok(DownloadStatus::Cached);
    }

    let partial = request.destination.with_extension("part");
    let _ = std::fs::remove_file(&partial);
    transport.fetch_to(&request.url, &partial)?;
    if !crate::cache::verify_file(
        &partial,
        request.expected_sha1.as_deref(),
        request.expected_size,
    )
    .map_err(|e| EngineError::DownloadFailed(format!("verify download: {e}")))?
    {
        let _ = std::fs::remove_file(&partial);
        return Err(EngineError::DownloadFailed(
            "download failed integrity verification".into(),
        ));
    }

    std::fs::rename(&partial, &request.destination)
        .map_err(|e| EngineError::DownloadFailed(format!("commit download: {e}")))?;
    Ok(DownloadStatus::Downloaded)
}

#[cfg(test)]
mod tests {
    use super::*;
    struct MockTransport;
    impl DownloadTransport for MockTransport {
        fn fetch_to(&self, _url: &str, destination: &std::path::Path) -> Result<(), EngineError> {
            std::fs::write(destination, b"hello")
                .map_err(|e| EngineError::DownloadFailed(e.to_string()))
        }
    }

    #[test]
    fn downloads_and_verifies_atomically() {
        let dir = std::env::temp_dir().join(format!("a-launcher-download-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let destination = dir.join("client.jar");
        let request = DownloadRequest {
            url: "https://example.invalid/client.jar".into(),
            destination: destination.clone(),
            expected_sha1: Some("aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d".into()),
            expected_size: Some(5),
        };
        assert_eq!(prepare_download(&MockTransport, &request).unwrap(), DownloadStatus::Downloaded);
        assert!(destination.exists());
        let _ = std::fs::remove_dir_all(dir);
    }
}


pub struct ReqwestDownloadTransport { client: reqwest::Client }

impl ReqwestDownloadTransport {
    pub fn new() -> Result<Self, EngineError> {
        let client = reqwest::Client::builder().build()
            .map_err(|e| EngineError::DownloadFailed(format!("create HTTP client: {e}")))?;
        Ok(Self { client })
    }

    pub async fn fetch_to_async(&self, url: &str, destination: &std::path::Path) -> Result<(), EngineError> {
        let response = self.client.get(url).send().await
            .map_err(|e| EngineError::DownloadFailed(format!("HTTP request: {e}")))?;
        let response = response.error_for_status()
            .map_err(|e| EngineError::DownloadFailed(format!("HTTP status: {e}")))?;
        let mut file = tokio::fs::File::create(destination).await
            .map_err(|e| EngineError::DownloadFailed(format!("create response file: {e}")))?;
        let mut stream = response.bytes_stream();
        use futures_util::StreamExt;
        while let Some(chunk) = stream.next().await {
            let chunk = chunk
                .map_err(|e| EngineError::DownloadFailed(format!("read response chunk: {e}")))?;
            tokio::io::AsyncWriteExt::write_all(&mut file, &chunk).await
                .map_err(|e| EngineError::DownloadFailed(format!("write response: {e}")))?;
        }
        tokio::io::AsyncWriteExt::flush(&mut file).await
            .map_err(|e| EngineError::DownloadFailed(format!("flush response: {e}")))?
    }
}


pub fn prepare_download_async<'a>(
    transport: &'a ReqwestDownloadTransport,
    request: &'a DownloadRequest,
) -> impl std::future::Future<Output = Result<DownloadStatus, EngineError>> + 'a {
    async move {
        if request.url.trim().is_empty() {
            return Err(EngineError::DownloadFailed("download URL is empty".into()));
        }
        if let Some(parent) = request.destination.parent() {
            tokio::fs::create_dir_all(parent).await
                .map_err(|e| EngineError::DownloadFailed(format!("create destination: {e}")))?;
        }
        if request.destination.exists()
            && crate::cache::verify_file(&request.destination, request.expected_sha1.as_deref(), request.expected_size)
                .map_err(|e| EngineError::DownloadFailed(format!("verify cache: {e}")))? {
            return Ok(DownloadStatus::Cached);
        }
        let partial = request.destination.with_extension("part");
        let _ = tokio::fs::remove_file(&partial).await;
        transport.fetch_to_async(&request.url, &partial).await?;
        if !crate::cache::verify_file(&partial, request.expected_sha1.as_deref(), request.expected_size)
            .map_err(|e| EngineError::DownloadFailed(format!("verify download: {e}")))? {
            let _ = tokio::fs::remove_file(&partial).await;
            return Err(EngineError::DownloadFailed("download failed integrity verification".into()));
        }
        tokio::fs::rename(&partial, &request.destination).await
            .map_err(|e| EngineError::DownloadFailed(format!("commit download: {e}")))?;
        Ok(DownloadStatus::Downloaded)
    }
}
