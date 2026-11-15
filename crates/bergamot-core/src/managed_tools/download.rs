use reqwest::header::LOCATION;
use reqwest::StatusCode;
use sha2::{Digest, Sha256};
use std::io;
use std::path::Path;
use thiserror::Error;
use tokio::io::AsyncWriteExt;
use tokio_util::sync::CancellationToken;

pub const ALLOWED_HOSTS: &[&str] = &[
    "github.com",
    "www.gyan.dev",
    "release-assets.githubusercontent.com",
    "objects.githubusercontent.com",
    "johnvansickle.com",
    "www.johnvansickle.com",
    "ffmpeg.martin-riedl.de",
    "evermeet.cx",
    "download.imagemagick.org",
];

#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("Untrusted host '{0}'; not in managed tools allowlist")]
    UntrustedHost(String),
    #[error("Insecure protocol '{0}'; HTTPS is required")]
    InsecureProtocol(String),
    #[error("Too many redirects (exceeded limit of 10)")]
    TooManyRedirects,
    #[error("HTTP error: status {0}")]
    HttpStatus(StatusCode),
    #[error("Download exceeded maximum allowed size of {0} bytes")]
    TooLarge(u64),
    #[error("Download was cancelled")]
    Cancelled,
    #[error("SHA-256 checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },
}

pub fn is_allowed_host(host: &str) -> bool {
    host == "127.0.0.1" || host == "localhost" || ALLOWED_HOSTS.contains(&host)
}

fn validate_download_url(url: &reqwest::Url) -> Result<(), DownloadError> {
    let host = url
        .host_str()
        .ok_or_else(|| DownloadError::InvalidUrl(url.to_string()))?;
    let is_local = host == "127.0.0.1" || host == "localhost";
    if !is_local && url.scheme() != "https" {
        return Err(DownloadError::InsecureProtocol(url.scheme().to_owned()));
    }
    if !is_allowed_host(host) {
        return Err(DownloadError::UntrustedHost(host.to_owned()));
    }
    Ok(())
}

pub async fn download_file<F>(
    initial_url: &str,
    expected_sha256: &str,
    destination: &Path,
    max_bytes: u64,
    on_progress: F,
    cancel: &CancellationToken,
) -> Result<(), DownloadError>
where
    F: Fn(u64, Option<u64>) + Send + Sync + 'static,
{
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let mut current_url =
        reqwest::Url::parse(initial_url).map_err(|e| DownloadError::InvalidUrl(e.to_string()))?;
    let mut redirects = 0;

    let response = loop {
        if cancel.is_cancelled() {
            return Err(DownloadError::Cancelled);
        }

        // Validate every URL immediately before requesting it, including redirect targets.
        validate_download_url(&current_url)?;

        let request = client.get(current_url.clone()).send();
        let resp = tokio::select! {
            _ = cancel.cancelled() => return Err(DownloadError::Cancelled),
            res = request => res?,
        };

        let status = resp.status();
        if status.is_redirection() {
            redirects += 1;
            if redirects > 10 {
                return Err(DownloadError::TooManyRedirects);
            }
            let location = resp
                .headers()
                .get(LOCATION)
                .ok_or_else(|| {
                    DownloadError::InvalidUrl("Missing Location header in redirect".into())
                })?
                .to_str()
                .map_err(|e| DownloadError::InvalidUrl(e.to_string()))?;

            current_url = current_url
                .join(location)
                .map_err(|e| DownloadError::InvalidUrl(e.to_string()))?;
            continue;
        }

        if !status.is_success() {
            return Err(DownloadError::HttpStatus(status));
        }

        break resp;
    };

    let total_bytes = response.content_length();
    let mut stream = response;
    let mut file = tokio::fs::File::create(destination).await?;
    let mut hasher = Sha256::new();
    let mut downloaded: u64 = 0;

    on_progress(0, total_bytes);

    while let Some(chunk) = stream.chunk().await? {
        if cancel.is_cancelled() {
            drop(file);
            let _ = tokio::fs::remove_file(destination).await;
            return Err(DownloadError::Cancelled);
        }

        downloaded += chunk.len() as u64;
        if downloaded > max_bytes {
            drop(file);
            let _ = tokio::fs::remove_file(destination).await;
            return Err(DownloadError::TooLarge(max_bytes));
        }

        hasher.update(&chunk);
        file.write_all(&chunk).await?;
        on_progress(downloaded, total_bytes);
    }

    file.flush().await?;
    drop(file);

    let actual_sha256 = format!("{:x}", hasher.finalize());
    if !actual_sha256.eq_ignore_ascii_case(expected_sha256) {
        let _ = tokio::fs::remove_file(destination).await;
        return Err(DownloadError::ChecksumMismatch {
            expected: expected_sha256.to_owned(),
            actual: actual_sha256,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redirect_target_url_is_checked_for_scheme_and_exact_host() {
        assert!(
            validate_download_url(&reqwest::Url::parse("https://github.com/file").unwrap()).is_ok()
        );
        assert!(
            validate_download_url(&reqwest::Url::parse("https://www.gyan.dev/file").unwrap())
                .is_ok()
        );
        assert!(matches!(
            validate_download_url(
                &reqwest::Url::parse("https://github.com.evil.invalid/file").unwrap()
            ),
            Err(DownloadError::UntrustedHost(_))
        ));
        assert!(matches!(
            validate_download_url(&reqwest::Url::parse("http://github.com/file").unwrap()),
            Err(DownloadError::InsecureProtocol(_))
        ));
    }
}
