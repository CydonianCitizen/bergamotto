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
    ALLOWED_HOSTS.contains(&host)
}

fn validate_download_url(url: &reqwest::Url) -> Result<(), DownloadError> {
    let host = url
        .host_str()
        .ok_or_else(|| DownloadError::InvalidUrl(url.to_string()))?;
    if url.scheme() != "https" {
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

    let response =
        request_with_redirects(&client, initial_url, cancel, validate_download_url).await?;

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

async fn request_with_redirects<F>(
    client: &reqwest::Client,
    initial_url: &str,
    cancel: &CancellationToken,
    validate_url: F,
) -> Result<reqwest::Response, DownloadError>
where
    F: Fn(&reqwest::Url) -> Result<(), DownloadError>,
{
    let mut current_url =
        reqwest::Url::parse(initial_url).map_err(|e| DownloadError::InvalidUrl(e.to_string()))?;
    let mut redirects = 0;

    loop {
        if cancel.is_cancelled() {
            return Err(DownloadError::Cancelled);
        }

        // Apply the same policy before every request, including every redirect target.
        validate_url(&current_url)?;

        let request = client.get(current_url.clone()).send();
        let response = tokio::select! {
            _ = cancel.cancelled() => return Err(DownloadError::Cancelled),
            result = request => result?,
        };

        if response.status().is_redirection() {
            redirects += 1;
            if redirects > 10 {
                return Err(DownloadError::TooManyRedirects);
            }
            let location = response
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

        if !response.status().is_success() {
            return Err(DownloadError::HttpStatus(response.status()));
        }

        return Ok(response);
    }
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
        assert!(matches!(
            validate_download_url(&reqwest::Url::parse("https://localhost/file").unwrap()),
            Err(DownloadError::UntrustedHost(host)) if host == "localhost"
        ));
        assert!(matches!(
            validate_download_url(&reqwest::Url::parse("https://127.0.0.1/file").unwrap()),
            Err(DownloadError::UntrustedHost(host)) if host == "127.0.0.1"
        ));
    }

    #[tokio::test]
    async fn redirect_targets_use_production_policy() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        for (target, expected) in [
            ("https://localhost/target", "localhost"),
            ("https://127.0.0.1/target", "127.0.0.1"),
            ("http://github.com/target", "http"),
            ("https://evil.invalid/target", "evil.invalid"),
        ] {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let address = listener.local_addr().unwrap();
            let server = tokio::spawn(async move {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut request = [0; 1024];
                let _ = socket.read(&mut request).await.unwrap();
                socket
                    .write_all(
                        format!(
                            "HTTP/1.1 302 Found\r\nLocation: {target}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                        )
                        .as_bytes(),
                    )
                    .await
                    .unwrap();
            });

            let client = reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .unwrap();
            let initial = format!("http://127.0.0.1:{}/start", address.port());
            let cancel = CancellationToken::new();
            let allow_test_loopback_initial = |url: &reqwest::Url| {
                if url.as_str() == initial {
                    Ok(())
                } else {
                    validate_download_url(url)
                }
            };
            let result =
                request_with_redirects(&client, &initial, &cancel, allow_test_loopback_initial)
                    .await;

            match expected {
                "localhost" | "127.0.0.1" | "evil.invalid" => assert!(matches!(
                    result,
                    Err(DownloadError::UntrustedHost(host)) if host == expected
                )),
                "http" => assert!(matches!(
                    result,
                    Err(DownloadError::InsecureProtocol(scheme)) if scheme == expected
                )),
                _ => unreachable!(),
            }
            server.await.unwrap();
        }
    }
}
