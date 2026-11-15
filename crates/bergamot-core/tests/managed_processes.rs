#![cfg(unix)]

use bergamot_core::descriptor::ExecutorConfig;
use bergamot_core::executors::process;
use bergamot_core::managed_tools::extractor::{extract_archive, ExtractionError};
use bergamot_core::managed_tools::manifest::{ArchiveFormat, VerificationConfig};
use bergamot_core::managed_tools::verifier::{verify_tool, VerificationError};
use bergamot_core::{AppSettings, CancellationToken, ExecutionError};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tempfile::tempdir;

fn write_executable(path: &Path, script: &[u8]) {
    use std::os::unix::fs::PermissionsExt;

    let mut file = fs::File::create(path).unwrap();
    file.write_all(script).unwrap();
    file.sync_all().unwrap();
    drop(file);
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
}

fn descendant_script(marker_name: &str) -> Vec<u8> {
    format!(
        "#!/bin/sh\n(while :; do printf . >> \"$(dirname \"$0\")/{marker_name}\"; sleep 0.05; done) &\nwhile [ ! -e \"$(dirname \"$0\")/{marker_name}\" ]; do sleep 0.01; done\nexit 0\n"
    )
    .into_bytes()
}

async fn wait_for_file(path: &Path) {
    tokio::time::timeout(Duration::from_secs(3), async {
        while !path.exists() {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("subprocess descendant did not start");
}

async fn assert_file_stops_growing(path: &Path) {
    let size = fs::metadata(path).unwrap().len();
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_eq!(
        fs::metadata(path).unwrap().len(),
        size,
        "descendant is alive"
    );
}

fn verification_config() -> VerificationConfig {
    VerificationConfig {
        command: vec![],
        encoders_command: None,
        required_encoders: vec![],
        formats_command: None,
        required_formats: vec![],
        test_script_args: None,
    }
}

#[tokio::test]
async fn verifier_cancellation_kills_descendant_without_blocking_runtime() {
    let temp = tempdir().unwrap();
    let binary = temp.path().join("verifier");
    let marker = temp.path().join("verifier-heartbeat");
    write_executable(&binary, &descendant_script("verifier-heartbeat"));

    let cancel = CancellationToken::new();
    let verifier_cancel = cancel.clone();
    let config = verification_config();
    let verifier =
        tokio::spawn(
            async move { verify_tool("fixture", &binary, &config, &verifier_cancel).await },
        );

    let ticks = Arc::new(AtomicUsize::new(0));
    let ticker_ticks = Arc::clone(&ticks);
    let ticker = tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(10)).await;
            ticker_ticks.fetch_add(1, Ordering::Relaxed);
        }
    });

    wait_for_file(&marker).await;
    tokio::time::timeout(Duration::from_secs(1), async {
        while ticks.load(Ordering::Relaxed) < 2 {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("runtime did not make progress");
    assert!(
        !verifier.is_finished(),
        "descendant should keep verifier pending"
    );
    cancel.cancel();
    let error = tokio::time::timeout(Duration::from_secs(2), verifier)
        .await
        .expect("verifier cancellation did not return")
        .unwrap()
        .unwrap_err();
    ticker.abort();
    assert!(matches!(error, VerificationError::Cancelled));
    assert_file_stops_growing(&marker).await;
}

#[tokio::test]
async fn python_dependency_probe_uses_the_caller_cancellation_token() {
    let temp = tempdir().unwrap();
    let custom_path = temp.path().join("custom-bin");
    let empty_path = temp.path().join("empty-bin");
    fs::create_dir(&custom_path).unwrap();
    fs::create_dir(&empty_path).unwrap();
    let python = custom_path.join("python3");
    let marker = custom_path.join("python-heartbeat");
    write_executable(&python, &descendant_script("python-heartbeat"));

    let executor = ExecutorConfig::Process {
        program: "python3".into(),
        args: vec![],
        working_directory: None,
        env: [("PATH".into(), empty_path.to_string_lossy().into_owned())].into(),
        timeout_seconds: 5,
    };
    let settings = AppSettings {
        custom_paths: vec![custom_path.to_string_lossy().into_owned()],
        managed_tools_dir: None,
        ..AppSettings::default()
    };
    let cancel = CancellationToken::new();
    let resolver_cancel = cancel.clone();
    let resolver = tokio::spawn(async move {
        process::resolve_dependency_detailed(&executor, &settings, &resolver_cancel).await
    });

    wait_for_file(&marker).await;
    cancel.cancel();
    let error = tokio::time::timeout(Duration::from_secs(2), resolver)
        .await
        .expect("dependency resolution cancellation did not return")
        .unwrap()
        .unwrap_err();
    assert!(matches!(error, ExecutionError::Cancelled));
    assert_file_stops_growing(&marker).await;
}

#[tokio::test]
async fn appimage_extraction_uses_the_cancelable_process_group_runner() {
    let temp = tempdir().unwrap();
    let appimage = temp.path().join("fixture.AppImage");
    let marker = temp.path().join("appimage-heartbeat");
    let destination = temp.path().join("extract");
    write_executable(&appimage, &descendant_script("appimage-heartbeat"));

    let cancel = CancellationToken::new();
    let extractor_cancel = cancel.clone();
    let extractor_archive = PathBuf::from(&appimage);
    let extractor_destination = PathBuf::from(&destination);
    let extractor = tokio::spawn(async move {
        extract_archive(
            &extractor_archive,
            ArchiveFormat::AppImage,
            &extractor_destination,
            10,
            1_000,
            &extractor_cancel,
        )
        .await
    });

    wait_for_file(&marker).await;
    cancel.cancel();
    let error = tokio::time::timeout(Duration::from_secs(2), extractor)
        .await
        .expect("AppImage extraction cancellation did not return")
        .unwrap()
        .unwrap_err();
    assert!(matches!(error, ExtractionError::Cancelled));
    assert_file_stops_growing(&marker).await;
}
