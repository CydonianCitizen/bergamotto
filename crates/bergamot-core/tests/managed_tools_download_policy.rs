use bergamot_core::managed_tools::{download_file, DownloadError};
use std::path::Path;
use tempfile::tempdir;
use tokio_util::sync::CancellationToken;

async fn assert_rejected_without_request(url: &str, expected: fn(&DownloadError) -> bool) {
    let temp = tempdir().unwrap();
    let result = download_file(
        url,
        &"0".repeat(64),
        &temp.path().join("download"),
        100,
        |_, _| {},
        &CancellationToken::new(),
    )
    .await;

    let error = result.expect_err("unsafe URL must be rejected before network access");
    assert!(expected(&error), "unexpected error for {url}: {error}");
    assert!(!Path::new(&temp.path().join("download")).exists());
}

#[tokio::test]
async fn normal_build_rejects_local_hosts_and_http_before_requesting() {
    assert_rejected_without_request(
        "https://localhost/file",
        |error| matches!(error, DownloadError::UntrustedHost(host) if host == "localhost"),
    )
    .await;
    assert_rejected_without_request(
        "https://127.0.0.1/file",
        |error| matches!(error, DownloadError::UntrustedHost(host) if host == "127.0.0.1"),
    )
    .await;
    assert_rejected_without_request(
        "https://[::1]/file",
        |error| matches!(error, DownloadError::UntrustedHost(host) if host == "[::1]"),
    )
    .await;
    assert_rejected_without_request(
        "http://github.com/file",
        |error| matches!(error, DownloadError::InsecureProtocol(scheme) if scheme == "http"),
    )
    .await;
    assert_rejected_without_request(
        "http://127.0.0.1/file",
        |error| matches!(error, DownloadError::InsecureProtocol(scheme) if scheme == "http"),
    )
    .await;
}
